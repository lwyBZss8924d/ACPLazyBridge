//! Codex proto mode event handling
//!
//! This module translates Codex CLI proto events into the official
//! `agent_client_protocol` streaming surface so downstream ACP clients receive
//! canonical `SessionNotification` and `SessionUpdate` payloads.  It is
//! responsible for:
//!
//! - Deserialising Codex protocol JSON into strongly typed helper events.
//! - Building ACP content blocks (text, reasoning, plan, available commands,
//!   mode updates, tool calls) directly with the v0.4.2 models.
//! - Preserving Codex metadata such as tool raw I/O, stop reasons, and
//!   notification timing while applying the LastChunkGuard deduplication rules.
//! - Emitting updates through `AgentSideConnection::session_notification` so
//!   notify/idle stop reasons propagate as `StopReason::EndTurn` and
//!   `StopReason::IdleTimeout`.
//!
//! The module intentionally avoids defining forked ACP types—any schema changes
//! must be pulled from the upstream crate to stay compliant with the SDD
//! constitution’s anti-abstraction rule (Article VIII).

use crate::tool_calls::{
    extract_shell_command, extract_shell_params, format_tool_output, map_tool_kind,
    MAX_OUTPUT_PREVIEW_BYTES,
};
use agent_client_protocol::Error as AcpError;
use agent_client_protocol::{
    AudioContent, AvailableCommand, BlobResourceContents, ContentBlock, EmbeddedResource,
    EmbeddedResourceResource, ImageContent, Plan, PlanEntry, PlanEntryPriority, PlanEntryStatus,
    ResourceLink, SessionId, SessionModeId, SessionNotification, SessionUpdate, TextContent,
    TextResourceContents, ToolCall, ToolCallContent, ToolCallId, ToolCallLocation, ToolCallStatus,
    ToolCallUpdate, ToolCallUpdateFields, ToolKind,
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tracing::{debug, error, info, trace};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CodexUserMessageEvent {
    pub message: String,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodexPlanItem {
    pub step: String,
    pub status: CodexStepStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CodexStepStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CodexPlanUpdateEvent {
    #[serde(default)]
    pub explanation: Option<String>,
    pub plan: Vec<CodexPlanItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CodexToolAnnotations {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CodexToolDefinition {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "inputSchema", default)]
    pub input_schema: Option<Value>,
    #[serde(default)]
    pub annotations: Option<CodexToolAnnotations>,
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CodexSessionConfiguredEvent {
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

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
    UserMessage(CodexUserMessageEvent),
    AgentReasoning {
        text: String,
    },
    AgentReasoningDelta {
        delta: String,
    },
    AgentReasoningRawContent {
        text: String,
    },
    AgentReasoningRawContentDelta {
        delta: String,
    },
    AgentReasoningSectionBreak,
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
    PlanUpdate(CodexPlanUpdateEvent),
    McpListToolsResponse {
        tools: HashMap<String, CodexToolDefinition>,
    },
    SessionConfigured(CodexSessionConfiguredEvent),
    TaskStarted {
        #[serde(default)]
        model_context_window: Option<u32>,
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

#[derive(Debug, Clone)]
struct ToolCallRecord {
    // Cached state so we can emit ToolCall/ToolCallUpdate pairs with the right
    // ACP fields even when Codex only sends partial deltas.
    status: ToolCallStatus,
    title: String,
    kind: ToolKind,
    locations: Vec<ToolCallLocation>,
    raw_input: Option<Value>,
}

/// Manages streaming from Codex proto to ACP
///
/// Note: Uses singular update field per ACP spec SessionNotification structure,
/// not updates array. This matches agent-client-protocol/rust/client.rs.
pub struct CodexStreamManager {
    session_id: SessionId,
    tx: mpsc::UnboundedSender<SessionNotification>,
    last_text_chunk: Option<String>,
    finalized: bool,
    tool_calls: HashMap<String, ToolCallRecord>,
    last_tool_call_id: Option<String>,
}

impl CodexStreamManager {
    pub fn new(session_id: SessionId, tx: mpsc::UnboundedSender<SessionNotification>) -> Self {
        Self {
            session_id,
            tx,
            last_text_chunk: None,
            finalized: false,
            tool_calls: HashMap::new(),
            last_tool_call_id: None,
        }
    }

    /// Process a line from Codex stdout
    pub async fn process_line(&mut self, line: &str) -> Result<()> {
        if line.trim().is_empty() {
            return Ok(());
        }

        let value: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(e) => {
                debug!("Non-JSON line from Codex: {} - {}", line, e);
                return Ok(());
            }
        };

        // Codex proto outputs events in {"id":"...", "msg":{...}} format
        // Extract the msg field which contains the actual event
        let event_value = if let Some(msg) = value.get("msg") {
            msg.clone()
        } else {
            // Fall back to parsing the whole value for backward compatibility
            value.clone()
        };

        let event: CodexEvent = match serde_json::from_value(event_value.clone()) {
            Ok(e) => e,
            Err(e) => {
                debug!("Unknown Codex event format: {} - {}", event_value, e);
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
            CodexEvent::UserMessage(event) => {
                self.send_user_message(event).await?;
            }
            CodexEvent::AgentReasoning { text } => {
                self.send_agent_thought(&text).await?;
            }
            CodexEvent::AgentReasoningDelta { delta } => {
                self.send_agent_thought(&delta).await?;
            }
            CodexEvent::AgentReasoningRawContent { text } => {
                self.send_agent_thought(&text).await?;
            }
            CodexEvent::AgentReasoningRawContentDelta { delta } => {
                self.send_agent_thought(&delta).await?;
            }
            CodexEvent::AgentReasoningSectionBreak => {}
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
            CodexEvent::PlanUpdate(update) => {
                self.send_plan_update(update).await?;
            }
            CodexEvent::McpListToolsResponse { tools } => {
                self.send_available_commands(tools).await?;
            }
            CodexEvent::SessionConfigured(event) => {
                self.send_current_mode_update(event).await?;
            }
            CodexEvent::TaskStarted {
                model_context_window,
            } => {
                debug!("Task started: context_window={:?}", model_context_window);
                // Task started indicates Codex is processing the request
            }
            CodexEvent::TaskComplete { reason } => {
                info!("Task complete: {:?}", reason);
                self.finalized = true;
            }
            CodexEvent::Error { message, code } => {
                self.handle_error(message, code).await?;
            }
            CodexEvent::Unknown => {
                debug!("Unknown Codex event type");
            }
        }

        Ok(())
    }

    async fn send_chunk(&mut self, content: String) -> Result<()> {
        if self.finalized {
            trace!("Skipping chunk after finalization");
            return Ok(());
        }

        if let Some(ref last) = self.last_text_chunk {
            if last == &content {
                trace!("Skipping duplicate chunk");
                return Ok(());
            }
        }

        let block = content_block_from_string(&content);
        let notification =
            self.build_notification(SessionUpdate::AgentMessageChunk { content: block });
        debug!(
            "Sending AgentMessageChunk notification for session {}: content_len={}",
            self.session_id.0,
            content.len()
        );
        self.tx
            .send(notification)
            .context("Failed to send update")?;
        self.last_text_chunk = Some(content);
        Ok(())
    }

    async fn send_user_message(&mut self, event: CodexUserMessageEvent) -> Result<()> {
        if !event.message.trim().is_empty() {
            let block = content_block_from_string(&event.message);
            let notification =
                self.build_notification(SessionUpdate::UserMessageChunk { content: block });
            self.tx
                .send(notification)
                .context("Failed to send user message chunk")?;
        }

        if let Some(images) = event.images {
            for image in images {
                if let Some(block) = content_block_from_image_source(&image) {
                    let notification =
                        self.build_notification(SessionUpdate::UserMessageChunk { content: block });
                    self.tx
                        .send(notification)
                        .context("Failed to send user image chunk")?;
                }
            }
        }

        Ok(())
    }

    async fn send_agent_thought(&mut self, text: &str) -> Result<()> {
        if text.trim().is_empty() {
            return Ok(());
        }

        let block = content_block_from_string(text);
        let notification =
            self.build_notification(SessionUpdate::AgentThoughtChunk { content: block });
        self.tx
            .send(notification)
            .context("Failed to send agent thought chunk")?;
        Ok(())
    }

    async fn send_plan_update(&mut self, update: CodexPlanUpdateEvent) -> Result<()> {
        if update.plan.is_empty() {
            return Ok(());
        }

        let entries: Vec<PlanEntry> = update.plan.into_iter().map(plan_entry_from_codex).collect();

        let meta = update.explanation.and_then(|explanation| {
            if explanation.trim().is_empty() {
                None
            } else {
                Some(json!({ "explanation": explanation }))
            }
        });

        let plan = Plan { entries, meta };
        let notification = self.build_notification(SessionUpdate::Plan(plan));
        self.tx
            .send(notification)
            .context("Failed to send plan update")?;
        Ok(())
    }

    async fn send_available_commands(
        &mut self,
        tools: HashMap<String, CodexToolDefinition>,
    ) -> Result<()> {
        let mut commands = Vec::new();
        for (key, tool) in tools {
            if let Some(command) = available_command_from_tool(&key, tool) {
                commands.push(command);
            }
        }

        if commands.is_empty() {
            return Ok(());
        }

        let notification = self.build_notification(SessionUpdate::AvailableCommandsUpdate {
            available_commands: commands,
        });
        self.tx
            .send(notification)
            .context("Failed to send available commands update")?;
        Ok(())
    }

    async fn send_current_mode_update(&mut self, event: CodexSessionConfiguredEvent) -> Result<()> {
        let Some(model) = event.model.filter(|m| !m.trim().is_empty()) else {
            return Ok(());
        };

        let notification = self.build_notification(SessionUpdate::CurrentModeUpdate {
            current_mode_id: SessionModeId(Arc::from(model.as_str())),
        });

        self.tx
            .send(notification)
            .context("Failed to send current mode update")?;
        Ok(())
    }

    async fn send_tool_call(
        &mut self,
        id: String,
        name: String,
        arguments: Value,
        status: Option<&str>,
        output: Option<Value>,
        error: Option<String>,
    ) -> Result<()> {
        let tool_status = match status.unwrap_or("pending") {
            "completed" | "success" => ToolCallStatus::Completed,
            "in_progress" | "running" => ToolCallStatus::InProgress,
            "failed" | "error" => ToolCallStatus::Failed,
            _ => ToolCallStatus::Pending,
        };

        self.last_tool_call_id = Some(id.clone());

        let shell_params =
            if name.contains("shell") || name.contains("bash") || name.contains("command") {
                Some(extract_shell_params(&name, &arguments))
            } else {
                None
            };

        let title = if let Some(cmd) = extract_shell_command(&name, &arguments) {
            if let Some(params) = shell_params.as_ref() {
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

        let kind = map_tool_kind(&name);

        let locations = shell_params
            .as_ref()
            .and_then(|params| {
                params.workdir.as_ref().map(|dir| {
                    vec![ToolCallLocation {
                        path: PathBuf::from(dir),
                        line: None,
                        meta: None,
                    }]
                })
            })
            .unwrap_or_default();

        let mut content_blocks: Vec<ToolCallContent> = Vec::new();
        if let Some(ref out) = output {
            let formatted = format_tool_output(&name, out, MAX_OUTPUT_PREVIEW_BYTES);
            if !formatted.is_empty() {
                content_blocks.push(ToolCallContent::from(formatted));
            }
        }

        if let Some(ref err) = error {
            content_blocks.push(ToolCallContent::from(format!("[Error]: {}", err)));
        }

        if matches!(
            tool_status,
            ToolCallStatus::Completed | ToolCallStatus::Failed
        ) && content_blocks.is_empty()
        {
            let default_text = if tool_status == ToolCallStatus::Completed {
                "Tool execution completed successfully"
            } else {
                "Tool execution failed"
            };
            content_blocks.push(ToolCallContent::from(default_text));
        } else if tool_status == ToolCallStatus::InProgress && content_blocks.is_empty() {
            content_blocks.push(ToolCallContent::from("Tool is running..."));
        }

        let enhanced_raw_input = if let Some(params) = shell_params.as_ref() {
            let mut enhanced = json!({
                "original": arguments.clone(),
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

        let raw_output = if matches!(
            tool_status,
            ToolCallStatus::Completed | ToolCallStatus::Failed
        ) {
            if let Some(ref err) = error {
                Some(json!({
                    "status": "failed",
                    "error": err
                }))
            } else if let Some(out) = output.clone() {
                Some(out)
            } else {
                Some(json!({
                    "status": if tool_status == ToolCallStatus::Completed {
                        "completed"
                    } else {
                        "failed"
                    }
                }))
            }
        } else {
            None
        };

        let tool_call_id = ToolCallId(Arc::from(id.as_str()));
        let record = ToolCallRecord {
            status: tool_status,
            title: title.clone(),
            kind,
            locations: locations.clone(),
            raw_input: Some(enhanced_raw_input.clone()),
        };

        if let Some(previous) = self.tool_calls.get(&id).cloned() {
            let mut fields = ToolCallUpdateFields {
                status: Some(tool_status),
                ..Default::default()
            };

            if previous.title != title {
                fields.title = Some(title.clone());
            }
            if previous.kind != kind {
                fields.kind = Some(kind);
            }
            if previous.locations != locations {
                fields.locations = Some(locations.clone());
            }
            if previous.raw_input.as_ref() != Some(&enhanced_raw_input) {
                fields.raw_input = Some(enhanced_raw_input.clone());
            }
            if !content_blocks.is_empty() {
                fields.content = Some(content_blocks.clone());
            }
            if let Some(value) = raw_output.clone() {
                fields.raw_output = Some(value);
            }

            let redundant = previous.status == tool_status
                && fields.content.is_none()
                && fields.raw_output.is_none()
                && fields.title.is_none()
                && fields.kind.is_none()
                && fields.locations.is_none()
                && fields.raw_input.is_none();

            if redundant {
                self.tool_calls.insert(id, record);
                return Ok(());
            }

            if fields != ToolCallUpdateFields::default() {
                let notification =
                    self.build_notification(SessionUpdate::ToolCallUpdate(ToolCallUpdate {
                        id: tool_call_id,
                        fields,
                        meta: None,
                    }));
                self.tx
                    .send(notification)
                    .context("Failed to send tool call update")?;
            }
        } else {
            debug!(
                "Sending initial tool call {} with status {:?}",
                id, tool_status
            );
            let notification = self.build_notification(SessionUpdate::ToolCall(ToolCall {
                id: tool_call_id,
                title,
                kind,
                status: tool_status,
                content: content_blocks,
                locations,
                raw_input: Some(enhanced_raw_input.clone()),
                raw_output: raw_output.clone(),
                meta: None,
            }));
            self.tx
                .send(notification)
                .context("Failed to send tool call")?;
        }

        self.tool_calls.insert(id, record);

        Ok(())
    }

    async fn handle_error(&mut self, message: String, code: Option<String>) -> Result<()> {
        error!("Codex error: {} (code: {:?})", message, code);

        let error_category = match code.as_deref() {
            Some("timeout") | Some("TIMEOUT") => "timeout",
            Some("permission_denied") | Some("PERMISSION_DENIED") => "permission_denied",
            Some("not_found") | Some("NOT_FOUND") => "not_found",
            Some("cancelled") | Some("CANCELLED") => "cancelled",
            Some("rate_limit") | Some("RATE_LIMIT") => "rate_limit",
            _ => "error",
        };

        if let Some(tool_id) = self.last_tool_call_id.clone() {
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

            let mut fields = ToolCallUpdateFields {
                status: Some(ToolCallStatus::Failed),
                content: Some(vec![ToolCallContent::from(error_text)]),
                raw_output: Some(acp_error_value(error_category, &message, code.as_deref())),
                ..Default::default()
            };

            if let Some(record) = self.tool_calls.get_mut(&tool_id) {
                record.status = ToolCallStatus::Failed;
                if let Some(raw_input) = record.raw_input.clone() {
                    fields.raw_input = Some(raw_input);
                }
                if !record.locations.is_empty() {
                    fields.locations = Some(record.locations.clone());
                }
                fields.title = Some(record.title.clone());
                fields.kind = Some(record.kind);

                let notification =
                    self.build_notification(SessionUpdate::ToolCallUpdate(ToolCallUpdate {
                        id: ToolCallId(Arc::from(tool_id.as_str())),
                        fields,
                        meta: None,
                    }));
                self.tx
                    .send(notification)
                    .context("Failed to send error update")?;
            } else {
                let notification =
                    self.build_notification(SessionUpdate::ToolCallUpdate(ToolCallUpdate {
                        id: ToolCallId(Arc::from(tool_id.as_str())),
                        fields,
                        meta: None,
                    }));
                self.tx
                    .send(notification)
                    .context("Failed to send error update")?;
            }
        } else {
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

        Ok(())
    }

    fn build_notification(&self, update: SessionUpdate) -> SessionNotification {
        SessionNotification {
            session_id: self.session_id.clone(),
            update,
            meta: None,
        }
    }

    /// Check if streaming is finalized
    pub fn is_finalized(&self) -> bool {
        self.finalized
    }
}

fn plan_entry_from_codex(item: CodexPlanItem) -> PlanEntry {
    PlanEntry {
        content: item.step,
        priority: PlanEntryPriority::Medium,
        status: match item.status {
            CodexStepStatus::Pending => PlanEntryStatus::Pending,
            CodexStepStatus::InProgress => PlanEntryStatus::InProgress,
            CodexStepStatus::Completed => PlanEntryStatus::Completed,
        },
        meta: None,
    }
}

fn available_command_from_tool(key: &str, tool: CodexToolDefinition) -> Option<AvailableCommand> {
    let name = key.to_string();
    let description = tool
        .annotations
        .as_ref()
        .and_then(|a| a.description.clone())
        .or(tool.description.clone())
        .or(tool.title.clone())
        .unwrap_or_else(|| name.clone());

    Some(AvailableCommand {
        name,
        description,
        input: None,
        meta: None,
    })
}

fn content_block_from_string(text: &str) -> ContentBlock {
    match serde_json::from_str::<Value>(text) {
        Ok(Value::Object(obj)) => {
            let value = Value::Object(obj);
            content_block_from_value(&value).unwrap_or_else(|| ContentBlock::from(text.to_string()))
        }
        _ => ContentBlock::from(text.to_string()),
    }
}

fn content_block_from_value(value: &Value) -> Option<ContentBlock> {
    let obj = value.as_object()?;
    let block_type = obj.get("type")?.as_str()?;
    match block_type {
        "text" => {
            let text = obj.get("text")?.as_str()?.to_string();
            Some(ContentBlock::Text(TextContent {
                annotations: None,
                text,
                meta: None,
            }))
        }
        "image" => {
            let data = obj.get("data")?.as_str()?.to_string();
            let mime_type = obj
                .get("mimeType")
                .and_then(|v| v.as_str())
                .unwrap_or("image/png")
                .to_string();
            let uri = obj.get("uri").and_then(|v| v.as_str()).map(String::from);
            Some(ContentBlock::Image(ImageContent {
                annotations: None,
                data,
                mime_type,
                uri,
                meta: None,
            }))
        }
        "audio" => {
            let data = obj.get("data")?.as_str()?.to_string();
            let mime_type = obj
                .get("mimeType")
                .and_then(|v| v.as_str())
                .unwrap_or("audio/mpeg")
                .to_string();
            Some(ContentBlock::Audio(AudioContent {
                annotations: None,
                data,
                mime_type,
                meta: None,
            }))
        }
        "resource_link" => {
            let uri = obj.get("uri")?.as_str()?.to_string();
            let name = obj
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(&uri)
                .to_string();
            let description = obj
                .get("description")
                .and_then(|v| v.as_str())
                .map(String::from);
            let mime_type = obj
                .get("mimeType")
                .and_then(|v| v.as_str())
                .map(String::from);
            let title = obj.get("title").and_then(|v| v.as_str()).map(String::from);
            let size = obj.get("size").and_then(|v| v.as_i64());
            Some(ContentBlock::ResourceLink(ResourceLink {
                annotations: None,
                description,
                mime_type,
                name,
                size,
                title,
                uri,
                meta: None,
            }))
        }
        "resource" => {
            let resource = obj.get("resource")?.as_object()?;
            let embedded = if let Some(text) = resource.get("text").and_then(|v| v.as_str()) {
                let uri = resource
                    .get("uri")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let mime_type = resource
                    .get("mimeType")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                EmbeddedResourceResource::TextResourceContents(TextResourceContents {
                    mime_type,
                    text: text.to_string(),
                    uri,
                    meta: None,
                })
            } else if let Some(blob) = resource.get("blob").and_then(|v| v.as_str()) {
                let uri = resource
                    .get("uri")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let mime_type = resource
                    .get("mimeType")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                EmbeddedResourceResource::BlobResourceContents(BlobResourceContents {
                    blob: blob.to_string(),
                    mime_type,
                    uri,
                    meta: None,
                })
            } else {
                return None;
            };

            Some(ContentBlock::Resource(EmbeddedResource {
                annotations: None,
                resource: embedded,
                meta: None,
            }))
        }
        _ => None,
    }
}

fn content_block_from_image_source(source: &str) -> Option<ContentBlock> {
    if let Some(image) = parse_data_uri_image(source) {
        return Some(ContentBlock::Image(image));
    }

    if source.starts_with("http://") || source.starts_with("https://") {
        return Some(ContentBlock::ResourceLink(ResourceLink {
            annotations: None,
            description: None,
            mime_type: None,
            name: source.to_string(),
            size: None,
            title: None,
            uri: source.to_string(),
            meta: None,
        }));
    }

    None
}

fn parse_data_uri_image(data_uri: &str) -> Option<ImageContent> {
    if !data_uri.starts_with("data:") {
        return None;
    }

    let mut parts = data_uri.splitn(2, ',');
    let header = parts.next()?;
    let data = parts.next()?.to_string();
    let mime_type = header
        .trim_start_matches("data:")
        .trim_end_matches(";base64")
        .to_string();

    Some(ImageContent {
        annotations: None,
        data,
        mime_type,
        uri: None,
        meta: None,
    })
}

fn acp_error_value(category: &str, message: &str, code: Option<&str>) -> Value {
    let numeric_code = match category {
        "timeout" => -32001,
        "permission_denied" => -32002,
        "not_found" => -32003,
        "cancelled" => -32004,
        "rate_limit" => -32005,
        _ => -32603,
    };

    let error = AcpError::new((numeric_code, message.to_string())).with_data(json!({
        "category": category,
        "codex_code": code,
    }));

    serde_json::to_value(error).unwrap_or_else(|_| {
        json!({
            "code": numeric_code,
            "message": message,
            "data": {
                "category": category,
                "codex_code": code
            }
        })
    })
}

/// Read and process Codex stdout
pub async fn stream_codex_output<R>(
    reader: R,
    session_id: SessionId,
    tx: mpsc::UnboundedSender<SessionNotification>,
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
            break;
        }

        if let Err(e) = manager.process_line(&line).await {
            error!("Error processing Codex output line {}: {}", line.trim(), e);
        }

        if manager.is_finalized() {
            debug!("Stream finalized by task_complete");
            break;
        }
    }

    Ok(())
}

/// Serialize a session notification to JSON line
pub fn serialize_update(update: &SessionNotification) -> Result<String> {
    serde_json::to_string(update).context("Failed to serialize session notification")
}
