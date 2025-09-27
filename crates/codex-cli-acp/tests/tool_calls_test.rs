//! Integration tests for tool call functionality

// Test files can use unwrap() freely

use agent_client_protocol::{
    SessionId, SessionNotification, SessionUpdate, ToolCallStatus, ToolKind,
};
use codex_cli_acp::codex_proto::{CodexEvent, CodexStreamManager, ToolCallItem};
use codex_cli_acp::tool_calls::{
    extract_shell_command, format_tool_output, map_tool_kind, MAX_OUTPUT_PREVIEW_BYTES,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::mpsc;

fn session_id(value: &str) -> SessionId {
    SessionId(Arc::from(value))
}

fn expected_tool_kind(tag: &str) -> ToolKind {
    match tag {
        "read" => ToolKind::Read,
        "edit" => ToolKind::Edit,
        "delete" => ToolKind::Delete,
        "move" => ToolKind::Move,
        "search" => ToolKind::Search,
        "execute" => ToolKind::Execute,
        "think" => ToolKind::Think,
        "fetch" => ToolKind::Fetch,
        "switch_mode" => ToolKind::SwitchMode,
        _ => ToolKind::Other,
    }
}

#[tokio::test]
async fn test_single_tool_call_progression() {
    let (tx, mut rx) = mpsc::unbounded_channel::<SessionNotification>();
    let mut manager = CodexStreamManager::new(session_id("test_session"), tx);

    let event = CodexEvent::ToolCall {
        id: "tool_001".to_string(),
        name: "read_file".to_string(),
        arguments: json!({"path": "/test/file.txt"}),
        status: Some("pending".to_string()),
        output: None,
        error: None,
    };
    // ast-grep-ignore: rust-no-unwrap
    let event_json = serde_json::to_string(&event).unwrap();
    // ast-grep-ignore: rust-no-unwrap
    manager.process_line(&event_json).await.unwrap();

    // ast-grep-ignore: rust-no-unwrap
    let notification = rx.recv().await.unwrap();
    match notification.update {
        SessionUpdate::ToolCall(tool_call) => {
            assert_eq!(tool_call.id.0.as_ref(), "tool_001");
            assert_eq!(tool_call.title, "read_file");
            assert_eq!(tool_call.kind, ToolKind::Read);
            assert_eq!(tool_call.status, ToolCallStatus::Pending);
            assert!(tool_call.raw_input.is_some());
        }
        other => panic!("Expected ToolCall update, got {:?}", other),
    }

    let event = CodexEvent::ToolCall {
        id: "tool_001".to_string(),
        name: "read_file".to_string(),
        arguments: json!({"path": "/test/file.txt"}),
        status: Some("in_progress".to_string()),
        output: None,
        error: None,
    };
    // ast-grep-ignore: rust-no-unwrap
    let event_json = serde_json::to_string(&event).unwrap();
    // ast-grep-ignore: rust-no-unwrap
    manager.process_line(&event_json).await.unwrap();

    // ast-grep-ignore: rust-no-unwrap
    let notification = rx.recv().await.unwrap();
    match notification.update {
        SessionUpdate::ToolCallUpdate(update) => {
            assert_eq!(update.id.0.as_ref(), "tool_001");
            assert_eq!(update.fields.status, Some(ToolCallStatus::InProgress));
        }
        other => panic!("Expected ToolCallUpdate, got {:?}", other),
    }

    let event = CodexEvent::ToolCall {
        id: "tool_001".to_string(),
        name: "read_file".to_string(),
        arguments: json!({"path": "/test/file.txt"}),
        status: Some("completed".to_string()),
        output: Some(json!({"content": "File contents here"})),
        error: None,
    };
    // ast-grep-ignore: rust-no-unwrap
    let event_json = serde_json::to_string(&event).unwrap();
    // ast-grep-ignore: rust-no-unwrap
    manager.process_line(&event_json).await.unwrap();

    // ast-grep-ignore: rust-no-unwrap
    let notification = rx.recv().await.unwrap();
    match notification.update {
        SessionUpdate::ToolCallUpdate(update) => {
            assert_eq!(update.id.0.as_ref(), "tool_001");
            assert_eq!(update.fields.status, Some(ToolCallStatus::Completed));
            assert!(update
                .fields
                .raw_output
                .as_ref()
                .and_then(|value| value.get("content"))
                .is_some());
            assert!(update.fields.raw_input.is_none());
        }
        other => panic!("Expected ToolCallUpdate, got {:?}", other),
    }
}

#[tokio::test]
async fn test_batch_tool_calls() {
    let (tx, mut rx) = mpsc::unbounded_channel::<SessionNotification>();
    let mut manager = CodexStreamManager::new(session_id("tool-call-session"), tx);

    let event = CodexEvent::ToolCalls {
        calls: vec![
            ToolCallItem {
                id: "tool_a".to_string(),
                name: "read_file".to_string(),
                arguments: json!({"path": "."}),
                status: Some("pending".to_string()),
                output: None,
                error: None,
            },
            ToolCallItem {
                id: "tool_b".to_string(),
                name: "write_file".to_string(),
                arguments: json!({"path": "./out.txt", "content": "data"}),
                status: Some("pending".to_string()),
                output: None,
                error: None,
            },
        ],
    };
    // ast-grep-ignore: rust-no-unwrap
    let event_json = serde_json::to_string(&event).unwrap();
    // ast-grep-ignore: rust-no-unwrap
    manager.process_line(&event_json).await.unwrap();

    // ast-grep-ignore: rust-no-unwrap
    let first = rx.recv().await.unwrap();
    // ast-grep-ignore: rust-no-unwrap
    let second = rx.recv().await.unwrap();
    let ids: Vec<_> = [first, second]
        .into_iter()
        .map(|notification| match notification.update {
            SessionUpdate::ToolCall(tool_call) => tool_call.id.0.as_ref().to_string(),
            other => panic!("Expected ToolCall, got {:?}", other),
        })
        .collect();

    assert_eq!(ids, vec!["tool_a", "tool_b"]);
}

#[test]
fn test_tool_kind_mapping_comprehensive() {
    let test_cases = vec![
        ("read_file", "read"),
        ("write_file", "edit"),
        ("delete_file", "delete"),
        ("move_file", "move"),
        ("search_code", "search"),
        ("execute_command", "execute"),
        ("think_about_problem", "think"),
        ("fetch_url", "fetch"),
        ("switch_mode", "switch_mode"),
        ("unknown_tool", "other"),
    ];

    for (tool_name, expected_kind) in test_cases {
        assert_eq!(
            map_tool_kind(tool_name),
            expected_tool_kind(expected_kind),
            "Tool {} should map to {}",
            tool_name,
            expected_kind
        );
    }
}

#[test]
fn test_shell_command_extraction() {
    assert_eq!(
        extract_shell_command("local_shell", &json!({"command": "ls -la"})),
        Some("ls -la".to_string())
    );

    assert_eq!(
        extract_shell_command("exec", &json!({"cmd": "pwd"})),
        Some("pwd".to_string())
    );

    assert_eq!(
        extract_shell_command("run_bash", &json!({"script": "echo hello"})),
        Some("echo hello".to_string())
    );

    assert_eq!(
        extract_shell_command("python_shell", &json!({"code": "print(test)"})),
        Some("print(test)".to_string())
    );

    assert_eq!(
        extract_shell_command("read_file", &json!({"path": "/file.txt"})),
        None
    );
}

#[test]
fn test_output_truncation() {
    let long_output = "a".repeat(MAX_OUTPUT_PREVIEW_BYTES * 2);
    let truncated = format_tool_output("example", &json!({"output": long_output}), 64);
    assert!(truncated.contains("[truncated"));
}

#[test]
fn test_shell_params_extraction() {
    use codex_cli_acp::tool_calls::extract_shell_params;

    let params = extract_shell_params(
        "local_shell",
        &json!({
            "command": "ls",
            "cwd": "/tmp",
            "timeout_ms": 1_000,
            "with_escalated_permissions": true,
            "justification": "test"
        }),
    );

    assert_eq!(params.command.as_deref(), Some("ls"));
    assert_eq!(params.workdir.as_deref(), Some("/tmp"));
    assert_eq!(params.timeout_ms, Some(1_000));
    assert_eq!(params.with_escalated_permissions, Some(true));
    assert_eq!(params.justification.as_deref(), Some("test"));
}
