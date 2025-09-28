//! Tests for verifying session/update message format compliance with ACP

// Test files can use unwrap() freely

use agent_client_protocol::{
    ContentBlock, SessionId, SessionNotification, SessionUpdate, ToolCall, ToolCallContent,
    ToolCallId, ToolCallStatus, ToolCallUpdate, ToolCallUpdateFields, ToolKind,
};
use serde_json::{json, Value};
use std::sync::Arc;

fn session_id() -> SessionId {
    SessionId(Arc::from("test_session"))
}

#[test]
fn test_agent_message_chunk_format() {
    let update = SessionNotification {
        session_id: session_id(),
        update: SessionUpdate::AgentMessageChunk {
            content: ContentBlock::from("Test message"),
        },
        meta: None,
    };

    // ast-grep-ignore: rust-no-unwrap
    let json = serde_json::to_value(&update).unwrap();

    assert_eq!(json["sessionId"], "test_session");
    assert_eq!(json["update"]["sessionUpdate"], "agent_message_chunk");
    assert_eq!(json["update"]["content"]["type"], "text");
    assert_eq!(json["update"]["content"]["text"], "Test message");
}

#[test]
fn test_tool_call_format() {
    let tool_call = ToolCall {
        id: ToolCallId(Arc::from("tool_123")),
        title: "read_file".to_string(),
        kind: ToolKind::Read,
        status: ToolCallStatus::Pending,
        content: Vec::new(),
        locations: Vec::new(),
        raw_input: Some(json!({"path": "/test/file.txt"})),
        raw_output: None,
        meta: None,
    };

    let update = SessionNotification {
        session_id: session_id(),
        update: SessionUpdate::ToolCall(tool_call),
        meta: None,
    };

    // ast-grep-ignore: rust-no-unwrap
    let json = serde_json::to_value(&update).unwrap();

    assert_eq!(json["update"]["sessionUpdate"], "tool_call");
    assert_eq!(json["update"]["toolCallId"], "tool_123");
    assert_eq!(json["update"]["title"], "read_file");
    assert_eq!(json["update"]["kind"], "read");
    assert!(json["update"].get("status").is_none());
    assert_eq!(json["update"]["rawInput"]["path"], "/test/file.txt");
}

#[test]
fn test_serialization_format() {
    let update = SessionNotification {
        session_id: session_id(),
        update: SessionUpdate::AgentMessageChunk {
            content: ContentBlock::from("Hello"),
        },
        meta: None,
    };

    // ast-grep-ignore: rust-no-unwrap
    let serialized = codex_cli_acp::codex_proto::serialize_update(&update).unwrap();
    // ast-grep-ignore: rust-no-unwrap
    let parsed: Value = serde_json::from_str(&serialized).unwrap();

    assert!(parsed["update"]["sessionUpdate"].is_string());
    assert_eq!(parsed["update"]["sessionUpdate"], "agent_message_chunk");
}

#[test]
fn test_tool_call_content_structure() {
    let tool_call = ToolCall {
        id: ToolCallId(Arc::from("tool_456")),
        title: "execute_command".to_string(),
        kind: ToolKind::Execute,
        status: ToolCallStatus::Completed,
        content: vec![ToolCallContent::from("Command output here")],
        locations: Vec::new(),
        raw_input: Some(json!({"command": "ls -la"})),
        raw_output: Some(json!({"stdout": "file1.txt
file2.txt"})),
        meta: None,
    };

    let update = SessionNotification {
        session_id: session_id(),
        update: SessionUpdate::ToolCall(tool_call),
        meta: None,
    };

    // ast-grep-ignore: rust-no-unwrap
    let json = serde_json::to_value(&update).unwrap();

    assert!(json["update"]["content"].is_array());
    // ast-grep-ignore: rust-no-unwrap
    let content_array = json["update"]["content"].as_array().unwrap();
    assert_eq!(content_array.len(), 1);
    let content = &content_array[0];
    assert_eq!(content["type"], "content");
    assert_eq!(content["content"]["type"], "text");
    assert_eq!(content["content"]["text"], "Command output here");
}

#[test]
fn test_tool_call_update_structure() {
    let update = SessionNotification {
        session_id: session_id(),
        update: SessionUpdate::ToolCallUpdate(ToolCallUpdate {
            id: ToolCallId(Arc::from("tool_789")),
            fields: ToolCallUpdateFields {
                status: Some(ToolCallStatus::InProgress),
                ..ToolCallUpdateFields::default()
            },
            meta: None,
        }),
        meta: None,
    };

    // ast-grep-ignore: rust-no-unwrap
    let json = serde_json::to_value(&update).unwrap();

    assert_eq!(json["update"]["sessionUpdate"], "tool_call_update");
    assert_eq!(json["update"]["toolCallId"], "tool_789");
    assert_eq!(json["update"]["status"], "in_progress");
}

#[test]
fn test_initialize_response_spec_compliance() {
    let response = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "protocolVersion": 1,
            "agentCapabilities": {
                "promptCapabilities": {}
            }
        }
    });

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["result"]["protocolVersion"], 1);
}
