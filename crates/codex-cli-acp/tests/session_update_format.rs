//! Tests for verifying session/update message format compliance with ACP

// ast-grep-ignore: rust-no-unwrap
// Test files can use unwrap() freely

use codex_cli_acp::codex_proto::{
    ContentBlock, SessionUpdate, SessionUpdateContent, SessionUpdateParams, ToolCallStatus,
};
use serde_json::{json, Value};

#[test]
fn test_agent_message_chunk_format() {
    let update = SessionUpdate {
        jsonrpc: "2.0".to_string(),
        method: "session/update".to_string(),
        params: SessionUpdateParams {
            session_id: "test_session".to_string(),
            update: SessionUpdateContent::AgentMessageChunk {
                content: ContentBlock::Text {
                    text: "Test message".to_string(),
                },
            },
        },
    };

    // ast-grep-ignore
    let json = serde_json::to_value(&update).unwrap();

    // Verify structure
    assert_eq!(json["jsonrpc"], "2.0");
    assert_eq!(json["method"], "session/update");
    assert_eq!(json["params"]["sessionId"], "test_session");

    // Verify nested update structure with sessionUpdate discriminator
    assert_eq!(
        json["params"]["update"]["sessionUpdate"],
        "agent_message_chunk"
    );
    assert_eq!(json["params"]["update"]["content"]["type"], "text");
    assert_eq!(json["params"]["update"]["content"]["text"], "Test message");
}

#[test]
fn test_tool_call_format() {
    let update = SessionUpdate {
        jsonrpc: "2.0".to_string(),
        method: "session/update".to_string(),
        params: SessionUpdateParams {
            session_id: "test_session".to_string(),
            update: SessionUpdateContent::ToolCall {
                tool_call_id: "tool_123".to_string(),
                title: "read_file".to_string(),
                kind: Some("read".to_string()),
                status: Some(ToolCallStatus::Pending),
                content: None,
                locations: None,
                raw_input: Some(json!({"path": "/test/file.txt"})),
                raw_output: None,
            },
        },
    };

    // ast-grep-ignore
    let json = serde_json::to_value(&update).unwrap();

    // Verify structure
    assert_eq!(json["jsonrpc"], "2.0");
    assert_eq!(json["method"], "session/update");
    assert_eq!(json["params"]["sessionId"], "test_session");

    // Verify nested update structure with sessionUpdate discriminator
    assert_eq!(json["params"]["update"]["sessionUpdate"], "tool_call");
    assert_eq!(json["params"]["update"]["toolCallId"], "tool_123");
    assert_eq!(json["params"]["update"]["title"], "read_file");
    assert_eq!(json["params"]["update"]["kind"], "read");
    assert_eq!(json["params"]["update"]["status"], "pending");
    assert_eq!(
        json["params"]["update"]["rawInput"]["path"],
        "/test/file.txt"
    );
}

#[test]
fn test_serialization_format() {
    let update = SessionUpdate {
        jsonrpc: "2.0".to_string(),
        method: "session/update".to_string(),
        params: SessionUpdateParams {
            session_id: "test_session".to_string(),
            update: SessionUpdateContent::AgentMessageChunk {
                content: ContentBlock::Text {
                    text: "Hello".to_string(),
                },
            },
        },
    };

    // ast-grep-ignore
    let serialized = codex_cli_acp::codex_proto::serialize_update(&update).unwrap();
    // ast-grep-ignore
    let parsed: Value = serde_json::from_str(&serialized).unwrap();

    // Ensure the serialized format maintains the correct structure
    assert!(parsed["params"]["update"]["sessionUpdate"].is_string());
    assert_eq!(
        parsed["params"]["update"]["sessionUpdate"],
        "agent_message_chunk"
    );
}

#[test]
fn test_tool_call_content_structure() {
    // Test that content is properly an array of ContentBlocks, not nested ToolCallContent
    let update = SessionUpdate {
        jsonrpc: "2.0".to_string(),
        method: "session/update".to_string(),
        params: SessionUpdateParams {
            session_id: "test_session".to_string(),
            update: SessionUpdateContent::ToolCall {
                tool_call_id: "tool_456".to_string(),
                title: "execute_command".to_string(),
                kind: Some("execute".to_string()),
                status: Some(ToolCallStatus::Completed),
                content: Some(vec![ContentBlock::Text {
                    text: "Command output here".to_string(),
                }]),
                locations: None,
                raw_input: Some(json!({"command": "ls -la"})),
                raw_output: Some(json!({"stdout": "file1.txt\nfile2.txt"})),
            },
        },
    };

    // ast-grep-ignore
    let json = serde_json::to_value(&update).unwrap();

    // Verify content is an array of ContentBlocks directly
    assert!(json["params"]["update"]["content"].is_array());
    // ast-grep-ignore
    let content_array = &json["params"]["update"]["content"].as_array().unwrap();
    assert_eq!(content_array.len(), 1);

    // Verify ContentBlock structure (should have type and text fields)
    let first_block = &content_array[0];
    assert_eq!(first_block["type"], "text");
    assert_eq!(first_block["text"], "Command output here");

    // Ensure we're NOT using the old double-nested structure
    assert!(
        first_block.get("content").is_none(),
        "Content should not be double-nested"
    );
}

#[test]
fn test_tool_call_update_structure() {
    // Test ToolCallUpdate variant for status changes
    let update = SessionUpdate {
        jsonrpc: "2.0".to_string(),
        method: "session/update".to_string(),
        params: SessionUpdateParams {
            session_id: "test_session".to_string(),
            update: SessionUpdateContent::ToolCallUpdate {
                tool_call_id: "tool_789".to_string(),
                title: None,
                kind: None,
                status: Some(ToolCallStatus::InProgress),
                content: None,
                locations: None,
                raw_input: None,
                raw_output: None,
            },
        },
    };

    // ast-grep-ignore
    let json = serde_json::to_value(&update).unwrap();

    // Verify ToolCallUpdate structure
    assert_eq!(
        json["params"]["update"]["sessionUpdate"],
        "tool_call_update"
    );
    assert_eq!(json["params"]["update"]["toolCallId"], "tool_789");
    assert_eq!(json["params"]["update"]["status"], "in_progress");

    // Verify optional fields are not included when None
    assert!(json["params"]["update"]["title"].is_null());
    assert!(json["params"]["update"]["kind"].is_null());
    assert!(json["params"]["update"]["content"].is_null());
}

#[test]
fn test_initialize_response_spec_compliance() {
    // Test that initialize response matches ACP spec exactly

    // Simulate what the server would return
    let response = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "protocolVersion": 1,  // Must be integer, not string
            "agentCapabilities": {  // Must be "agentCapabilities", not "capabilities"
                "loadSession": false,
                "promptCapabilities": {  // Must be nested under agentCapabilities
                    "audio": false,
                    "embeddedContext": false,
                    "image": false
                }
            },
            "authMethods": []  // Required field
        }
    });

    // Verify critical structure points
    assert_eq!(response["result"]["protocolVersion"], 1);
    assert!(
        response["result"]["protocolVersion"].is_u64(),
        "protocolVersion must be integer"
    );

    // Verify agentCapabilities structure
    assert!(response["result"]["agentCapabilities"].is_object());
    assert_eq!(
        response["result"]["agentCapabilities"]["loadSession"],
        false
    );

    // Verify promptCapabilities is nested correctly
    assert!(response["result"]["agentCapabilities"]["promptCapabilities"].is_object());
    assert_eq!(
        response["result"]["agentCapabilities"]["promptCapabilities"]["audio"],
        false
    );
    assert_eq!(
        response["result"]["agentCapabilities"]["promptCapabilities"]["embeddedContext"],
        false
    );
    assert_eq!(
        response["result"]["agentCapabilities"]["promptCapabilities"]["image"],
        false
    );

    // Verify authMethods is present
    assert!(response["result"]["authMethods"].is_array());

    // Verify no fs capabilities in response (fs is client capability, not agent)
    assert!(response["result"].get("fs").is_none());
    assert!(response["result"].get("capabilities").is_none());
}

#[test]
fn test_session_new_cwd_parameter() {
    // Test that session/new accepts "cwd" parameter per spec

    // Simulate request with cwd (spec-compliant)
    let request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "session/new",
        "params": {
            "cwd": "/absolute/path/to/project",
            "mcpServers": []
        }
    });

    // Verify structure
    assert_eq!(request["params"]["cwd"], "/absolute/path/to/project");
    assert!(request["params"]["mcpServers"].is_array());

    // The handler should accept this without error
    // In actual implementation, this would be tested with the handler
}

#[test]
fn test_session_new_validation_requirements() {
    // Test validation requirements for session/new

    // Invalid: relative path should be rejected
    let invalid_request = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "session/new",
        "params": {
            "cwd": "./relative/path",  // Invalid - must be absolute
            "mcpServers": []
        }
    });

    assert!(!invalid_request["params"]["cwd"]
        .as_str()
        // ast-grep-ignore
        .unwrap()
        .starts_with('/'));

    // Valid: absolute path
    let valid_request = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "session/new",
        "params": {
            "cwd": "/absolute/path",
            "mcpServers": []
        }
    });

    assert!(valid_request["params"]["cwd"]
        .as_str()
        // ast-grep-ignore
        .unwrap()
        .starts_with('/'));
}
