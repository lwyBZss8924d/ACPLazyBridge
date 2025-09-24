use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use acp_lazy_core::permissions::map_acp_to_codex;
use acp_lazy_core::runtime::{
    ProviderAdapter, RuntimeConfig, RuntimeServer, SessionNotifier, SessionState,
};
use acp_lazy_core::transport::{write_line, ProcessTransport};
use agent_client_protocol::{
    Agent, AgentCapabilities, AuthenticateRequest, AuthenticateResponse, CancelNotification,
    ContentBlock, Error, ExtNotification, ExtRequest, ExtResponse, InitializeRequest,
    InitializeResponse, LoadSessionRequest, LoadSessionResponse, NewSessionRequest,
    NewSessionResponse, PromptRequest, PromptResponse, SessionId, SessionNotification,
    SessionUpdate, StopReason, ToolCall, ToolCallContent, ToolCallId, ToolCallStatus,
    ToolCallUpdate, ToolCallUpdateFields,
};
use anyhow::Error as AnyhowError;
use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::sync::{mpsc, Mutex, Notify, RwLock};
use tokio::task::JoinSet;
use tokio::time::{self, Duration, Instant};
use tracing::{debug, warn};

use crate::codex_proto::{self, SessionUpdate as CodexSessionUpdate, SessionUpdateContent};
use crate::notify_source::{create_notify_source, NotifyEvent};

#[derive(Default)]
struct CodexProviderAdapter {
    processes: Arc<RwLock<HashMap<String, Arc<ProcessEntry>>>>,
}

struct ProcessEntry {
    transport: Mutex<Option<ProcessTransport>>,
    cancelled: AtomicBool,
    cancel_notify: Notify,
    notify_source: Mutex<Option<Box<dyn crate::notify_source::NotifySource + Send>>>,
}

impl ProcessEntry {
    fn new() -> Self {
        Self {
            transport: Mutex::new(None),
            cancelled: AtomicBool::new(false),
            cancel_notify: Notify::new(),
            notify_source: Mutex::new(None),
        }
    }

    async fn store_transport(&self, transport: ProcessTransport) {
        let mut guard = self.transport.lock().await;
        *guard = Some(transport);
    }

    async fn take_transport(&self) -> Option<ProcessTransport> {
        self.transport.lock().await.take()
    }

    async fn store_notify_source(
        &self,
        source: Box<dyn crate::notify_source::NotifySource + Send>,
    ) {
        let mut guard = self.notify_source.lock().await;
        *guard = Some(source);
    }

    async fn take_notify_source(
        &self,
    ) -> Option<Box<dyn crate::notify_source::NotifySource + Send>> {
        self.notify_source.lock().await.take()
    }

    fn mark_cancelled(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
        self.cancel_notify.notify_waiters();
    }

    fn cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
}

impl CodexProviderAdapter {
    fn agent_capabilities_internal(&self) -> AgentCapabilities {
        AgentCapabilities {
            load_session: false,
            prompt_capabilities: acp_prompt_caps(),
            mcp_capabilities: Default::default(),
            meta: None,
        }
    }

    async fn prepare_entry(&self, session_key: &str) -> Arc<ProcessEntry> {
        if let Some(entry) = self.remove_entry(session_key).await {
            self.shutdown_entry(&entry).await;
        }

        let entry = Arc::new(ProcessEntry::new());
        self.processes
            .write()
            .await
            .insert(session_key.to_string(), entry.clone());
        entry
    }

    async fn remove_entry(&self, session_key: &str) -> Option<Arc<ProcessEntry>> {
        self.processes.write().await.remove(session_key)
    }

    async fn shutdown_entry(&self, entry: &Arc<ProcessEntry>) {
        if let Some(mut process) = entry.take_transport().await {
            if process.is_running() {
                if let Err(e) = process.kill().await {
                    warn!("Failed to kill Codex process: {}", e);
                }
            }
            if let Err(e) = process.wait().await {
                warn!("Failed to wait for Codex process exit: {}", e);
            }
        }

        if let Some(mut source) = entry.take_notify_source().await {
            if let Err(e) = source.stop().await {
                warn!("Failed to stop notify source: {}", e);
            }
        }
    }

    async fn finish_prompt(
        &self,
        session_key: &str,
        entry: Arc<ProcessEntry>,
        join_set: &mut JoinSet<Result<(), Error>>,
    ) {
        self.shutdown_entry(&entry).await;

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(())) => {}
                Ok(Err(err)) => warn!("Background task error: {:?}", err),
                Err(join_err) => warn!("Background task join error: {}", join_err),
            }
        }

        self.processes.write().await.remove(session_key);
    }
}

fn acp_prompt_caps() -> agent_client_protocol::PromptCapabilities {
    agent_client_protocol::PromptCapabilities::default()
}

fn anyhow_to_acp(err: AnyhowError) -> Error {
    Error::internal_error().with_data(err.to_string())
}

#[async_trait(?Send)]
impl ProviderAdapter for CodexProviderAdapter {
    fn agent_capabilities(&self) -> AgentCapabilities {
        self.agent_capabilities_internal()
    }

    async fn handle_prompt(
        &self,
        session: SessionState,
        request: PromptRequest,
        notifier: SessionNotifier,
        config: &RuntimeConfig,
    ) -> Result<PromptResponse, Error> {
        let session_id_str = session.session_id.0.to_string();

        match self
            .spawn_and_stream_codex(
                &session,
                &request,
                notifier.clone(),
                config,
                session_id_str.clone(),
            )
            .await
        {
            Ok(stop_reason) => Ok(PromptResponse {
                stop_reason,
                meta: None,
            }),
            Err(spawn_err) => {
                warn!(
                    "Codex process failed to start for session {}: {}",
                    session_id_str, spawn_err.message
                );
                Err(spawn_err)
            }
        }
    }

    async fn handle_cancel(&self, notification: CancelNotification) -> Result<(), Error> {
        let session_key = notification.session_id.0.to_string();
        let entry = {
            let map = self.processes.read().await;
            map.get(&session_key).cloned()
        };

        if let Some(entry) = entry {
            entry.mark_cancelled();
            if let Some(mut process) = entry.take_transport().await {
                if let Err(e) = process.kill().await {
                    return Err(Error::internal_error().with_data(e.to_string()));
                }
            }
            if let Some(mut source) = entry.take_notify_source().await {
                if let Err(e) = source.stop().await {
                    warn!("Failed to stop notify source during cancel: {}", e);
                }
            }
        }

        Ok(())
    }

    async fn on_permission_mode_changed(
        &self,
        session_id: &SessionId,
        _session: &SessionState,
    ) -> Result<(), Error> {
        if let Some(entry) = self.remove_entry(&session_id.0).await {
            self.shutdown_entry(&entry).await;
        }
        Ok(())
    }
}

impl CodexProviderAdapter {
    async fn spawn_and_stream_codex(
        &self,
        session: &SessionState,
        request: &PromptRequest,
        notifier: SessionNotifier,
        config: &RuntimeConfig,
        session_key: String,
    ) -> Result<StopReason, Error> {
        let entry = self.prepare_entry(&session_key).await;

        // Prepare CLI args based on permission mode.
        let overrides = map_acp_to_codex(session.permission_mode);
        let mut args = vec!["proto".to_string()];
        args.extend(overrides.to_cli_args());

        // Notify integration (mirrors legacy behavior).
        let notify_path = std::env::var("ACPLB_NOTIFY_PATH").ok();
        let notify_inject = std::env::var("ACPLB_NOTIFY_INJECT").unwrap_or_else(|_| "auto".into());
        let notify_cmd = std::env::var("ACPLB_NOTIFY_CMD").ok();

        if notify_path.is_some() {
            let should_inject = match notify_inject.as_str() {
                "never" => false,
                "force" => true,
                _ => notify_cmd.is_none(),
            };
            if should_inject {
                if let Ok(forwarder) = resolve_forwarder_path() {
                    args.push("-c".into());
                    args.push(format!("notify=[\"{}\"]", forwarder));
                }
            } else if let Some(cmd) = notify_cmd.clone() {
                args.push("-c".into());
                args.push(format!("notify={}", cmd));
            }
        }

        let codex_cmd = std::env::var("CODEX_CMD").unwrap_or_else(|_| "codex".into());

        let mut process =
            match ProcessTransport::spawn(&codex_cmd, &args, None, session.working_dir.to_str())
                .await
            {
                Ok(proc) => proc,
                Err(err) => {
                    self.processes.write().await.remove(&session_key);
                    return Err(Error::internal_error().with_data(err.to_string()));
                }
            };

        if let Err(e) = process.monitor_stderr() {
            warn!("Failed to monitor Codex stderr: {}", e);
        }

        let messages = build_codex_messages(request);
        let codex_request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "prompt",
            "params": {
                "messages": messages
            }
        });

        if let Err(e) = write_line(process.stdin(), &codex_request.to_string()).await {
            self.processes.write().await.remove(&session_key);
            return Err(Error::internal_error().with_data(e.to_string()));
        }

        let stdout = match process.take_stdout() {
            Some(stdout) => stdout,
            None => {
                self.processes.write().await.remove(&session_key);
                return Err(Error::internal_error().with_data("missing stdout"));
            }
        };

        entry.store_transport(process).await;

        let (update_tx, mut update_rx) = mpsc::unbounded_channel::<CodexSessionUpdate>();
        let mut join_set: JoinSet<Result<(), Error>> = JoinSet::new();
        let stream_session = session_key.clone();
        join_set.spawn(async move {
            codex_proto::stream_codex_output(stdout, stream_session, update_tx)
                .await
                .map_err(anyhow_to_acp)
        });

        let (notify_tx, mut notify_rx) = mpsc::unbounded_channel::<NotifyEvent>();
        let mut notify_enabled = false;
        if let Some(path) = notify_path.clone() {
            let notify_kind = std::env::var("ACPLB_NOTIFY_KIND").ok();
            let mut source =
                create_notify_source(&path, notify_kind.as_deref(), config.polling_interval_ms);
            if let Err(e) = source.start_monitoring(notify_tx.clone()).await {
                warn!("Notify monitoring failed for {}: {}", session_key, e);
            } else {
                entry.store_notify_source(source).await;
                notify_enabled = true;
            }
        }
        drop(notify_tx);

        let notifier = notifier.clone();
        let idle_interval = Duration::from_millis(config.polling_interval_ms.max(1));
        let idle_timeout = Duration::from_millis(config.idle_timeout_ms.max(1));
        let mut last_activity = Instant::now();
        let mut stream_open = true;
        let mut stop_reason = StopReason::EndTurn;

        let idle_timer = time::sleep(idle_interval);
        tokio::pin!(idle_timer);

        loop {
            tokio::select! {
                _ = entry.cancel_notify.notified() => {
                    stop_reason = StopReason::Cancelled;
                    break;
                }
                update = update_rx.recv(), if stream_open => {
                    match update {
                        Some(update) => {
                            if let Some(session_notification) = convert_session_update(update) {
                                if let Some(tx) = notifier.as_ref() {
                                    let _ = tx.send(session_notification);
                                }
                            }
                            last_activity = Instant::now();
                        }
                        None => {
                            stream_open = false;
                        }
                    }
                }
                notify = notify_rx.recv(), if notify_enabled => {
                    match notify {
                        Some(event) => {
                            if event.event_type == "agent-turn-complete" {
                                debug!("Session {} received agent-turn-complete", session_key);
                                stop_reason = StopReason::EndTurn;
                                break;
                            }
                        }
                        None => {
                            notify_enabled = false;
                        }
                    }
                }
                _ = &mut idle_timer => {
                    if entry.cancelled() {
                        stop_reason = StopReason::Cancelled;
                        break;
                    }

                    let now = Instant::now();
                    if now.duration_since(last_activity) >= idle_timeout {
                        debug!("Session {} idle timeout reached", session_key);
                        break;
                    }

                    idle_timer.as_mut().reset(now + idle_interval);
                }
            }

            if !stream_open && !notify_enabled {
                break;
            }
        }

        self.finish_prompt(&session_key, entry.clone(), &mut join_set)
            .await;

        if entry.cancelled() {
            Ok(StopReason::Cancelled)
        } else {
            Ok(stop_reason)
        }
    }
}

fn resolve_forwarder_path() -> Result<String, Error> {
    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(parent) = current_exe.parent() {
            let forwarder = parent.join("acplb-notify-forwarder");
            if forwarder.exists() {
                return Ok(forwarder.to_string_lossy().to_string());
            }
        }
    }
    let target_paths = [
        "target/debug/acplb-notify-forwarder",
        "target/release/acplb-notify-forwarder",
        "../target/debug/acplb-notify-forwarder",
        "../target/release/acplb-notify-forwarder",
    ];
    for candidate in target_paths {
        let path = PathBuf::from(candidate);
        if path.exists() {
            return Ok(path
                .canonicalize()
                .unwrap_or(path)
                .to_string_lossy()
                .to_string());
        }
    }
    Ok("acplb-notify-forwarder".into())
}

fn build_codex_messages(request: &PromptRequest) -> Vec<Value> {
    let mut blocks: Vec<Value> = Vec::new();
    for block in &request.prompt {
        if let ContentBlock::Text(text) = block {
            blocks.push(json!({
                "type": "text",
                "text": text.text
            }));
        }
    }
    vec![json!({
        "role": "user",
        "content": blocks
    })]
}

fn convert_session_update(update: CodexSessionUpdate) -> Option<SessionNotification> {
    let session_id = SessionId(Arc::from(update.params.session_id));
    let acp_update = match update.params.update {
        SessionUpdateContent::AgentMessageChunk { content } => {
            let text = convert_content_block(content)?;
            SessionUpdate::AgentMessageChunk { content: text }
        }
        SessionUpdateContent::ToolCall { .. } | SessionUpdateContent::ToolCallUpdate { .. } => {
            convert_tool_update(update.params.update).ok()?
        }
    };
    Some(SessionNotification {
        session_id,
        update: acp_update,
        meta: None,
    })
}

fn convert_content_block(block: codex_proto::ContentBlock) -> Option<ContentBlock> {
    match block {
        codex_proto::ContentBlock::Text { text } => {
            Some(ContentBlock::Text(agent_client_protocol::TextContent {
                text,
                annotations: None,
                meta: None,
            }))
        }
    }
}

fn convert_tool_update(content: SessionUpdateContent) -> Result<SessionUpdate, Error> {
    match content {
        SessionUpdateContent::ToolCall {
            tool_call_id,
            title,
            kind,
            status,
            content,
            raw_input,
            raw_output,
            ..
        } => {
            let tool_call = ToolCall {
                id: ToolCallId(Arc::from(tool_call_id)),
                title,
                kind: map_tool_kind(kind),
                status: status.map_or(ToolCallStatus::Pending, map_tool_status),
                content: convert_tool_content(content),
                locations: Vec::new(),
                raw_input,
                raw_output,
                meta: None,
            };
            Ok(SessionUpdate::ToolCall(tool_call))
        }
        SessionUpdateContent::ToolCallUpdate {
            tool_call_id,
            status,
            title,
            kind,
            content,
            raw_output,
            ..
        } => {
            let fields = ToolCallUpdateFields {
                kind: kind.map(|value| map_tool_kind(Some(value))),
                status: status.map(map_tool_status),
                title,
                content: content.map(|value| convert_tool_content(Some(value))),
                locations: None,
                raw_input: None,
                raw_output,
            };
            Ok(SessionUpdate::ToolCallUpdate(ToolCallUpdate {
                id: ToolCallId(Arc::from(tool_call_id)),
                fields,
                meta: None,
            }))
        }
        _ => Err(Error::internal_error()),
    }
}

fn convert_tool_content(content: Option<Vec<codex_proto::ContentBlock>>) -> Vec<ToolCallContent> {
    content
        .unwrap_or_default()
        .into_iter()
        .filter_map(|block| convert_content_block(block).map(ToolCallContent::from))
        .collect()
}

fn map_tool_kind(kind: Option<String>) -> agent_client_protocol::ToolKind {
    match kind.as_deref() {
        Some("read") => agent_client_protocol::ToolKind::Read,
        Some("edit") => agent_client_protocol::ToolKind::Edit,
        Some("delete") => agent_client_protocol::ToolKind::Delete,
        Some("move") => agent_client_protocol::ToolKind::Move,
        Some("search") => agent_client_protocol::ToolKind::Search,
        Some("execute") => agent_client_protocol::ToolKind::Execute,
        Some("think") => agent_client_protocol::ToolKind::Think,
        Some("fetch") => agent_client_protocol::ToolKind::Fetch,
        Some("switch_mode") => agent_client_protocol::ToolKind::SwitchMode,
        _ => agent_client_protocol::ToolKind::Other,
    }
}

fn map_tool_status(status: codex_proto::ToolCallStatus) -> ToolCallStatus {
    match status {
        codex_proto::ToolCallStatus::Pending => ToolCallStatus::Pending,
        codex_proto::ToolCallStatus::InProgress => ToolCallStatus::InProgress,
        codex_proto::ToolCallStatus::Completed => ToolCallStatus::Completed,
        codex_proto::ToolCallStatus::Failed => ToolCallStatus::Failed,
    }
}

/// Shared runtime agent used by the Codex adapter.
#[derive(Clone)]
pub struct CodexAgent {
    runtime: RuntimeServer,
}

impl CodexAgent {
    /// Construct a new Codex agent instance.
    pub fn new() -> Self {
        Self::new_with_notifier(None)
    }

    /// Construct a Codex agent instance configured for tests.
    pub fn for_testing() -> Self {
        Self::new()
    }

    /// Construct with a specific runtime configuration (primarily for tests).
    pub fn with_config(config: RuntimeConfig, notifier: SessionNotifier) -> Self {
        let adapter: Arc<dyn ProviderAdapter> = Arc::new(CodexProviderAdapter::default());
        let runtime = RuntimeServer::new(adapter, config, notifier);
        Self { runtime }
    }

    pub fn new_with_notifier(notifier: SessionNotifier) -> Self {
        let adapter: Arc<dyn ProviderAdapter> = Arc::new(CodexProviderAdapter::default());
        let runtime = RuntimeServer::with_defaults(adapter, notifier);
        Self { runtime }
    }

    pub fn runtime(&self) -> &RuntimeServer {
        &self.runtime
    }
}

impl Default for CodexAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl Agent for CodexAgent {
    async fn initialize(&self, args: InitializeRequest) -> Result<InitializeResponse, Error> {
        self.runtime.initialize(args).await
    }

    async fn authenticate(&self, args: AuthenticateRequest) -> Result<AuthenticateResponse, Error> {
        self.runtime.authenticate(args).await
    }

    async fn new_session(&self, args: NewSessionRequest) -> Result<NewSessionResponse, Error> {
        self.runtime.new_session(args).await
    }

    async fn load_session(&self, args: LoadSessionRequest) -> Result<LoadSessionResponse, Error> {
        self.runtime.load_session(args).await
    }

    async fn prompt(&self, args: PromptRequest) -> Result<PromptResponse, Error> {
        self.runtime.prompt(args).await
    }

    async fn cancel(&self, notification: CancelNotification) -> Result<(), Error> {
        self.runtime.cancel(notification).await
    }

    async fn set_session_mode(
        &self,
        args: agent_client_protocol::SetSessionModeRequest,
    ) -> Result<agent_client_protocol::SetSessionModeResponse, Error> {
        self.runtime.set_session_mode(args).await
    }

    #[cfg(feature = "unstable")]
    async fn set_session_model(
        &self,
        args: agent_client_protocol::SetSessionModelRequest,
    ) -> Result<agent_client_protocol::SetSessionModelResponse, Error> {
        self.runtime.set_session_model(args).await
    }

    async fn ext_method(&self, args: ExtRequest) -> Result<ExtResponse, Error> {
        self.runtime.ext_method(args).await
    }

    async fn ext_notification(&self, notification: ExtNotification) -> Result<(), Error> {
        self.runtime.ext_notification(notification).await
    }
}
