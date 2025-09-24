//! Contract tests for the shared ACP runtime.
//!
//! These tests describe the observable behavior expected from the
//! `RuntimeServer` abstraction and ensure the runtime maintains the protocol
//! guarantees captured in the specification and plan.

use std::path::PathBuf;
use std::sync::Arc;

use acp_lazy_core::permissions::AcpPermissionMode;
use acp_lazy_core::runtime::{
    ProviderAdapter, RuntimeConfig, RuntimeServer, SessionNotifier, SessionState,
};
use agent_client_protocol::{
    AgentCapabilities, AuthMethodId, AuthenticateRequest, CancelNotification, ClientCapabilities,
    ContentBlock, Error, ErrorCode, ExtNotification, ExtRequest, InitializeRequest,
    LoadSessionRequest, McpServer, NewSessionRequest, PromptRequest, PromptResponse, RawValue,
    SessionId, SessionModeId, SetSessionModeRequest, StopReason, VERSION,
};
use anyhow::{bail, Context, Result};
use async_trait::async_trait;

fn runtime() -> RuntimeServer {
    let adapter: std::sync::Arc<dyn ProviderAdapter> = std::sync::Arc::new(TestAdapter);
    RuntimeServer::with_defaults(adapter, None)
}

#[derive(Default)]
struct TestAdapter;

#[async_trait(?Send)]
impl ProviderAdapter for TestAdapter {
    fn agent_capabilities(&self) -> AgentCapabilities {
        AgentCapabilities::default()
    }

    async fn handle_prompt(
        &self,
        _session: SessionState,
        _request: PromptRequest,
        _notifier: SessionNotifier,
        _config: &RuntimeConfig,
    ) -> Result<PromptResponse, Error> {
        Ok(PromptResponse {
            stop_reason: StopReason::EndTurn,
            meta: None,
        })
    }
}

fn new_session_request(cwd: PathBuf) -> NewSessionRequest {
    NewSessionRequest {
        cwd,
        mcp_servers: Vec::<McpServer>::new(),
        meta: None,
    }
}

fn session_id(value: &str) -> SessionId {
    SessionId(Arc::from(value))
}

fn raw_json(value: serde_json::Value) -> Result<Arc<RawValue>> {
    RawValue::from_string(value.to_string())
        .map(Arc::from)
        .context("valid raw json")
}

fn assert_error_code(err: Error, expected: ErrorCode) {
    assert_eq!(
        err.code, expected.code,
        "unexpected error code: {}",
        err.message
    );
}

#[tokio::test]
async fn initialize_returns_v1_capabilities() -> Result<()> {
    let runtime = runtime();
    let request = InitializeRequest {
        protocol_version: VERSION,
        client_capabilities: ClientCapabilities::default(),
        meta: None,
    };

    let response = runtime.initialize(request).await?;

    assert_eq!(response.protocol_version, VERSION);
    assert!(!response.agent_capabilities.load_session);
    assert!(!response.agent_capabilities.prompt_capabilities.audio);
    assert!(
        !response
            .agent_capabilities
            .prompt_capabilities
            .embedded_context
    );
    assert!(response.auth_methods.is_empty());
    Ok(())
}

#[tokio::test]
async fn new_session_creates_entry_with_default_permission() -> Result<()> {
    let runtime = runtime();
    let cwd = std::env::current_dir()?;

    let response = runtime
        .new_session(new_session_request(cwd.clone()))
        .await?;

    assert_eq!(runtime.session_count().await, 1);

    let stored: SessionState = runtime
        .session_state(&response.session_id)
        .await
        .context("session should exist")?;
    assert_eq!(stored.working_dir, cwd);
    assert_eq!(stored.permission_mode, AcpPermissionMode::Default);
    Ok(())
}

#[tokio::test]
async fn prompt_without_session_returns_invalid_params() -> Result<()> {
    let runtime = runtime();
    let request = PromptRequest {
        session_id: session_id("missing"),
        prompt: vec![ContentBlock::from("hello world")],
        meta: None,
    };

    let error = match runtime.prompt(request).await {
        Ok(_) => bail!("prompt should fail"),
        Err(err) => err,
    };
    assert_error_code(error, ErrorCode::INVALID_PARAMS);
    Ok(())
}

#[tokio::test]
async fn authenticate_is_not_supported() -> Result<()> {
    let runtime = runtime();
    let request = AuthenticateRequest {
        method_id: AuthMethodId(Arc::from("noop")),
        meta: None,
    };

    let error = match runtime.authenticate(request).await {
        Ok(_) => bail!("authenticate should be unsupported"),
        Err(err) => err,
    };
    assert_error_code(error, ErrorCode::METHOD_NOT_FOUND);
    Ok(())
}

#[tokio::test]
async fn load_session_is_not_supported() -> Result<()> {
    let runtime = runtime();
    let request = LoadSessionRequest {
        mcp_servers: Vec::new(),
        cwd: std::env::current_dir()?,
        session_id: session_id("unknown"),
        meta: None,
    };

    let error = match runtime.load_session(request).await {
        Ok(_) => bail!("load_session should be unsupported"),
        Err(err) => err,
    };
    assert_error_code(error, ErrorCode::METHOD_NOT_FOUND);
    Ok(())
}

#[tokio::test]
async fn set_session_mode_updates_permission_mode() -> Result<()> {
    let runtime = runtime();
    let cwd = std::env::current_dir()?;
    let created = runtime.new_session(new_session_request(cwd)).await?;

    let request = SetSessionModeRequest {
        session_id: created.session_id.clone(),
        mode_id: SessionModeId(Arc::from("bypass-permissions")),
        meta: None,
    };

    runtime.set_session_mode(request).await?;

    let stored = runtime
        .session_state(&created.session_id)
        .await
        .context("session should exist")?;
    assert_eq!(stored.permission_mode, AcpPermissionMode::BypassPermissions);
    Ok(())
}

#[tokio::test]
async fn cancel_without_active_prompt_is_noop() -> Result<()> {
    let runtime = runtime();
    runtime
        .cancel(CancelNotification {
            session_id: session_id("missing"),
            meta: None,
        })
        .await?;
    Ok(())
}

#[tokio::test]
async fn ext_method_returns_method_not_found() -> Result<()> {
    let runtime = runtime();
    let error = match runtime
        .ext_method(ExtRequest {
            method: Arc::from("custom.feature"),
            params: raw_json(serde_json::json!({}))?,
        })
        .await
    {
        Ok(_) => bail!("ext_method should not be implemented"),
        Err(err) => err,
    };
    assert_error_code(error, ErrorCode::METHOD_NOT_FOUND);
    Ok(())
}

#[tokio::test]
async fn ext_notification_is_gracefully_ignored() -> Result<()> {
    let runtime = runtime();
    runtime
        .ext_notification(ExtNotification {
            method: Arc::from("custom.notification"),
            params: raw_json(serde_json::json!({}))?,
        })
        .await?;
    Ok(())
}
