use acp_lazy_core::logging;
use anyhow::Result;
use codex_cli_acp::codex_agent::CodexAgent;
use serde_json::json;
use tokio::io::AsyncWriteExt;
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

            let (_conn, io_task) =
                agent_client_protocol::AgentSideConnection::new(agent, stdout, stdin, |fut| {
                    tokio::task::spawn_local(fut);
                });

            tokio::task::spawn_local(async move {
                while let Some(notification) = notify_rx.recv().await {
                    tracing::debug!(
                        "Writing SessionNotification as JSON-RPC: session_id={}, update_type={:?}",
                        notification.session_id.0,
                        std::mem::discriminant(&notification.update)
                    );

                    // T033c fix: Write notification as proper JSON-RPC notification to stdout
                    // Zed expects JSON-RPC format with "jsonrpc", "method", and "params" fields
                    let json_rpc_notification = json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": notification
                    });

                    match serde_json::to_string(&json_rpc_notification) {
                        Ok(json) => {
                            println!("{}", json);
                            // Ensure immediate delivery
                            if let Err(e) = tokio::io::stdout().flush().await {
                                warn!("Failed to flush stdout: {}", e);
                            }
                            tracing::debug!("Sent JSON-RPC notification: {}", json);
                        }
                        Err(e) => {
                            warn!("Failed to serialize JSON-RPC notification: {}", e);
                        }
                    }
                }
                tracing::debug!("SessionNotification channel closed");
            });

            io_task.await
        })
        .await?;

    Ok(())
}
