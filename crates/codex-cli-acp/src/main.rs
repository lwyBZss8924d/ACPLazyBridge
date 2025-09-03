use acp_lazy_core::{
    permissions::{map_acp_to_codex, AcpPermissionMode},
    protocol::{Error, RequestId, Response},
    transport::{read_lines, write_line, ProcessTransport},
};
use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info};

mod codex_proto;

struct SessionState {
    _id: String,
    working_dir: String,
    codex_process: Arc<RwLock<Option<ProcessTransport>>>,
    permission_mode: AcpPermissionMode,
    stream_tx: Option<mpsc::UnboundedSender<codex_proto::SessionUpdate>>,
}

#[derive(Clone)]
struct AcpServer {
    sessions: Arc<RwLock<HashMap<String, SessionState>>>,
    protocol_version: String,
}

impl AcpServer {
    fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            protocol_version: "2024-11-05".to_string(),
        }
    }

    async fn handle_initialize(&self, id: RequestId, params: &Value) -> Result<Response> {
        let client_version = params
            .get("protocolVersion")
            .and_then(|v| v.as_str())
            .unwrap_or("2024-11-05");

        info!(
            "Initialize request from client with protocol version: {}",
            client_version
        );

        Ok(Response {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "protocolVersion": self.protocol_version,
                "capabilities": {
                    "loadSession": false,
                    "fs": {
                        "readTextFile": true,
                        "writeTextFile": true
                    }
                },
                "promptCapabilities": {
                    "image": false
                },
                "serverInfo": {
                    "name": "codex-cli-acp",
                    "version": env!("CARGO_PKG_VERSION")
                }
            })),
            error: None,
        })
    }

    async fn handle_session_new(&self, id: RequestId, params: &Value) -> Result<Response> {
        let working_dir = params
            .get("workingDirectory")
            .and_then(|v| v.as_str())
            .unwrap_or(".");

        let permission_mode = params
            .get("permissionMode")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<AcpPermissionMode>().ok())
            .unwrap_or(AcpPermissionMode::Default);

        let session_id = format!("session_{}", uuid::Uuid::new_v4());

        info!(
            "Creating new session {} with working dir: {} and permission mode: {:?}",
            session_id, working_dir, permission_mode
        );

        let session = SessionState {
            _id: session_id.clone(),
            working_dir: working_dir.to_string(),
            codex_process: Arc::new(RwLock::new(None)),
            permission_mode,
            stream_tx: None,
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session);

        Ok(Response {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "sessionId": session_id
            })),
            error: None,
        })
    }

    async fn handle_session_prompt(&self, id: RequestId, params: &Value) -> Result<Response> {
        let session_id = params
            .get("sessionId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing sessionId"))?;

        let prompt = params
            .get("prompt")
            .ok_or_else(|| anyhow!("Missing prompt"))?;

        info!("Processing prompt for session {}", session_id);

        // Setup streaming channel if not already done
        let (tx, mut rx) = mpsc::unbounded_channel::<codex_proto::SessionUpdate>();
        
        // Get or spawn Codex process and set up streaming
        let stdout = {
            let mut sessions = self.sessions.write().await;
            let session = sessions
                .get_mut(session_id)
                .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;
            
            // Store the sender for potential future use
            session.stream_tx = Some(tx.clone());
            
            let mut process_guard = session.codex_process.write().await;
            if process_guard.is_none() {
                let codex_overrides = map_acp_to_codex(session.permission_mode);
                let mut args = vec!["proto".to_string()];
                args.extend(codex_overrides.to_cli_args());

                debug!("Spawning Codex with args: {:?}", args);

                let mut process = ProcessTransport::spawn(
                    "codex", 
                    &args, 
                    None, 
                    Some(&session.working_dir)
                )
                .await
                .context("Failed to spawn Codex process")?;

                // Monitor stderr for debugging
                process.monitor_stderr()?;

                *process_guard = Some(process);
            }

            // Send prompt to Codex
            let codex_request = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "prompt",
                "params": {
                    "messages": [prompt]
                }
            });

            write_line(
                process_guard.as_mut().unwrap().stdin(),
                &codex_request.to_string(),
            )
            .await?;
            
            // Take stdout for streaming (can only be done once per process)
            process_guard.as_mut().unwrap().take_stdout()
                .ok_or_else(|| anyhow!("Failed to take stdout from Codex process"))?
        };

        // Start streaming task
        let session_id_clone = session_id.to_string();
        let stream_task = tokio::spawn(async move {
            if let Err(e) = codex_proto::stream_codex_output(
                stdout,
                session_id_clone,
                tx
            ).await {
                error!("Streaming error: {}", e);
            }
        });

        // Forward stream updates to stdout
        let mut stdout = tokio::io::stdout();
        let mut _last_update = None;
        let mut timeout_counter = 0;
        
        loop {
            tokio::select! {
                update = rx.recv() => {
                    match update {
                        Some(update) => {
                            let json = codex_proto::serialize_update(&update)?;
                            write_line(&mut stdout, &json).await?;
                            _last_update = Some(update);
                            timeout_counter = 0;
                        }
                        None => {
                            // Channel closed, streaming done
                            break;
                        }
                    }
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                    timeout_counter += 1;
                    // After 1.2 seconds of no activity, assume completion
                    if timeout_counter > 12 {
                        debug!("Idle timeout reached, ending turn");
                        break;
                    }
                }
            }
        }

        // Wait for streaming task to complete
        let _ = stream_task.await;

        Ok(Response {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "stopReason": "end_turn"
            })),
            error: None,
        })
    }

    async fn handle_session_cancel(&self, params: &Value) -> Result<()> {
        let session_id = params
            .get("sessionId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing sessionId"))?;

        info!("Cancelling session {}", session_id);

        let sessions = self.sessions.read().await;
        if let Some(session) = sessions.get(session_id) {
            let mut process_guard = session.codex_process.write().await;
            if let Some(mut process) = process_guard.take() {
                process.kill().await?;
            }
        }

        Ok(())
    }

    async fn process_message(&self, line: &str) -> Result<String> {
        let msg: Value = serde_json::from_str(line).context("Failed to parse JSON-RPC message")?;

        // Check if it's a request or notification
        if let Some(method) = msg.get("method").and_then(|m| m.as_str()) {
            if msg.get("id").is_some() {
                // It's a request
                let id = serde_json::from_value::<RequestId>(msg["id"].clone())
                    .unwrap_or(RequestId::Null);
                let empty_params = json!({});
                let params = msg.get("params").unwrap_or(&empty_params);

                // Execute handler but do not propagate error; map to JSON-RPC error response
                let result: Result<Response> = match method {
                    "initialize" => self.handle_initialize(id.clone(), params).await,
                    "session/new" => self.handle_session_new(id.clone(), params).await,
                    "session/prompt" => self.handle_session_prompt(id.clone(), params).await,
                    _ => Ok(Response::error(id.clone(), Error::method_not_found(method))),
                };

                match result {
                    Ok(response) => Ok(serde_json::to_string(&response)?),
                    Err(e) => {
                        let msg = e.to_string();
                        let rpc_error = if msg.starts_with("Missing ")
                            || msg.starts_with("Session not found")
                        {
                            Error::invalid_params(msg)
                        } else {
                            Error::internal_error(msg)
                        };
                        let error_response = Response::error(id, rpc_error);
                        Ok(serde_json::to_string(&error_response)?)
                    }
                }
            } else {
                // It's a notification
                match method {
                    "session/cancel" => {
                        let empty_params = json!({});
                        let params = msg.get("params").unwrap_or(&empty_params);
                        self.handle_session_cancel(params).await?;
                        Ok(String::new()) // No response for notifications
                    }
                    _ => {
                        debug!("Ignoring unknown notification: {}", method);
                        Ok(String::new()) // No response for notifications
                    }
                }
            }
        } else {
            // Invalid message - return error response
            let error_response = Response::error(RequestId::Null, Error::invalid_request());
            Ok(serde_json::to_string(&error_response)?)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    acp_lazy_core::logging::init();
    info!("codex-cli-acp starting");

    let server = AcpServer::new();
    let stdin = tokio::io::stdin();

    // Process stdin messages
    read_lines(stdin, |line| {
        let server = server.clone();
        async move {
            let mut stdout = tokio::io::stdout();
            match server.process_message(&line).await {
                Ok(response) if !response.is_empty() => {
                    write_line(&mut stdout, &response).await?;
                }
                Ok(_) => {
                    // Empty response for notifications
                }
                Err(e) => {
                    error!("Error processing message: {}", e);
                    // Unknown parse or internal error prior to classification; no request ID
                    let error_response =
                        Response::error(RequestId::Null, Error::internal_error(e.to_string()));
                    write_line(&mut stdout, &serde_json::to_string(&error_response)?).await?;
                }
            }
            Ok(())
        }
    })
    .await?;

    info!("codex-cli-acp exiting");
    Ok(())
}
