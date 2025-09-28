//! Tool call lifecycle scaffolding.
//!
//! Phase 3.2 introduces RED tests that encode the expected agent_client_protocol
//! lifecycle semantics. Implementation must refactor the streaming mapper until
//! these tests pass.

#[path = "support/mod.rs"]
mod support;

use agent_client_protocol::{
    ContentBlock, SessionId, SessionNotification, SessionUpdate, StopReason, ToolCallStatus,
};
use codex_cli_acp::codex_proto::{CodexEvent, ToolCallItem};
use codex_cli_acp::notify_source::NotifyEvent;
use insta::assert_json_snapshot;
use serde_json::json;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use support::SnapshotHarness;
use tokio::sync::{mpsc, Notify};
use tokio::time::{sleep, Instant};

fn parse_notification(value: &serde_json::Value) -> SessionNotification {
    // ast-grep-ignore: rust-no-unwrap
    serde_json::from_value(value.clone())
        .expect("Tool call update should already conform to agent_client_protocol v0.4.3")
}

async fn simulate_stop_reason(
    cancel_notify: Arc<Notify>,
    cancel_flag: Arc<AtomicBool>,
    mut update_rx: mpsc::UnboundedReceiver<SessionNotification>,
    mut notify_rx: Option<mpsc::UnboundedReceiver<NotifyEvent>>,
    idle_interval: Duration,
    idle_timeout: Duration,
) -> StopReason {
    let mut last_activity = Instant::now();
    let mut stream_open = true;
    let mut notify_enabled = notify_rx.is_some();
    let mut stop_reason = StopReason::EndTurn;

    let idle_timer = tokio::time::sleep(idle_interval);
    tokio::pin!(idle_timer);

    loop {
        tokio::select! {
            _ = cancel_notify.notified() => {
                stop_reason = StopReason::Cancelled;
                break;
            }
            update = update_rx.recv(), if stream_open => {
                match update {
                    Some(_) => {
                        last_activity = Instant::now();
                    }
                    None => {
                        stream_open = false;
                    }
                }
            }
            notify = async {
                match &mut notify_rx {
                    Some(rx) => rx.recv().await,
                    None => None,
                }
            }, if notify_enabled => {
                match notify {
                    Some(event) => {
                        if event.event_type == "agent-turn-complete" {
                            stop_reason = StopReason::EndTurn;
                            break;
                        }
                    }
                    None => {
                        notify_enabled = false;
                    }
                }
            }
            _ = &mut idle_timer => {
                if cancel_flag.load(Ordering::SeqCst) {
                    stop_reason = StopReason::Cancelled;
                    break;
                }

                let now = Instant::now();
                if now.duration_since(last_activity) >= idle_timeout {
                    stop_reason = StopReason::EndTurn;
                    break;
                }

                idle_timer.as_mut().reset(now + idle_interval);
            }
        }

        if !stream_open && !notify_enabled {
            break;
        }
    }

    stop_reason
}

#[tokio::test]
async fn harness_emits_tool_call_updates() {
    let mut harness = SnapshotHarness::new("tool-call-session");
    let event = CodexEvent::ToolCall {
        id: "tool_001".to_string(),
        name: "read_file".to_string(),
        arguments: json!({"path": "/tmp/file.txt"}),
        status: Some("pending".to_string()),
        output: None,
        error: None,
    };

    // ast-grep-ignore: rust-no-unwrap
    harness.ingest_event(event).await.unwrap();
    let updates = harness.drain_json();

    assert_eq!(updates.len(), 1);
    let update_tag = updates[0]
        .get("update")
        .and_then(|update| update.get("sessionUpdate"))
        .and_then(|tag| tag.as_str());

    assert_eq!(update_tag, Some("tool_call"));
}

#[tokio::test]
async fn harness_emits_batched_tool_calls() {
    let mut harness = SnapshotHarness::new("tool-call-session");
    let event = CodexEvent::ToolCalls {
        calls: vec![
            ToolCallItem {
                id: "tool_a".to_string(),
                name: "list_files".to_string(),
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
    harness.ingest_raw(&event_json).await.unwrap();

    let updates = harness.drain_json();

    assert_eq!(updates.len(), 2);
    assert!(updates.iter().all(|update| {
        update
            .get("update")
            .and_then(|u| u.get("sessionUpdate"))
            .and_then(|tag| tag.as_str())
            == Some("tool_call")
    }));
}

#[tokio::test]
async fn tool_call_status_transitions_use_official_update_fields() {
    let mut harness = SnapshotHarness::new("status-session");

    // ast-grep-ignore: rust-no-unwrap
    harness
        .ingest_event(CodexEvent::ToolCall {
            id: "tool_state".to_string(),
            name: "read_file".to_string(),
            arguments: json!({"path": "./file.txt"}),
            status: Some("pending".to_string()),
            output: None,
            error: None,
        })
        .await
        .unwrap();

    // ast-grep-ignore: rust-no-unwrap
    harness
        .ingest_event(CodexEvent::ToolCall {
            id: "tool_state".to_string(),
            name: "read_file".to_string(),
            arguments: json!({"path": "./file.txt"}),
            status: Some("in_progress".to_string()),
            output: None,
            error: None,
        })
        .await
        .unwrap();

    // ast-grep-ignore: rust-no-unwrap
    harness
        .ingest_event(CodexEvent::ToolCall {
            id: "tool_state".to_string(),
            name: "read_file".to_string(),
            arguments: json!({"path": "./file.txt"}),
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
    "sessionId": "status-session",
    "update": {
      "sessionUpdate": "tool_call",
      "toolCallId": "tool_state",
      "title": "read_file",
      "kind": "read",
      "rawInput": {
        "path": "./file.txt"
      }
    }
  },
  {
    "sessionId": "status-session",
    "update": {
      "sessionUpdate": "tool_call_update",
      "toolCallId": "tool_state",
      "status": "in_progress",
      "content": [
        {
          "type": "content",
          "content": {
            "type": "text",
            "text": "Tool is running..."
          }
        }
      ]
    }
  },
  {
    "sessionId": "status-session",
    "update": {
      "sessionUpdate": "tool_call_update",
      "toolCallId": "tool_state",
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
async fn tool_call_error_maps_to_failed_status_update() {
    let mut harness = SnapshotHarness::new("error-session");

    // ast-grep-ignore: rust-no-unwrap
    harness
        .ingest_event(CodexEvent::ToolCall {
            id: "tool_error".to_string(),
            name: "read_file".to_string(),
            arguments: json!({"path": "./missing.txt"}),
            status: Some("pending".to_string()),
            output: None,
            error: None,
        })
        .await
        .unwrap();

    // ast-grep-ignore: rust-no-unwrap
    harness
        .ingest_event(CodexEvent::Error {
            message: "File not found".to_string(),
            code: Some("ENOENT".to_string()),
        })
        .await
        .unwrap();

    let updates = harness.drain_json();
    assert_eq!(updates.len(), 2);

    let pending = parse_notification(&updates[0]);
    match &pending.update {
        SessionUpdate::ToolCall(call) => {
            assert_eq!(call.status, ToolCallStatus::Pending);
        }
        other => panic!("expected ToolCall variant, got {:?}", other),
    }

    let failed = parse_notification(&updates[1]);
    assert_json_snapshot!(failed, @r###"
{
  "sessionId": "error-session",
  "update": {
    "sessionUpdate": "tool_call_update",
    "toolCallId": "tool_error",
    "kind": "read",
    "status": "failed",
    "title": "read_file",
    "content": [
      {
        "type": "content",
        "content": {
          "type": "text",
          "text": "Error [ENOENT]: File not found"
        }
      }
    ],
    "rawInput": {
      "path": "./missing.txt"
    },
    "rawOutput": {
      "code": -32603,
      "data": {
        "category": "error",
        "codex_code": "ENOENT"
      },
      "message": "File not found"
    }
  }
}
"###);
}

#[tokio::test]
async fn notify_completion_should_emit_official_stop_reason() {
    let cancel_notify = Arc::new(Notify::new());
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let (update_tx, update_rx) = mpsc::unbounded_channel();
    let (notify_tx, notify_rx) = mpsc::unbounded_channel();

    let idle_interval = Duration::from_millis(5);
    let idle_timeout = Duration::from_millis(100);

    let simulation = simulate_stop_reason(
        cancel_notify.clone(),
        cancel_flag.clone(),
        update_rx,
        Some(notify_rx),
        idle_interval,
        idle_timeout,
    );

    // ast-grep-ignore: rust-no-unwrap
    update_tx
        .send(SessionNotification {
            session_id: SessionId(Arc::from("notify-session")),
            update: SessionUpdate::AgentMessageChunk {
                content: ContentBlock::from("ping"),
            },
            meta: None,
        })
        .unwrap();

    tokio::spawn(async move {
        sleep(Duration::from_millis(20)).await;
        let _ = notify_tx.send(NotifyEvent {
            event_type: "agent-turn-complete".into(),
            turn_id: None,
            input_messages: None,
            last_assistant_message: None,
        });
    });

    let stop_reason = simulation.await;
    assert_eq!(stop_reason, StopReason::EndTurn);
}

#[tokio::test]
async fn idle_timeout_should_emit_official_stop_reason() {
    let cancel_notify = Arc::new(Notify::new());
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let (_update_tx, update_rx) = mpsc::unbounded_channel::<SessionNotification>();

    let stop_reason = simulate_stop_reason(
        cancel_notify,
        cancel_flag,
        update_rx,
        None,
        Duration::from_millis(5),
        Duration::from_millis(20),
    )
    .await;

    assert_eq!(stop_reason, StopReason::EndTurn);
}
