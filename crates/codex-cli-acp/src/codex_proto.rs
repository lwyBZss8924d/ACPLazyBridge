//! Codex proto mode event handling
//!
//! This module handles parsing and mapping of Codex native proto events to ACP events.

use crate::tool_calls::{
    extract_shell_command, extract_shell_params, format_tool_output, map_tool_kind,
    MAX_OUTPUT_PREVIEW_BYTES,
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tracing::{debug, error, info, trace};

/// Codex proto event types
#[derive(Debug, Clone, Deserialize, Serialize)]
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
        #[serde(default)]
        output: Option<Value>,
        #[serde(default)]
        error: Option<String>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolCallItem {
    pub id: String,
    pub name: String,
    pub arguments: Value,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub output: Option<Value>,
    #[serde(default)]
    pub error: Option<String>,
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
    Text { text: String },
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
    /// Track tool calls that have been sent to avoid duplicate pending events
    tool_call_states: HashMap<String, ToolCallStatus>,
}

impl CodexStreamManager {
    pub fn new(session_id: String, tx: mpsc::UnboundedSender<SessionUpdate>) -> Self {
        Self {
            session_id,
            tx,
            last_sent_chunk: None,
            finalized: false,
            tool_call_states: HashMap::new(),
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
                output,
                error,
            } => {
                self.send_tool_call(id, name, arguments, status.as_deref(), output, error)
                    .await?;
            }
            CodexEvent::ToolCalls { calls } => {
                for call in calls {
                    self.send_tool_call(
                        call.id,
                        call.name,
                        call.arguments,
                        call.status.as_deref(),
                        call.output,
                        call.error,
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

                // Map Codex error codes to semantic categories
                let error_category = match code.as_deref() {
                    Some("timeout") | Some("TIMEOUT") => "timeout",
                    Some("permission_denied") | Some("PERMISSION_DENIED") => "permission_denied",
                    Some("not_found") | Some("NOT_FOUND") => "not_found",
                    Some("cancelled") | Some("CANCELLED") => "cancelled",
                    Some("rate_limit") | Some("RATE_LIMIT") => "rate_limit",
                    _ => "error",
                };

                // Check if this error is in the context of a tool call
                // If we have active tool calls, map the error to the most recent one
                if let Some((tool_id, _)) = self.tool_call_states.iter().last() {
                    // Send as ToolCallUpdate with status=failed for the last tool
                    let tool_id_str = tool_id.clone();

                    // Format error message with category
                    let error_text = match error_category {
                        "timeout" => format!("Tool execution timed out: {}", message),
                        "permission_denied" => format!("Permission denied: {}", message),
                        "not_found" => format!("Resource not found: {}", message),
                        "cancelled" => format!("Tool execution cancelled: {}", message),
                        "rate_limit" => format!("Rate limit exceeded: {}", message),
                        _ => {
                            if let Some(ref code) = code {
                                format!("Error [{}]: {}", code, message)
                            } else {
                                format!("Error: {}", message)
                            }
                        }
                    };

                    let update = SessionUpdate {
                        jsonrpc: "2.0".to_string(),
                        method: "session/update".to_string(),
                        params: SessionUpdateParams {
                            session_id: self.session_id.clone(),
                            update: SessionUpdateContent::ToolCallUpdate {
                                tool_call_id: tool_id_str.clone(),
                                title: None,
                                kind: None,
                                status: Some(ToolCallStatus::Failed),
                                content: Some(vec![ContentBlock::Text { text: error_text }]),
                                locations: None,
                                raw_input: None,
                                raw_output: Some(json!({
                                    "error": message,
                                    "code": code,
                                    "category": error_category
                                })),
                            },
                        },
                    };
                    self.tx
                        .send(update)
                        .context("Failed to send error update")?;

                    // Mark the tool as failed in our tracking
                    self.tool_call_states
                        .insert(tool_id_str, ToolCallStatus::Failed);
                } else {
                    // No tool context, send as a message chunk
                    let error_msg = match error_category {
                        "timeout" => format!("Operation timed out: {}", message),
                        "permission_denied" => format!("Permission denied: {}", message),
                        "not_found" => format!("Not found: {}", message),
                        "cancelled" => format!("Operation cancelled: {}", message),
                        "rate_limit" => format!("Rate limit exceeded: {}", message),
                        _ => {
                            if let Some(ref code) = code {
                                format!("Error [{}]: {}", code, message)
                            } else {
                                format!("Error: {}", message)
                            }
                        }
                    };
                    self.send_chunk(error_msg).await?;
                }
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

    /// Send a tool call event with enhanced status tracking and output formatting
    async fn send_tool_call(
        &mut self,
        id: String,
        name: String,
        arguments: Value,
        status: Option<&str>,
        output: Option<Value>,
        error: Option<String>,
    ) -> Result<()> {
        // Parse the status
        let tool_status = match status.unwrap_or("pending") {
            "completed" | "success" => ToolCallStatus::Completed,
            "in_progress" | "running" => ToolCallStatus::InProgress,
            "failed" | "error" => ToolCallStatus::Failed,
            _ => ToolCallStatus::Pending,
        };

        // Check if we've already sent this tool call to track state transitions
        let previous_status = self.tool_call_states.get(&id).copied();

        // Determine if this is an initial call or an update
        let is_initial = previous_status.is_none();

        // Update our tracking
        self.tool_call_states.insert(id.clone(), tool_status);

        // Extract shell parameters if this is a shell tool
        let shell_params =
            if name.contains("shell") || name.contains("bash") || name.contains("command") {
                Some(extract_shell_params(&name, &arguments))
            } else {
                None
            };

        // Determine the title - for shell commands, include workdir if different
        let title = if let Some(cmd) = extract_shell_command(&name, &arguments) {
            if let Some(ref params) = shell_params {
                if let Some(ref workdir) = params.workdir {
                    format!("{}: {} (in {})", name, cmd, workdir)
                } else {
                    format!("{}: {}", name, cmd)
                }
            } else {
                format!("{}: {}", name, cmd)
            }
        } else {
            name.clone()
        };

        // Map to ACP tool kind using our utility
        let kind = Some(map_tool_kind(&name));

        // Build locations array if workdir is present
        let locations = if let Some(ref params) = shell_params {
            params.workdir.as_ref().map(|dir| {
                vec![json!({
                    "path": dir,
                    "type": "directory"
                })]
            })
        } else {
            None
        };

        // Format content based on output/error
        let content =
            if tool_status == ToolCallStatus::Completed || tool_status == ToolCallStatus::Failed {
                let mut content_blocks = Vec::new();

                // Add output if present
                if let Some(ref out) = output {
                    let formatted = format_tool_output(&name, out, MAX_OUTPUT_PREVIEW_BYTES);
                    if !formatted.is_empty() {
                        content_blocks.push(ContentBlock::Text { text: formatted });
                    }
                }

                // Add error if present
                if let Some(ref err) = error {
                    content_blocks.push(ContentBlock::Text {
                        text: format!("[Error]: {}", err),
                    });
                }

                // Default message if no output or error
                if content_blocks.is_empty() {
                    content_blocks.push(ContentBlock::Text {
                        text: match tool_status {
                            ToolCallStatus::Completed => {
                                "Tool execution completed successfully".to_string()
                            }
                            ToolCallStatus::Failed => "Tool execution failed".to_string(),
                            _ => format!("Tool status: {:?}", tool_status),
                        },
                    });
                }

                Some(content_blocks)
            } else if tool_status == ToolCallStatus::InProgress {
                Some(vec![ContentBlock::Text {
                    text: "Tool is running...".to_string(),
                }])
            } else {
                None
            };

        // Enhance raw_input with extracted parameters for debugging
        let enhanced_raw_input = if let Some(ref params) = shell_params {
            let mut enhanced = json!({
                "original": arguments,
                "extracted": {}
            });

            if let Some(obj) = enhanced["extracted"].as_object_mut() {
                if let Some(ref cmd) = params.command {
                    obj.insert("command".to_string(), json!(cmd));
                }
                if let Some(ref workdir) = params.workdir {
                    obj.insert("workdir".to_string(), json!(workdir));
                }
                if let Some(timeout) = params.timeout_ms {
                    obj.insert("timeout_ms".to_string(), json!(timeout));
                }
                if let Some(escalated) = params.with_escalated_permissions {
                    obj.insert("with_escalated_permissions".to_string(), json!(escalated));
                }
                if let Some(ref justification) = params.justification {
                    obj.insert("justification".to_string(), json!(justification));
                }
            }

            enhanced
        } else {
            arguments.clone()
        };

        // Prepare raw output for completed/failed states
        let raw_output = if tool_status == ToolCallStatus::Completed
            || tool_status == ToolCallStatus::Failed
        {
            if let Some(ref err) = error {
                Some(json!({
                    "status": "failed",
                    "error": err
                }))
            } else if let Some(ref out) = output {
                // Include full output in raw_output
                Some(out.clone())
            } else {
                Some(json!({
                    "status": if tool_status == ToolCallStatus::Completed { "completed" } else { "failed" }
                }))
            }
        } else {
            None
        };

        // Create the appropriate update based on whether this is initial or update
        let update = if is_initial {
            // Initial tool call - send as ToolCall
            debug!(
                "Sending initial tool call {} with status {:?}",
                id, tool_status
            );
            SessionUpdate {
                jsonrpc: "2.0".to_string(),
                method: "session/update".to_string(),
                params: SessionUpdateParams {
                    session_id: self.session_id.clone(),
                    update: SessionUpdateContent::ToolCall {
                        tool_call_id: id,
                        title: title.clone(),
                        kind: kind.clone(),
                        status: Some(tool_status),
                        content: content.clone(),
                        locations,
                        raw_input: Some(enhanced_raw_input.clone()),
                        raw_output: raw_output.clone(),
                    },
                },
            }
        } else {
            // Status update - send as ToolCallUpdate
            debug!(
                "Sending tool call update for {} from {:?} to {:?}",
                id, previous_status, tool_status
            );

            // Only send update if status actually changed or we have new output
            if previous_status == Some(tool_status) && output.is_none() && error.is_none() {
                trace!("Skipping duplicate status update for tool {}", id);
                return Ok(());
            }

            SessionUpdate {
                jsonrpc: "2.0".to_string(),
                method: "session/update".to_string(),
                params: SessionUpdateParams {
                    session_id: self.session_id.clone(),
                    update: SessionUpdateContent::ToolCallUpdate {
                        tool_call_id: id,
                        title: None, // Don't repeat title in updates
                        kind: None,  // Don't repeat kind in updates
                        status: Some(tool_status),
                        content,
                        locations: None,
                        raw_input: None, // Already sent in initial call
                        raw_output,
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
            error!(
                "Error processing Codex output line '{}': {}",
                line.trim(),
                e
            );
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
