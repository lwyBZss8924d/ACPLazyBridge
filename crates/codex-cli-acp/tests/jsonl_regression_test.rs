use std::path::PathBuf;

use agent_client_protocol::{
    Agent, ClientCapabilities, ContentBlock, InitializeRequest, NewSessionRequest, PromptRequest,
    StopReason, VERSION,
};
use anyhow::Result;
use codex_cli_acp::codex_agent::CodexAgent;

fn codex_agent() -> CodexAgent {
    CodexAgent::for_testing()
}

fn temp_cwd() -> Result<PathBuf> {
    let dir = tempfile::tempdir()?;
    Ok(dir.path().to_path_buf())
}

#[tokio::test]
async fn jsonl_regression_playback_remains_compatible() -> Result<()> {
    let agent = codex_agent();

    agent
        .initialize(InitializeRequest {
            protocol_version: VERSION,
            client_capabilities: ClientCapabilities::default(),
            meta: None,
        })
        .await?;

    let session = agent
        .new_session(NewSessionRequest {
            cwd: temp_cwd()?,
            mcp_servers: Vec::new(),
            meta: None,
        })
        .await?;

    let playback_result = agent
        .prompt(PromptRequest {
            session_id: session.session_id.clone(),
            prompt: vec![ContentBlock::from("jsonl regression placeholder")],
            meta: None,
        })
        .await?;

    assert_eq!(playback_result.stop_reason, StopReason::EndTurn);
    Ok(())
}
