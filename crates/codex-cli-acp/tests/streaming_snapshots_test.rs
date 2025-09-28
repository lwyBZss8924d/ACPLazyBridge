//! Snapshot scaffolding for Codex streaming updates.
//!
//! Phase 3.2 populates these tests with RED expectations that enforce the
//! agent_client_protocol v0.4.3 schema. Implementation work must update the
//! streaming mapper so these tests pass.

#[path = "support/mod.rs"]
mod support;

use agent_client_protocol::SessionNotification;
use codex_cli_acp::codex_proto::CodexEvent;
use insta::assert_json_snapshot;
use serde_json::{json, Value};
use support::SnapshotHarness;

fn parse_notification(value: &Value) -> SessionNotification {
    // ast-grep-ignore: rust-no-unwrap
    serde_json::from_value(value.clone())
        .expect("SessionNotification should already match agent_client_protocol v0.4.3 schema")
}

#[tokio::test]
async fn harness_emits_updates_for_agent_messages() {
    let mut harness = SnapshotHarness::new("test_session");
    let event = CodexEvent::AgentMessage {
        message: "hello".to_string(),
        _timestamp: None,
    };

    // ast-grep-ignore: rust-no-unwrap
    harness.ingest_event(event).await.unwrap();
    let updates = harness.drain_json();

    assert_eq!(updates.len(), 1);
    // The detailed insta-based assertions are part of Phase 3.2; for now we
    // simply verify the harness wires the stream manager to the receiver.
    assert_eq!(updates[0]["sessionId"].as_str(), Some("test_session"));
    assert_eq!(
        updates[0]["update"]["sessionUpdate"].as_str(),
        Some("agent_message_chunk")
    );
}

#[tokio::test]
async fn session_update_variants_deserialize_into_official_schema() {
    let mut harness = SnapshotHarness::new("variant-session");

    // ast-grep-ignore: rust-no-unwrap
    harness
        .ingest_event(CodexEvent::AgentMessage {
            message: "stream chunk".to_string(),
            _timestamp: None,
        })
        .await
        .unwrap();

    // ast-grep-ignore: rust-no-unwrap
    harness
        .ingest_event(CodexEvent::ToolCall {
            id: "tool_001".to_string(),
            name: "read_file".to_string(),
            arguments: json!({"path": "/tmp/file.txt"}),
            status: Some("pending".to_string()),
            output: None,
            error: None,
        })
        .await
        .unwrap();

    // ast-grep-ignore: rust-no-unwrap
    harness
        .ingest_event(CodexEvent::ToolCall {
            id: "tool_001".to_string(),
            name: "read_file".to_string(),
            arguments: json!({"path": "/tmp/file.txt"}),
            status: Some("completed".to_string()),
            output: Some(json!({"content": "done"})),
            error: None,
        })
        .await
        .unwrap();

    let updates = harness.drain_json();
    assert_eq!(updates.len(), 3);

    let parsed: Vec<SessionNotification> = updates.iter().map(parse_notification).collect();

    assert_json_snapshot!(parsed, @r###"
[
  {
    "sessionId": "variant-session",
    "update": {
      "sessionUpdate": "agent_message_chunk",
      "content": {
        "type": "text",
        "text": "stream chunk"
      }
    }
  },
  {
    "sessionId": "variant-session",
    "update": {
      "sessionUpdate": "tool_call",
      "toolCallId": "tool_001",
      "title": "read_file",
      "kind": "read",
      "rawInput": {
        "path": "/tmp/file.txt"
      }
    }
  },
  {
    "sessionId": "variant-session",
    "update": {
      "sessionUpdate": "tool_call_update",
      "toolCallId": "tool_001",
      "status": "completed",
      "content": [
        {
          "type": "content",
          "content": {
            "type": "text",
            "text": "{\n  \"content\": \"done\"\n}"
          }
        }
      ],
      "rawOutput": {
        "content": "done"
      }
    }
  }
]
"###);
}

#[tokio::test]
async fn rich_session_updates_cover_user_thought_plan_and_mode() {
    let mut harness = SnapshotHarness::new("rich-session");

    // user message with plain text
    // ast-grep-ignore: rust-no-unwrap
    harness
        .ingest_raw(r#"{"type":"user_message","message":"User prompt","kind":"plain"}"#)
        .await
        .unwrap();

    // user message carrying image data URL
    // ast-grep-ignore: rust-no-unwrap
    harness
        .ingest_raw(
            r#"{"type":"user_message","message":"","images":["data:image/png;base64,QUJD"]}"#,
        )
        .await
        .unwrap();

    // agent reasoning chunk
    harness
        .ingest_raw(r#"{"type":"agent_reasoning","text":"Thinking step"}"#)
        .await
        // ast-grep-ignore: rust-no-unwrap
        .unwrap();

    // plan update with two steps
    // ast-grep-ignore: rust-no-unwrap
    harness
        .ingest_raw(
            r#"{"type":"plan_update","explanation":"Initial plan","plan":[{"step":"Explore project","status":"pending"},{"step":"Draft fix","status":"in_progress"}]}"#,
        )
        .await
        .unwrap();

    // available commands from MCP tools listing
    // ast-grep-ignore: rust-no-unwrap
    harness
        .ingest_raw(
            r#"{"type":"mcp_list_tools_response","tools":{"server/tool":{"name":"tool","title":"Tool Title","description":"Run the tool","inputSchema":{"type":"object"}}}}"#,
        )
        .await
        .unwrap();

    // current mode update derived from session configuration
    harness
        .ingest_raw(
            r#"{"type":"session_configured","session_id":"session-42","model":"codex-pro","history_log_id":0,"history_entry_count":0,"rollout_path":"/tmp/log"}"#,
        )
        .await
        // ast-grep-ignore: rust-no-unwrap
        .unwrap();

    let updates = harness.drain_json();

    // ast-grep-ignore: rust-no-unwrap
    println!("{}", serde_json::to_string_pretty(&updates).unwrap());

    assert_json_snapshot!(updates, @r###"
[
  {
    "sessionId": "rich-session",
    "update": {
      "content": {
        "text": "User prompt",
        "type": "text"
      },
      "sessionUpdate": "user_message_chunk"
    }
  },
  {
    "sessionId": "rich-session",
    "update": {
      "content": {
        "data": "QUJD",
        "mimeType": "image/png",
        "type": "image"
      },
      "sessionUpdate": "user_message_chunk"
    }
  },
  {
    "sessionId": "rich-session",
    "update": {
      "content": {
        "text": "Thinking step",
        "type": "text"
      },
      "sessionUpdate": "agent_thought_chunk"
    }
  },
  {
    "sessionId": "rich-session",
    "update": {
      "_meta": {
        "explanation": "Initial plan"
      },
      "entries": [
        {
          "content": "Explore project",
          "priority": "medium",
          "status": "pending"
        },
        {
          "content": "Draft fix",
          "priority": "medium",
          "status": "in_progress"
        }
      ],
      "sessionUpdate": "plan"
    }
  },
  {
    "sessionId": "rich-session",
    "update": {
      "availableCommands": [
        {
          "description": "Run the tool",
          "input": null,
          "name": "server/tool"
        }
      ],
      "sessionUpdate": "available_commands_update"
    }
  },
  {
    "sessionId": "rich-session",
    "update": {
      "currentModeId": "codex-pro",
      "sessionUpdate": "current_mode_update"
    }
  }
]
"###);
}

#[tokio::test]
async fn agent_message_json_content_blocks() {
    let mut harness = SnapshotHarness::new("media-session");

    let image_payload = r#"{"type":"image","data":"UVdF","mimeType":"image/png","uri":null}"#;
    let agent_event = json!({
        "type": "agent_message",
        "message": image_payload,
    });
    // ast-grep-ignore: rust-no-unwrap
    harness.ingest_raw(&agent_event.to_string()).await.unwrap();

    let updates = harness.drain_json();

    assert_json_snapshot!(updates, @r###"
[
  {
    "sessionId": "media-session",
    "update": {
      "content": {
        "data": "UVdF",
        "mimeType": "image/png",
        "type": "image"
      },
      "sessionUpdate": "agent_message_chunk"
    }
  }
]
"###);
}
#[tokio::test]
async fn deduplicated_agent_chunks_should_parse_with_official_schema() {
    let mut harness = SnapshotHarness::new("dedup-session");
    // ast-grep-ignore: rust-no-unwrap
    harness
        .ingest_event(CodexEvent::AgentMessage {
            message: "duplicate chunk".to_string(),
            _timestamp: None,
        })
        .await
        .unwrap();
    // ast-grep-ignore: rust-no-unwrap
    harness
        .ingest_event(CodexEvent::AgentMessageDelta {
            delta: "duplicate chunk".to_string(),
            _timestamp: None,
        })
        .await
        .unwrap();

    let updates = harness.drain_json();
    // Guard expectation: duplicate delta suppressed
    assert_eq!(updates.len(), 1);

    let parsed = parse_notification(&updates[0]);
    assert_json_snapshot!(parsed, @r###"
{
  "sessionId": "dedup-session",
  "update": {
    "sessionUpdate": "agent_message_chunk",
    "content": {
      "type": "text",
      "text": "duplicate chunk"
    }
  }
}
"###);
}
