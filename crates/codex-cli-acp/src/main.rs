use anyhow::Result;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    acp_lazy_core::logging::init();
    info!("codex-cli-acp starting (skeleton)");

    // TODO: Implement ACP server stdio loop here.
    // 1) Read InitializeRequest from stdin, respond with InitializeResponse
    // 2) Read NewSessionRequest, spawn codex proto, respond with session_id
    // 3) Read PromptRequest, stream events from codex -> agent_message_chunk/tool_call updates
    // 4) On CancelNotification, interrupt codex

    warn!("codex-cli-acp skeleton: no-op process exiting");
    Ok(())
}

