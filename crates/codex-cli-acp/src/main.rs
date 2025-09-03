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

/// Configuration for ACP server behavior
#[derive(Clone)]
struct ServerConfig {
    /// Idle timeout in milliseconds before ending a turn (default: 1200ms)
    idle_timeout_ms: u64,
    /// Polling interval in milliseconds for timeout checks (default: 100ms)
    polling_interval_ms: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            // Default idle timeout of 1.2 seconds
            idle_timeout_ms: std::env::var("ACPLB_IDLE_TIMEOUT_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1200),
            // Default polling interval of 100ms
            polling_interval_ms: std::env::var("ACPLB_POLLING_INTERVAL_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
        }
    }
}

mod codex_proto;
mod validation;

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
    protocol_version: u16,
    config: ServerConfig,
}

impl AcpServer {
    fn new() -> Self {
        let config = ServerConfig::default();
        debug!(
            "Initializing ACP server with idle_timeout={}ms, polling_interval={}ms",
            config.idle_timeout_ms, config.polling_interval_ms
        );
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            protocol_version: 1,  // ACP protocol version 1
            config,
        }
    }

    async fn handle_initialize(&self, id: RequestId, params: &Value) -> Result<Response> {
        // Accept protocolVersion as either integer or string for compatibility
        let client_version = params
            .get("protocolVersion")
            .and_then(|v| {
                v.as_u64()
                    .map(|n| n as u16)
                    .or_else(|| v.as_str().and_then(|s| s.parse::<u16>().ok()))
            })
            .unwrap_or(1);

        info!(
            "Initialize request from client with protocol version: {}",
            client_version
        );

        // Note: Client capabilities with fs are in the request, not response
        // The agent doesn't advertise fs capabilities - those are client capabilities

        Ok(Response {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "protocolVersion": self.protocol_version,  // Integer per spec
                "agentCapabilities": {  // Renamed from "capabilities" per spec
                    "loadSession": false,
                    "promptCapabilities": {  // Nested under agentCapabilities
                        "audio": false,
                        "embeddedContext": false,
                        "image": false
                    }
                },
                "authMethods": []  // Required field per spec, even if empty
                // Note: serverInfo removed - not in ACP spec
            })),
            error: None,
        })
    }

    async fn handle_session_new(&self, id: RequestId, params: &Value) -> Result<Response> {
        // Per ACP spec: "cwd" is the required field name
        // Accept "workingDirectory" as fallback for compatibility
        let cwd = params
            .get("cwd")
            .and_then(|v| v.as_str())
            .or_else(|| {
                // Fallback to workingDirectory for backwards compatibility
                params.get("workingDirectory").and_then(|v| v.as_str())
            })
            .ok_or_else(|| validation::RpcError::invalid_params("Missing cwd parameter"))?;

        // Validate cwd is absolute path (required by ACP spec)
        // No default to "." - must be absolute
        validation::validate_absolute_path(cwd)
            .map_err(|e| anyhow::anyhow!(e))?;

        // mcpServers is required by spec, but we can ignore content for now
        let _mcp_servers = params
            .get("mcpServers")
            .ok_or_else(|| validation::RpcError::invalid_params("Missing mcpServers parameter"))?;

        let permission_mode = params
            .get("permissionMode")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<AcpPermissionMode>().ok())
            .unwrap_or(AcpPermissionMode::Default);

        let session_id = format!("session_{}", uuid::Uuid::new_v4());

        info!(
            "Creating new session {} with cwd: {} and permission mode: {:?}",
            session_id, cwd, permission_mode
        );

        let session = SessionState {
            _id: session_id.clone(),
            working_dir: cwd.to_string(),
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

        // Setup streaming channel 
        let (tx, mut rx) = mpsc::unbounded_channel::<codex_proto::SessionUpdate>();
        
        // Create a fresh Codex process for each prompt to avoid stdout ownership issues
        let stdout = {
            let mut sessions = self.sessions.write().await;
            let session = sessions
                .get_mut(session_id)
                .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;
            
            // Store the sender for potential future use
            session.stream_tx = Some(tx.clone());
            
            // Clean up any existing process first
            let mut process_guard = session.codex_process.write().await;
            if let Some(mut existing_process) = process_guard.take() {
                debug!("Cleaning up existing Codex process");
                let _ = existing_process.kill().await; // Best effort cleanup
            }
            
            // Always create a fresh process for each prompt
            let codex_overrides = map_acp_to_codex(session.permission_mode);
            let mut args = vec!["proto".to_string()];
            args.extend(codex_overrides.to_cli_args());

            debug!("Spawning fresh Codex process with args: {:?}", args);

            // Support CODEX_CMD environment variable for custom Codex path
            let codex_cmd = std::env::var("CODEX_CMD")
                .unwrap_or_else(|_| "codex".to_string());
            
            debug!("Using Codex command: {}", codex_cmd);

            let mut process = ProcessTransport::spawn(
                &codex_cmd, 
                &args, 
                None, 
                Some(&session.working_dir)
            )
            .await
            .context("Failed to spawn Codex process. Set CODEX_CMD env var if codex is not in PATH")?;

            // Monitor stderr for debugging
            if let Err(e) = process.monitor_stderr() {
                error!("Failed to monitor stderr: {}", e);
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

            if let Err(e) = write_line(
                process.stdin(),
                &codex_request.to_string(),
            ).await {
                error!("Failed to write prompt to Codex: {}", e);
                return Err(anyhow!("Failed to send prompt to Codex: {}", e));
            }
            
            // Take stdout for streaming
            let stdout = process.take_stdout()
                .ok_or_else(|| anyhow!("Failed to take stdout from Codex process"))?;
            
            // Store the process back
            *process_guard = Some(process);
            
            stdout
        };

        // Start streaming task with better error handling
        let session_id_clone = session_id.to_string();
        let session_id_for_error = session_id.to_string();
        let stream_task = tokio::spawn(async move {
            match codex_proto::stream_codex_output(
                stdout,
                session_id_clone,
                tx
            ).await {
                Ok(_) => {
                    debug!("Streaming completed successfully");
                }
                Err(e) => {
                    error!("Streaming error in session {}: {}", session_id_for_error, e);
                    // TODO: Consider sending an error update to the client
                    // For now, we log the error and let the timeout mechanism handle completion
                }
            }
        });

        // Forward stream updates to stdout with configurable timeout
        let mut stdout = tokio::io::stdout();
        let mut _last_update = None;
        let mut timeout_counter = 0;
        let max_timeout_count = self.config.idle_timeout_ms / self.config.polling_interval_ms;
        
        debug!(
            "Starting stream forwarding with timeout={}ms ({}x{}ms polls)", 
            self.config.idle_timeout_ms, max_timeout_count, self.config.polling_interval_ms
        );
        
        loop {
            tokio::select! {
                update = rx.recv() => {
                    match update {
                        Some(update) => {
                            let json = codex_proto::serialize_update(&update)?;
                            write_line(&mut stdout, &json).await?;
                            _last_update = Some(update);
                            timeout_counter = 0; // Reset timeout on activity
                        }
                        None => {
                            // Channel closed, streaming done
                            debug!("Stream channel closed, ending turn");
                            break;
                        }
                    }
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(self.config.polling_interval_ms)) => {
                    timeout_counter += 1;
                    // After configured idle time with no activity, assume completion
                    if timeout_counter >= max_timeout_count {
                        debug!("Idle timeout reached after {}ms, ending turn", self.config.idle_timeout_ms);
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

    // Note: fs/read_text_file and fs/write_text_file are CLIENT methods
    // The agent sends these requests TO the client, not receives them
    // These handlers have been removed per ACP spec compliance

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
                    // Note: fs/* methods removed - they are client methods, not agent methods
                    _ => Ok(Response::error(id.clone(), Error::method_not_found(method))),
                };

                match result {
                    Ok(response) => Ok(serde_json::to_string(&response)?),
                    Err(e) => {
                        // Check if error is an RpcError with classification
                        let rpc_error = if let Some(rpc_err) = e.downcast_ref::<validation::RpcError>() {
                            match rpc_err.kind {
                                validation::RpcErrorKind::InvalidParams => Error::invalid_params(rpc_err.message.clone()),
                                validation::RpcErrorKind::MethodNotFound => Error::method_not_found(&rpc_err.message),
                                validation::RpcErrorKind::Internal => Error::internal_error(rpc_err.message.clone()),
                            }
                        } else {
                            // Fall back to string-based classification for legacy errors
                            let msg = e.to_string();
                            if msg.starts_with("Missing ") || msg.starts_with("Session not found") {
                                Error::invalid_params(msg)
                            } else {
                                Error::internal_error(msg)
                            }
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

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to run a JSON-RPC request through process_message
    async fn rpc(server: &AcpServer, req: serde_json::Value) -> serde_json::Value {
        let out = server.process_message(&req.to_string()).await.expect("rpc ok");
        serde_json::from_str(&out).expect("json ok")
    }

    #[tokio::test]
    async fn initialize_accepts_string_and_integer_protocol_version() {
        let server = AcpServer::new();

        // String client protocol version
        let req_str = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {"protocolVersion": "1"}
        });
        let resp_str = rpc(&server, req_str).await;
        assert_eq!(resp_str["result"]["protocolVersion"], 1);
        assert!(resp_str["result"]["agentCapabilities"].is_object());
        assert!(resp_str["result"]["authMethods"].is_array());

        // Integer client protocol version
        let req_int = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "initialize",
            "params": {"protocolVersion": 1}
        });
        let resp_int = rpc(&server, req_int).await;
        assert_eq!(resp_int["result"]["protocolVersion"], 1);
    }

    #[tokio::test]
    async fn session_new_validates_and_supports_working_directory_fallback() {
        let server = AcpServer::new();

        // Missing mcpServers -> -32602
        let req_missing_mcp = json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "session/new",
            "params": {"cwd": "/abs/path"}
        });
        let resp_missing_mcp = rpc(&server, req_missing_mcp).await;
        assert_eq!(resp_missing_mcp["error"]["code"], -32602);

        // Relative path cwd -> -32602
        let req_rel_cwd = json!({
            "jsonrpc": "2.0",
            "id": 4,
            "method": "session/new",
            "params": {"cwd": "./rel", "mcpServers": []}
        });
        let resp_rel_cwd = rpc(&server, req_rel_cwd).await;
        assert_eq!(resp_rel_cwd["error"]["code"], -32602);

        // workingDirectory fallback with absolute path -> OK
        let req_wd_fallback = json!({
            "jsonrpc": "2.0",
            "id": 5,
            "method": "session/new",
            "params": {"workingDirectory": "/abs/path", "mcpServers": []}
        });
        let resp_wd = rpc(&server, req_wd_fallback).await;
        assert!(resp_wd["result"]["sessionId"].is_string());
    }

    #[tokio::test]
    async fn fs_methods_return_method_not_found() {
        let server = AcpServer::new();
        let req = json!({
            "jsonrpc": "2.0",
            "id": 6,
            "method": "fs/read_text_file",
            "params": {"path": "/x"}
        });
        let resp = rpc(&server, req).await;
        assert_eq!(resp["error"]["code"], -32601);
    }
}
