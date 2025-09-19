//! Integration tests for tool call functionality

// ast-grep-ignore: rust-no-unwrap
// Test files can use unwrap() freely

use codex_cli_acp::codex_proto::{
    CodexEvent, CodexStreamManager, SessionUpdate, SessionUpdateContent, ToolCallItem,
    ToolCallStatus,
};
use codex_cli_acp::tool_calls::{
    extract_shell_command, format_tool_output, map_tool_kind, MAX_OUTPUT_PREVIEW_BYTES,
};
use serde_json::json;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_single_tool_call_progression() {
    let (tx, mut rx) = mpsc::unbounded_channel::<SessionUpdate>();
    let mut manager = CodexStreamManager::new("test_session".to_string(), tx);

    // Simulate a tool call with pending status
    let event = CodexEvent::ToolCall {
        id: "tool_001".to_string(),
        name: "read_file".to_string(),
        arguments: json!({"path": "/test/file.txt"}),
        status: Some("pending".to_string()),
        output: None,
        error: None,
    };

    // Process the pending event
    // ast-grep-ignore
    let event_json = serde_json::to_string(&event).unwrap();
    // ast-grep-ignore
    manager.process_line(&event_json).await.unwrap();

    // Check we got a ToolCall with pending status
    // ast-grep-ignore
    let update = rx.recv().await.unwrap();
    match &update.params.update {
        SessionUpdateContent::ToolCall {
            tool_call_id,
            title,
            kind,
            status,
            ..
        } => {
            assert_eq!(tool_call_id, "tool_001");
            assert_eq!(title, "read_file");
            assert_eq!(kind.as_deref(), Some("read"));
            assert_eq!(*status, Some(ToolCallStatus::Pending));
        }
        _ => panic!("Expected ToolCall"),
    }

    // Simulate in_progress status
    let event = CodexEvent::ToolCall {
        id: "tool_001".to_string(),
        name: "read_file".to_string(),
        arguments: json!({"path": "/test/file.txt"}),
        status: Some("in_progress".to_string()),
        output: None,
        error: None,
    };

    // ast-grep-ignore
    let event_json = serde_json::to_string(&event).unwrap();
    // ast-grep-ignore
    manager.process_line(&event_json).await.unwrap();

    // Check we got a ToolCallUpdate with in_progress status
    // ast-grep-ignore
    let update = rx.recv().await.unwrap();
    match &update.params.update {
        SessionUpdateContent::ToolCallUpdate {
            tool_call_id,
            status,
            ..
        } => {
            assert_eq!(tool_call_id, "tool_001");
            assert_eq!(*status, Some(ToolCallStatus::InProgress));
        }
        _ => panic!("Expected ToolCallUpdate"),
    }

    // Simulate completed status with output
    let event = CodexEvent::ToolCall {
        id: "tool_001".to_string(),
        name: "read_file".to_string(),
        arguments: json!({"path": "/test/file.txt"}),
        status: Some("completed".to_string()),
        output: Some(json!({"content": "File contents here"})),
        error: None,
    };

    // ast-grep-ignore
    let event_json = serde_json::to_string(&event).unwrap();
    // ast-grep-ignore
    manager.process_line(&event_json).await.unwrap();

    // Check we got a ToolCallUpdate with completed status and output
    // ast-grep-ignore
    let update = rx.recv().await.unwrap();
    match &update.params.update {
        SessionUpdateContent::ToolCallUpdate {
            tool_call_id,
            status,
            content,
            raw_output,
            ..
        } => {
            assert_eq!(tool_call_id, "tool_001");
            assert_eq!(*status, Some(ToolCallStatus::Completed));
            assert!(content.is_some());
            assert!(raw_output.is_some());
        }
        _ => panic!("Expected ToolCallUpdate"),
    }
}

#[tokio::test]
async fn test_batch_tool_calls() {
    let (tx, mut rx) = mpsc::unbounded_channel::<SessionUpdate>();
    let mut manager = CodexStreamManager::new("test_session".to_string(), tx);

    // Simulate batch tool calls
    let event = CodexEvent::ToolCalls {
        calls: vec![
            ToolCallItem {
                id: "tool_001".to_string(),
                name: "read_file".to_string(),
                arguments: json!({"path": "/file1.txt"}),
                status: Some("pending".to_string()),
                output: None,
                error: None,
            },
            ToolCallItem {
                id: "tool_002".to_string(),
                name: "write_file".to_string(),
                arguments: json!({"path": "/file2.txt", "content": "data"}),
                status: Some("pending".to_string()),
                output: None,
                error: None,
            },
            ToolCallItem {
                id: "tool_003".to_string(),
                name: "local_shell".to_string(),
                arguments: json!({"command": "ls -la"}),
                status: Some("pending".to_string()),
                output: None,
                error: None,
            },
        ],
    };

    // ast-grep-ignore
    let event_json = serde_json::to_string(&event).unwrap();
    // ast-grep-ignore
    manager.process_line(&event_json).await.unwrap();

    // Check we got 3 ToolCall events
    for i in 0..3 {
        // ast-grep-ignore
        let update = rx.recv().await.unwrap();
        match &update.params.update {
            SessionUpdateContent::ToolCall {
                tool_call_id,
                kind,
                status,
                ..
            } => {
                assert!(tool_call_id.starts_with("tool_"));
                assert_eq!(*status, Some(ToolCallStatus::Pending));

                // Check correct kind mapping
                if i == 0 {
                    assert_eq!(kind.as_deref(), Some("read"));
                } else if i == 1 {
                    assert_eq!(kind.as_deref(), Some("edit"));
                } else {
                    assert_eq!(kind.as_deref(), Some("execute"));
                }
            }
            _ => panic!("Expected ToolCall"),
        }
    }
}

#[tokio::test]
async fn test_shell_command_title() {
    let (tx, mut rx) = mpsc::unbounded_channel::<SessionUpdate>();
    let mut manager = CodexStreamManager::new("test_session".to_string(), tx);

    // Simulate a shell command tool call
    let event = CodexEvent::ToolCall {
        id: "tool_shell".to_string(),
        name: "local_shell".to_string(),
        arguments: json!({"command": "git status --porcelain"}),
        status: Some("pending".to_string()),
        output: None,
        error: None,
    };

    // ast-grep-ignore
    let event_json = serde_json::to_string(&event).unwrap();
    // ast-grep-ignore
    manager.process_line(&event_json).await.unwrap();

    // Check the title includes the command
    // ast-grep-ignore
    let update = rx.recv().await.unwrap();
    match &update.params.update {
        SessionUpdateContent::ToolCall { title, kind, .. } => {
            assert!(title.contains("git status --porcelain"));
            assert_eq!(kind.as_deref(), Some("execute"));
        }
        _ => panic!("Expected ToolCall"),
    }
}

#[tokio::test]
async fn test_tool_output_truncation() {
    let (tx, mut rx) = mpsc::unbounded_channel::<SessionUpdate>();
    let mut manager = CodexStreamManager::new("test_session".to_string(), tx);

    // Create large output that exceeds 2KB
    let large_output = "a".repeat(5000);

    // Simulate completed tool call with large output
    let event = CodexEvent::ToolCall {
        id: "tool_large".to_string(),
        name: "local_shell".to_string(),
        arguments: json!({"command": "cat largefile.txt"}),
        status: Some("completed".to_string()),
        output: Some(json!({
            "stdout": large_output.clone(),
            "exit_code": 0
        })),
        error: None,
    };

    // ast-grep-ignore
    let event_json = serde_json::to_string(&event).unwrap();
    // ast-grep-ignore
    manager.process_line(&event_json).await.unwrap();

    // Check the output is truncated in content but full in raw_output
    // ast-grep-ignore
    let update = rx.recv().await.unwrap();
    match &update.params.update {
        SessionUpdateContent::ToolCall {
            content,
            raw_output,
            ..
        } => {
            // Content should be truncated
            // ast-grep-ignore
            let content_text = &content.as_ref().unwrap()[0];
            let codex_cli_acp::codex_proto::ContentBlock::Text { text } = content_text;
            assert!(text.len() <= MAX_OUTPUT_PREVIEW_BYTES + 100); // Allow for truncation marker
            assert!(text.contains("[truncated"));
            assert!(text.starts_with("aaa"));
            assert!(text.ends_with("[exit code: 0]") || text.contains("aaa"));

            // Raw output should have full content
            // ast-grep-ignore
            let raw = raw_output.as_ref().unwrap();
            assert_eq!(raw["stdout"], large_output);
        }
        _ => panic!("Expected ToolCall"),
    }
}

#[tokio::test]
async fn test_tool_error_handling() {
    let (tx, mut rx) = mpsc::unbounded_channel::<SessionUpdate>();
    let mut manager = CodexStreamManager::new("test_session".to_string(), tx);

    // Simulate failed tool call with error
    let event = CodexEvent::ToolCall {
        id: "tool_error".to_string(),
        name: "write_file".to_string(),
        arguments: json!({"path": "/readonly/file.txt", "content": "data"}),
        status: Some("failed".to_string()),
        output: None,
        error: Some("Permission denied: cannot write to /readonly/file.txt".to_string()),
    };

    // ast-grep-ignore
    let event_json = serde_json::to_string(&event).unwrap();
    // ast-grep-ignore
    manager.process_line(&event_json).await.unwrap();

    // Check error is included in content
    // ast-grep-ignore
    let update = rx.recv().await.unwrap();
    match &update.params.update {
        SessionUpdateContent::ToolCall {
            status,
            content,
            raw_output,
            ..
        } => {
            assert_eq!(*status, Some(ToolCallStatus::Failed));

            // Content should include error message
            // ast-grep-ignore
            let content_text = &content.as_ref().unwrap()[0];
            let codex_cli_acp::codex_proto::ContentBlock::Text { text } = content_text;
            assert!(text.contains("Permission denied"));

            // Raw output should indicate failure
            // ast-grep-ignore
            let raw = raw_output.as_ref().unwrap();
            assert_eq!(raw["status"], "failed");
            // ast-grep-ignore
            assert!(raw["error"].as_str().unwrap().contains("Permission denied"));
        }
        _ => panic!("Expected ToolCall"),
    }
}

#[test]
fn test_tool_kind_mapping_comprehensive() {
    // Test all categories thoroughly
    let test_cases = vec![
        // Read operations
        ("read_file", "read"),
        ("get_content", "read"),
        ("fetch_file_data", "read"),
        ("cat", "read"),
        ("list_files", "read"),
        ("view_source", "read"),
        // Edit operations
        ("write_file", "edit"),
        ("edit_config", "edit"),
        ("update_settings", "edit"),
        ("modify_data", "edit"),
        ("patch_file", "edit"),
        ("change_value", "edit"),
        ("set_property", "edit"),
        // Delete operations
        ("delete_file", "delete"),
        ("remove_item", "delete"),
        ("rm", "delete"),
        ("rmdir", "delete"),
        // Move operations
        ("move_file", "move"),
        ("rename_folder", "move"),
        ("mv", "move"),
        // Search operations
        ("search_code", "search"),
        ("find_pattern", "search"),
        ("grep_files", "search"),
        ("locate_item", "search"),
        ("query_database", "search"),
        // Execute operations
        ("execute_command", "execute"),
        ("run_script", "execute"),
        ("local_shell", "execute"),
        ("bash", "execute"),
        ("python_exec", "execute"),
        ("cmd_run", "execute"),
        // Think operations
        ("think_about_problem", "think"),
        ("reason_through", "think"),
        ("plan_solution", "think"),
        ("analyze_code", "think"),
        ("consider_options", "think"),
        // Fetch operations
        ("fetch_url", "fetch"),
        ("download_file", "fetch"),
        ("curl_request", "fetch"),
        ("wget_resource", "fetch"),
        ("http_get", "fetch"),
        // Other operations
        ("custom_tool", "other"),
        ("unknown_operation", "other"),
        ("special_function", "other"),
    ];

    for (tool_name, expected_kind) in test_cases {
        assert_eq!(
            map_tool_kind(tool_name),
            expected_kind,
            "Tool '{}' should map to '{}'",
            tool_name,
            expected_kind
        );
    }
}

#[test]
fn test_shell_command_extraction() {
    // Test various argument patterns
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
        extract_shell_command("python_shell", &json!({"code": "print('test')"})),
        Some("print('test')".to_string())
    );

    // Non-shell tools should return None
    assert_eq!(
        extract_shell_command("read_file", &json!({"path": "/file.txt"})),
        None
    );
}

#[test]
fn test_output_formatting() {
    // Test string output
    let output = json!("Simple output");
    let formatted = format_tool_output("any_tool", &output, 100);
    assert_eq!(formatted, "Simple output");

    // Test structured output with stdout/stderr
    let output = json!({
        "stdout": "Command successful",
        "stderr": "Warning: deprecated",
        "exit_code": 0
    });
    let formatted = format_tool_output("shell", &output, 200);
    assert!(formatted.contains("Command successful"));
    assert!(formatted.contains("[stderr]"));
    assert!(formatted.contains("Warning: deprecated"));

    // Test array output
    let output = json!(["item1", "item2", "item3"]);
    let formatted = format_tool_output("list", &output, 100);
    assert!(formatted.contains("item1\nitem2\nitem3"));

    // Test truncation of large output
    let large = "x".repeat(3000);
    let output = json!(large);
    let formatted = format_tool_output("tool", &output, MAX_OUTPUT_PREVIEW_BYTES);
    assert!(formatted.len() <= MAX_OUTPUT_PREVIEW_BYTES + 100);
    assert!(formatted.contains("[truncated"));
}
// formatting: ensure trailing newline
