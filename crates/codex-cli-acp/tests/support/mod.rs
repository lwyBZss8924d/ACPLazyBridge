use agent_client_protocol::{SessionId, SessionNotification};
use anyhow::Result;
use codex_cli_acp::codex_proto::{CodexEvent, CodexStreamManager};
use serde_json::{self, Value};
use std::sync::Arc;
use tokio::sync::mpsc;

/// Test-only helper that forwards Codex proto events through the CodexStreamManager
/// and exposes serialized ACP notifications for assertions.
pub struct SnapshotHarness {
    manager: CodexStreamManager,
    rx: mpsc::UnboundedReceiver<SessionNotification>,
}

impl SnapshotHarness {
    /// Create a harness bound to the provided session identifier.
    pub fn new(session_id: &str) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let session_id = SessionId(Arc::from(session_id));
        let manager = CodexStreamManager::new(session_id, tx);
        Self { manager, rx }
    }

    /// Feed a Codex event into the stream manager.
    pub async fn ingest_event(&mut self, event: CodexEvent) -> Result<()> {
        let line = serde_json::to_string(&event)?;
        self.manager.process_line(&line).await
    }

    /// Feed a raw Codex output line into the stream manager.
    #[allow(dead_code)]
    pub async fn ingest_raw(&mut self, raw: &str) -> Result<()> {
        self.manager.process_line(raw).await
    }

    /// Drain all pending session updates as JSON values ready for snapshot
    /// comparison. The harness intentionally ignores serialization errors so
    /// a single malformed update does not poison the channel.
    pub fn drain_json(&mut self) -> Vec<Value> {
        let mut updates = Vec::new();
        while let Ok(update) = self.rx.try_recv() {
            if let Ok(json) = serde_json::to_value(update) {
                updates.push(json);
            }
        }
        updates
    }
}
