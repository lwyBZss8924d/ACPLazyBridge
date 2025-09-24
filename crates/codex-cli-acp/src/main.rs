use acp_lazy_core::logging;
use agent_client_protocol::Client;
use anyhow::Result;
use codex_cli_acp::codex_agent::CodexAgent;
use tokio::sync::mpsc;
use tokio::task::LocalSet;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
use tracing::warn;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    logging::init();

    let stdout = tokio::io::stdout().compat_write();
    let stdin = tokio::io::stdin().compat();

    let local_set = LocalSet::new();
    local_set
        .run_until(async move {
            let (notify_tx, mut notify_rx) = mpsc::unbounded_channel();
            let agent = CodexAgent::new_with_notifier(Some(notify_tx));

            let (conn, io_task) =
                agent_client_protocol::AgentSideConnection::new(agent, stdout, stdin, |fut| {
                    tokio::task::spawn_local(fut);
                });

            tokio::task::spawn_local(async move {
                while let Some(notification) = notify_rx.recv().await {
                    if let Err(err) = conn.session_notification(notification).await {
                        warn!("Failed to send session notification: {:?}", err);
                        break;
                    }
                }
            });

            io_task.await
        })
        .await?;

    Ok(())
}
