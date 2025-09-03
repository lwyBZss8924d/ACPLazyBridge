// Minimal ACP echo server example migrated from src/main.rs
// Build/run: cargo run -p codex-cli-acp --example minimal_acp_echo

use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::{
    collections::HashSet,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    sync::{Mutex, RwLock},
    time::{sleep, Duration},
};
use tracing::{debug, info, warn};

#[derive(Clone)]
struct AppState {
    sessions: Arc<RwLock<HashSet<String>>>,
    cancellations: Arc<RwLock<HashSet<String>>>,
    out: Arc<Mutex<tokio::io::Stdout>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashSet::new())),
            cancellations: Arc::new(RwLock::new(HashSet::new())),
            out: Arc::new(Mutex::new(tokio::io::stdout())),
        }
    }
}

impl AppState {
    async fn write_json(&self, v: &Value) -> Result<()> {
        let s = serde_json::to_string(v)?;
        let mut out = self.out.lock().await;
        out.write_all(s.as_bytes()).await?;
        out.write_all(b"\n").await?;
        out.flush().await?;
        Ok(())
    }

    async fn notify_agent_message_chunk(&self, session_id: &str, text: &str) -> Result<()> {
        let msg = json!({
            "jsonrpc": "2.0",
            "method": "session/update",
            "params": {
                "sessionId": session_id,
                "type": "agent_message_chunk",
                "content": text,
            }
        });
        self.write_json(&msg).await
    }

    async fn notify_turn_complete(&self, session_id: &str) -> Result<()> {
        let msg = json!({
            "jsonrpc": "2.0",
            "method": "session/update",
            "params": {
                "sessionId": session_id,
                "type": "agent-turn-complete",
            }
        });
        self.write_json(&msg).await
    }

    async fn mark_cancelled(&self, session_id: &str) {
        let mut set = self.cancellations.write().await;
        set.insert(session_id.to_string());
    }

    async fn take_cancelled(&self, session_id: &str) -> bool {
        let mut set = self.cancellations.write().await;
        set.remove(session_id)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    acp_lazy_core::logging::init();
    info!("examples/minimal_acp_echo starting");

    let state = AppState::default();
    let stdin = tokio::io::stdin();
    let mut lines = BufReader::new(stdin).lines();

    while let Some(line) = lines.next_line().await? {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let v: Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(e) => {
                let err = json!({
                    "jsonrpc": "2.0",
                    "id": Value::Null,
                    "error": {"code": -32700, "message": "Parse error", "data": e.to_string()},
                });
                state.write_json(&err).await?;
                continue;
            }
        };

        match v.get("method").and_then(|m| m.as_str()) {
            Some("initialize") => handle_initialize(&state, &v).await?,
            Some("session/new") => handle_new_session(&state, &v).await?,
            Some("session/prompt") => handle_prompt(&state, &v).await?,
            Some("session/cancel") => handle_cancel(&state, &v).await?,
            Some(other) => {
                let err = json!({
                    "jsonrpc": "2.0",
                    "id": v.get("id").cloned().unwrap_or(Value::Null),
                    "error": {"code": -32601, "message": format!("Method not found: {}", other)},
                });
                state.write_json(&err).await?;
            }
            None => {}
        }
    }

    warn!("examples/minimal_acp_echo exiting");
    Ok(())
}

async fn handle_initialize(state: &AppState, req: &Value) -> Result<()> {
    let id = req.get("id").cloned().unwrap_or(Value::Null);
    let protocol_version = req
        .get("params")
        .and_then(|p| p.get("protocolVersion"))
        .cloned()
        .unwrap_or(Value::String("2024-11-05".into()));
    let resp = json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": {
            "protocolVersion": protocol_version,
            "capabilities": {"loadSession": true},
            "promptCapabilities": {"image": false},
            "serverInfo": {"name": "minimal_acp_echo", "version": env!("CARGO_PKG_VERSION")}
        }
    });
    state.write_json(&resp).await
}

async fn handle_new_session(state: &AppState, req: &Value) -> Result<()> {
    let id = req.get("id").cloned().unwrap_or(Value::Null);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let session_id = format!("session_{}", now.as_millis());
    {
        let mut sessions = state.sessions.write().await;
        sessions.insert(session_id.clone());
    }
    let resp = json!({"jsonrpc":"2.0","id":id,"result":{"sessionId":session_id}});
    state.write_json(&resp).await
}

async fn handle_prompt(state: &AppState, req: &Value) -> Result<()> {
    let id = req.get("id").cloned().unwrap_or(Value::Null);
    let params = req.get("params").ok_or_else(|| anyhow!("Missing params"))?;
    let session_id = params
        .get("sessionId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing params.sessionId"))?;

    let prompt_text = if let Some(s) = params.get("prompt").and_then(|v| v.as_str()) {
        s.to_string()
    } else if let Some(arr) = params.get("prompt").and_then(|v| v.as_array()) {
        let mut acc = String::new();
        for block in arr {
            if let Some(t) = block.get("text").and_then(|t| t.as_str()) {
                acc.push_str(t);
            }
        }
        acc
    } else {
        String::new()
    };

    let session_id_owned = session_id.to_string();
    let state_clone = state.clone();

    tokio::spawn(async move {
        if prompt_text.is_empty() {
            let _ = state_clone
                .notify_agent_message_chunk(&session_id_owned, "(no content)")
                .await;
        } else {
            let mid = prompt_text.len() / 2;
            let (first, second) = prompt_text.split_at(mid);
            let _ = state_clone
                .notify_agent_message_chunk(&session_id_owned, &format!("Echo: {}", first))
                .await;
            sleep(Duration::from_millis(200)).await;
            if state_clone.take_cancelled(&session_id_owned).await {
                let _ = state_clone.notify_turn_complete(&session_id_owned).await;
                let _ = state_clone
                    .write_json(
                        &json!({"jsonrpc":"2.0","id":id,"result":{"stopReason":"cancelled"}}),
                    )
                    .await;
                return;
            }
            let _ = state_clone
                .notify_agent_message_chunk(&session_id_owned, second)
                .await;
        }
        let _ = state_clone.notify_turn_complete(&session_id_owned).await;
        let _ = state_clone
            .write_json(&json!({"jsonrpc":"2.0","id":id,"result":{"stopReason":"end_turn"}}))
            .await;
    });

    Ok(())
}

async fn handle_cancel(state: &AppState, req: &Value) -> Result<()> {
    let params = req.get("params").cloned().unwrap_or(Value::Null);
    let session_id = params
        .get("sessionId")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if session_id.is_empty() {
        debug!("cancel without sessionId");
        return Ok(());
    }
    state.mark_cancelled(session_id).await;
    Ok(())
}
