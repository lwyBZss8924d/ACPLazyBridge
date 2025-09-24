//! Runtime server orchestration.
//!
//! `RuntimeServer` coordinates ACP request handling while delegating
//! provider-specific behavior (process transport, streaming) to a
//! `ProviderAdapter` implementation.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use agent_client_protocol::AgentCapabilities;
use agent_client_protocol::{
    AuthenticateRequest, AuthenticateResponse, CancelNotification, Error, ExtNotification,
    ExtRequest, ExtResponse, InitializeRequest, InitializeResponse, LoadSessionRequest,
    LoadSessionResponse, NewSessionRequest, NewSessionResponse, PromptRequest, PromptResponse,
    SessionId, SetSessionModeRequest, SetSessionModeResponse, VERSION,
};
#[cfg(feature = "unstable")]
use agent_client_protocol::{SetSessionModelRequest, SetSessionModelResponse};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::permissions::AcpPermissionMode;
use crate::runtime::adapter::{ProviderAdapter, SessionNotifier};
use crate::runtime::session::{SessionState, SessionStore};

/// Configuration options for the runtime server.
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Idle timeout in milliseconds.
    pub idle_timeout_ms: u64,
    /// Polling interval in milliseconds.
    pub polling_interval_ms: u64,
    /// Optional evidence file path for runtime events.
    pub evidence_path: Option<PathBuf>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            idle_timeout_ms: std::env::var("ACPLB_IDLE_TIMEOUT_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1_200),
            polling_interval_ms: std::env::var("ACPLB_POLLING_INTERVAL_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            evidence_path: std::env::var("ACPLB_EVIDENCE_PATH").ok().map(PathBuf::from),
        }
    }
}

/// Core runtime entry point used by adapter crates.
#[derive(Clone)]
pub struct RuntimeServer {
    session_store: SessionStore,
    config: RuntimeConfig,
    provider: Arc<dyn ProviderAdapter>,
    notifier: SessionNotifier,
}

impl RuntimeServer {
    /// Create a new runtime server using the provided configuration.
    pub fn new(
        provider: Arc<dyn ProviderAdapter>,
        config: RuntimeConfig,
        notifier: SessionNotifier,
    ) -> Self {
        Self {
            session_store: SessionStore::default(),
            config,
            provider,
            notifier,
        }
    }

    /// Convenience constructor using default configuration values.
    pub fn with_defaults(provider: Arc<dyn ProviderAdapter>, notifier: SessionNotifier) -> Self {
        Self::new(provider, RuntimeConfig::default(), notifier)
    }

    /// Number of tracked sessions. Intended for testing and diagnostics.
    pub async fn session_count(&self) -> usize {
        self.session_store.len().await
    }

    /// Retrieve a copy of the stored session state.
    pub async fn session_state(&self, session_id: &SessionId) -> Option<SessionState> {
        self.session_store.get(session_id).await
    }

    pub async fn initialize(&self, _req: InitializeRequest) -> Result<InitializeResponse, Error> {
        info!(target: "acp_lazy_core::runtime", "initialize request received");

        let response = InitializeResponse {
            protocol_version: VERSION,
            agent_capabilities: self.merge_capabilities(),
            auth_methods: Vec::new(),
            meta: None,
        };

        self.record_event(
            "initialize",
            None,
            serde_json::json!({
                "protocolVersion": response.protocol_version,
            }),
        )
        .await;

        Ok(response)
    }

    pub async fn authenticate(
        &self,
        _req: AuthenticateRequest,
    ) -> Result<AuthenticateResponse, Error> {
        Err(Error::method_not_found())
    }

    pub async fn new_session(&self, req: NewSessionRequest) -> Result<NewSessionResponse, Error> {
        ensure_absolute(req.cwd.as_path())?;

        let session_id = SessionId(Arc::from(format!("session-{}", Uuid::new_v4())));
        let state = SessionState::new(
            session_id.clone(),
            req.cwd.clone(),
            AcpPermissionMode::Default,
        );
        info!(
            target: "acp_lazy_core::runtime",
            session_id = %state.session_id.0,
            cwd = %state.working_dir.display(),
            "creating session"
        );

        self.session_store.insert(state.clone()).await;
        self.provider.on_session_created(&state).await?;

        self.record_event(
            "session_created",
            Some(&state.session_id),
            serde_json::json!({
                "cwd": state.working_dir.display().to_string(),
            }),
        )
        .await;

        #[cfg(feature = "unstable")]
        let response = NewSessionResponse {
            session_id,
            modes: None,
            models: None,
            meta: None,
        };

        #[cfg(not(feature = "unstable"))]
        let response = NewSessionResponse {
            session_id,
            modes: None,
            meta: None,
        };

        Ok(response)
    }

    pub async fn load_session(
        &self,
        _req: LoadSessionRequest,
    ) -> Result<LoadSessionResponse, Error> {
        Err(Error::method_not_found())
    }

    pub async fn set_session_mode(
        &self,
        req: SetSessionModeRequest,
    ) -> Result<SetSessionModeResponse, Error> {
        let mode = req
            .mode_id
            .to_string()
            .parse::<AcpPermissionMode>()
            .map_err(|_| Error::invalid_params().with_data("unsupported session mode"))?;

        self.session_store
            .update_permission(&req.session_id, mode)
            .await
            .ok_or_else(Error::method_not_found)?;

        if let Some(updated) = self.session_store.get(&req.session_id).await {
            info!(
                target: "acp_lazy_core::runtime",
                session_id = %req.session_id.0,
                mode = %format!("{:?}", updated.permission_mode),
                "session mode updated"
            );

            self.provider
                .on_permission_mode_changed(&req.session_id, &updated)
                .await?;

            self.record_event(
                "session_mode_changed",
                Some(&req.session_id),
                serde_json::json!({
                    "mode": format!("{:?}", updated.permission_mode),
                }),
            )
            .await;
        }

        Ok(SetSessionModeResponse { meta: None })
    }

    pub async fn prompt(&self, req: PromptRequest) -> Result<PromptResponse, Error> {
        let session = self
            .session_store
            .get(&req.session_id)
            .await
            .ok_or_else(|| Error::invalid_params().with_data("unknown session id"))?;

        info!(
            target: "acp_lazy_core::runtime",
            session_id = %session.session_id.0,
            message_count = req.prompt.len(),
            "prompt started"
        );

        self.record_event(
            "prompt_started",
            Some(&session.session_id),
            serde_json::json!({
                "messageCount": req.prompt.len(),
            }),
        )
        .await;

        match self
            .provider
            .handle_prompt(session.clone(), req, self.notifier.clone(), &self.config)
            .await
        {
            Ok(response) => {
                info!(
                    target: "acp_lazy_core::runtime",
                    session_id = %session.session_id.0,
                    stop_reason = ?response.stop_reason,
                    "prompt completed"
                );

                self.record_event(
                    "prompt_completed",
                    Some(&session.session_id),
                    serde_json::json!({
                        "stopReason": response.stop_reason,
                    }),
                )
                .await;

                Ok(response)
            }
            Err(err) => {
                let message = err.message.clone();
                let code = err.code;

                warn!(
                    target: "acp_lazy_core::runtime",
                    session_id = %session.session_id.0,
                    code = code,
                    "prompt failed: {}",
                    message
                );

                self.record_event(
                    "prompt_failed",
                    Some(&session.session_id),
                    serde_json::json!({
                        "error": message,
                        "code": code,
                    }),
                )
                .await;

                Err(err)
            }
        }
    }

    pub async fn cancel(&self, notification: CancelNotification) -> Result<(), Error> {
        info!(
            target: "acp_lazy_core::runtime",
            session_id = %notification.session_id.0,
            "cancellation requested"
        );

        self.record_event(
            "prompt_cancelled",
            Some(&notification.session_id),
            serde_json::json!({}),
        )
        .await;

        self.provider.handle_cancel(notification).await
    }

    #[cfg(feature = "unstable")]
    pub async fn set_session_model(
        &self,
        _req: SetSessionModelRequest,
    ) -> Result<SetSessionModelResponse, Error> {
        Err(Error::method_not_found())
    }

    pub async fn ext_method(&self, _req: ExtRequest) -> Result<ExtResponse, Error> {
        Err(Error::method_not_found())
    }

    pub async fn ext_notification(&self, _notification: ExtNotification) -> Result<(), Error> {
        Ok(())
    }

    fn merge_capabilities(&self) -> AgentCapabilities {
        let provider_caps = self.provider.agent_capabilities();
        AgentCapabilities {
            load_session: provider_caps.load_session,
            prompt_capabilities: provider_caps.prompt_capabilities,
            mcp_capabilities: provider_caps.mcp_capabilities,
            meta: provider_caps.meta,
        }
    }

    async fn record_event(
        &self,
        event: &str,
        session_id: Option<&SessionId>,
        details: serde_json::Value,
    ) {
        if let Some(path) = &self.config.evidence_path {
            let payload = serde_json::json!({
                "timestampMs": current_timestamp_ms(),
                "event": event,
                "sessionId": session_id.map(|id| id.0.to_string()),
                "details": details,
            });

            match serde_json::to_string(&payload) {
                Ok(line) => {
                    if let Err(err) = append_line(path, line).await {
                        warn!(
                            target: "acp_lazy_core::runtime",
                            "failed to write runtime evidence ({}): {}",
                            event,
                            err
                        );
                    }
                }
                Err(err) => warn!(
                    target: "acp_lazy_core::runtime",
                    "failed to serialize runtime evidence ({}): {}",
                    event,
                    err
                ),
            }
        } else {
            debug!(
                target: "acp_lazy_core::runtime",
                "{} (session={:?})",
                event,
                session_id.map(|id| id.0.to_string())
            );
        }
    }
}

/// Validate that a working directory is absolute.
fn ensure_absolute(path: &Path) -> Result<(), Error> {
    if path.is_absolute() {
        Ok(())
    } else {
        Err(Error::invalid_params().with_data("cwd must be an absolute path"))
    }
}

fn current_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}

async fn append_line(path: &PathBuf, line: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .await?;
    file.write_all(line.as_bytes()).await?;
    file.write_all(b"\n").await
}
