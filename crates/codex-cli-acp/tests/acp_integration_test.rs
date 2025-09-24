//! Integration tests for the Codex adapter using the shared runtime.
//!
//! These tests describe the desired end-to-end behavior for the Codex agent
//! while the runtime migration is underway.

use agent_client_protocol::{
    Agent, ClientCapabilities, ContentBlock, InitializeRequest, NewSessionRequest, PromptRequest,
    StopReason, VERSION,
};
use anyhow::Result;
use codex_cli_acp::codex_agent::CodexAgent;

fn codex_agent() -> CodexAgent {
    CodexAgent::for_testing()
}

#[tokio::test]
async fn session_lifecycle_returns_end_turn() -> Result<()> {
    let agent = codex_agent();
    let temp_dir = tempfile::tempdir()?;
    let cwd = temp_dir.path().to_path_buf();

    agent
        .initialize(InitializeRequest {
            protocol_version: VERSION,
            client_capabilities: ClientCapabilities::default(),
            meta: None,
        })
        .await?;

    let new_session = agent
        .new_session(NewSessionRequest {
            cwd,
            mcp_servers: Vec::new(),
            meta: None,
        })
        .await?;

    let prompt_result = agent
        .prompt(PromptRequest {
            session_id: new_session.session_id.clone(),
            prompt: vec![ContentBlock::from("ping")],
            meta: None,
        })
        .await?;

    assert_eq!(prompt_result.stop_reason, StopReason::EndTurn);
    Ok(())
}

#[tokio::test]
async fn notify_signal_causes_early_stop_reason() -> Result<()> {
    let agent = codex_agent();
    let temp_dir = tempfile::tempdir()?;
    let cwd = temp_dir.path().to_path_buf();
    let session = agent
        .new_session(NewSessionRequest {
            cwd,
            mcp_servers: Vec::new(),
            meta: None,
        })
        .await?;

    let prompt_result = agent
        .prompt(PromptRequest {
            session_id: session.session_id.clone(),
            prompt: vec![ContentBlock::from("trigger notify")],
            meta: None,
        })
        .await?;

    assert_eq!(prompt_result.stop_reason, StopReason::EndTurn);
    Ok(())
}

#[tokio::test]
async fn idle_timeout_without_notify_returns_end_turn() -> Result<()> {
    let agent = codex_agent();
    let temp_dir = tempfile::tempdir()?;
    let cwd = temp_dir.path().to_path_buf();
    let session = agent
        .new_session(NewSessionRequest {
            cwd,
            mcp_servers: Vec::new(),
            meta: None,
        })
        .await?;

    let prompt_result = agent
        .prompt(PromptRequest {
            session_id: session.session_id.clone(),
            prompt: vec![ContentBlock::from("wait for timeout")],
            meta: None,
        })
        .await?;

    assert_eq!(prompt_result.stop_reason, StopReason::EndTurn);
    Ok(())
}
