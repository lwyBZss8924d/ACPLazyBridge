//! Codex proto mode event handling
//!
//! This module handles parsing and mapping of Codex native proto events to ACP events.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tracing::{debug, error, info, trace, warn};

/// Codex proto event types
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CodexEvent {
    AgentMessage {
        message: String,
        #[serde(default)]
        _timestamp: Option<String>,
    },
    AgentMessageDelta {
        delta: String,
        #[serde(default)]
        _timestamp: Option<String>,
    },
    ToolCall {
        id: String,
        name: String,
        arguments: Value,
        #[serde(default)]
        status: Option<String>,
    },
    ToolCalls {
        calls: Vec<ToolCallItem>,
    },
    TaskComplete {
        #[serde(default)]
        reason: Option<String>,
    },
    Error {
        message: String,
        #[serde(default)]
        code: Option<String>,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolCallItem {
    pub id: String,
    pub name: String,
    pub arguments: Value,
    #[serde(default)]
    pub status: Option<String>,
}

/// ACP session update event for streaming
#[derive(Debug, Clone, Serialize)]
pub struct SessionUpdate {
    pub jsonrpc: String,
    pub method: String,
    pub params: SessionUpdateParams,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionUpdateParams {
    pub session_id: String,
    pub update: SessionUpdateContent,
}

/// Content blocks for agent messages
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text {
        text: String,
    },
}

// ToolCallContent removed - using ContentBlock directly per ACP spec

/// Tool call status
#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ToolCallStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "sessionUpdate", rename_all = "snake_case")]
pub enum SessionUpdateContent {
    AgentMessageChunk {
        content: ContentBlock,
    },
    ToolCall {
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        kind: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<ToolCallStatus>,
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<Vec<ContentBlock>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        locations: Option<Vec<Value>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "rawInput")]
        raw_input: Option<Value>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "rawOutput")]
        raw_output: Option<Value>,
    },
    // Now used for tool call status updates (pending -> in_progress -> completed/failed)
    ToolCallUpdate {
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        kind: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<ToolCallStatus>,
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<Vec<ContentBlock>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        locations: Option<Vec<Value>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "rawInput")]
        raw_input: Option<Value>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "rawOutput")]
        raw_output: Option<Value>,
    },
}

/// Manages streaming from Codex proto to ACP
/// 
/// Note: Uses singular 'update' field per ACP spec SessionNotification structure,
/// not 'updates' array. This matches agent-client-protocol/rust/client.rs.
pub struct CodexStreamManager {
    session_id: String,
    tx: mpsc::UnboundedSender<SessionUpdate>,
    last_sent_chunk: Option<String>,
    finalized: bool,
}

impl CodexStreamManager {
    pub fn new(session_id: String, tx: mpsc::UnboundedSender<SessionUpdate>) -> Self {
        Self {
            session_id,
            tx,
            last_sent_chunk: None,
            finalized: false,
        }
    }

    /// Process a line from Codex stdout
    pub async fn process_line(&mut self, line: &str) -> Result<()> {
        // Skip empty lines
        if line.trim().is_empty() {
            return Ok(());
        }

        // Try to parse as JSON
        let value: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(e) => {
                debug!("Non-JSON line from Codex: {} - {}", line, e);
                return Ok(());
            }
        };

        // Try to parse as Codex event
        let event: CodexEvent = match serde_json::from_value(value.clone()) {
            Ok(e) => e,
            Err(e) => {
                debug!("Unknown Codex event format: {} - {}", value, e);
                return Ok(());
            }
        };

        trace!("Received Codex event: {:?}", event);

        match event {
            CodexEvent::AgentMessage { message, .. } => {
                self.send_chunk(message).await?;
            }
            CodexEvent::AgentMessageDelta { delta, .. } => {
                self.send_chunk(delta).await?;
            }
            CodexEvent::ToolCall {
                id,
                name,
                arguments,
                status,
            } => {
                self.send_tool_call(id, name, arguments, status.as_deref()).await?;
            }
            CodexEvent::ToolCalls { calls } => {
                for call in calls {
                    self.send_tool_call(
                        call.id,
                        call.name,
                        call.arguments,
                        call.status.as_deref(),
                    )
                    .await?;
                }
            }
            CodexEvent::TaskComplete { reason } => {
                info!("Task complete: {:?}", reason);
                self.finalized = true;
            }
            CodexEvent::Error { message, code } => {
                error!("Codex error: {} (code: {:?})", message, code);
                // Send error as a message chunk
                // TODO: When in context of a tool call, send as ToolCallUpdate with status=failed
                let error_msg = if let Some(ref code) = code {
                    format!("Error [{}]: {}", code, message)
                } else {
                    format!("Error: {}", message)
                };
                self.send_chunk(error_msg).await?;
            }
            CodexEvent::Unknown => {
                debug!("Unknown Codex event type");
            }
        }

        Ok(())
    }

    /// Send an agent message chunk
    async fn send_chunk(&mut self, content: String) -> Result<()> {
        // Skip if already finalized
        if self.finalized {
            return Ok(());
        }

        // Deduplicate if same as last chunk
        if let Some(ref last) = self.last_sent_chunk {
            if last == &content {
                trace!("Skipping duplicate chunk");
                return Ok(());
            }
        }

        let update = SessionUpdate {
            jsonrpc: "2.0".to_string(),
            method: "session/update".to_string(),
            params: SessionUpdateParams {
                session_id: self.session_id.clone(),
                update: SessionUpdateContent::AgentMessageChunk {
                    content: ContentBlock::Text {
                        text: content.clone(),
                    },
                },
            },
        };

        self.tx.send(update).context("Failed to send update")?;
        self.last_sent_chunk = Some(content);
        Ok(())
    }

    /// Send a tool call event
    async fn send_tool_call(
        &mut self,
        id: String,
        name: String,
        arguments: Value,
        status: Option<&str>,
    ) -> Result<()> {
        let tool_status = match status.unwrap_or("pending") {
            "completed" => ToolCallStatus::Completed,
            "in_progress" => ToolCallStatus::InProgress,
            "failed" => ToolCallStatus::Failed,
            _ => ToolCallStatus::Pending,
        };

        // For tool calls, use the name as the title
        let title = name.clone();
        
        // Determine tool kind based on the name (this is a heuristic)
        let kind = if name.contains("read") || name.contains("get") {
            Some("read".to_string())
        } else if name.contains("write") || name.contains("edit") || name.contains("update") {
            Some("edit".to_string())
        } else if name.contains("delete") || name.contains("remove") {
            Some("delete".to_string())
        } else if name.contains("search") || name.contains("find") {
            Some("search".to_string())
        } else if name.contains("exec") || name.contains("run") || name.contains("shell") {
            Some("execute".to_string())
        } else {
            Some("other".to_string())
        };

        // Use ToolCall for initial call, ToolCallUpdate for status changes
        let update = if tool_status == ToolCallStatus::Pending {
            // Initial tool call
            SessionUpdate {
                jsonrpc: "2.0".to_string(),
                method: "session/update".to_string(),
                params: SessionUpdateParams {
                    session_id: self.session_id.clone(),
                    update: SessionUpdateContent::ToolCall {
                        tool_call_id: id,
                        title,
                        kind,
                        status: Some(tool_status),
                        content: None,
                        locations: None,
                        raw_input: Some(arguments),
                        raw_output: None,
                    },
                },
            }
        } else {
            // Tool call update (status change)
            SessionUpdate {
                jsonrpc: "2.0".to_string(),
                method: "session/update".to_string(),
                params: SessionUpdateParams {
                    session_id: self.session_id.clone(),
                    update: SessionUpdateContent::ToolCallUpdate {
                        tool_call_id: id,
                        title: None,  // Don't repeat title in updates
                        kind: None,   // Don't repeat kind in updates
                        status: Some(tool_status),
                        content: if tool_status == ToolCallStatus::Completed {
                            // Include a preview of output when completed
                            Some(vec![ContentBlock::Text {
                                text: "Tool execution completed".to_string(),
                            }])
                        } else {
                            None
                        },
                        locations: None,
                        raw_input: None,  // Already sent in initial call
                        raw_output: if tool_status == ToolCallStatus::Completed || tool_status == ToolCallStatus::Failed {
                            // TODO: Capture actual output from tool
                            Some(json!({"status": tool_status}))
                        } else {
                            None
                        },
                    },
                },
            }
        };

        self.tx.send(update).context("Failed to send tool call")?;
        Ok(())
    }

    /// Check if streaming is finalized
    pub fn is_finalized(&self) -> bool {
        self.finalized
    }
}

/// Read and process Codex stdout
pub async fn stream_codex_output<R>(
    reader: R,
    session_id: String,
    tx: mpsc::UnboundedSender<SessionUpdate>,
) -> Result<()>
where
    R: tokio::io::AsyncRead + Unpin,
{
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    let mut manager = CodexStreamManager::new(session_id, tx);

    loop {
        line.clear();
        let bytes_read = reader
            .read_line(&mut line)
            .await
            .context("Failed to read from Codex stdout")?;

        if bytes_read == 0 {
            // EOF reached
            break;
        }

        if let Err(e) = manager.process_line(&line).await {
            error!("Error processing Codex output line '{}': {}", line.trim(), e);
            // Continue processing other lines despite individual errors
        }

        if manager.is_finalized() {
            debug!("Stream finalized by task_complete");
            break;
        }
    }

    Ok(())
}

/// Serialize a session update to JSON line
pub fn serialize_update(update: &SessionUpdate) -> Result<String> {
    serde_json::to_string(update).context("Failed to serialize session update")
}