//! Session state definitions for the shared runtime.
//!
//! These structures will track permission modes, working directories, notify
//! sources, and child processes once the runtime is implemented.

use agent_client_protocol::SessionId;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::permissions::AcpPermissionMode;

/// Session metadata tracked by the runtime.
#[derive(Debug, Clone)]
pub struct SessionState {
    pub session_id: SessionId,
    pub working_dir: PathBuf,
    pub permission_mode: AcpPermissionMode,
}

impl SessionState {
    /// Create a new session state instance.
    pub fn new(
        session_id: SessionId,
        working_dir: PathBuf,
        permission_mode: AcpPermissionMode,
    ) -> Self {
        Self {
            session_id,
            working_dir,
            permission_mode,
        }
    }
}

/// Shared session store wrapper used by the runtime.
#[derive(Clone, Default)]
pub struct SessionStore {
    inner: Arc<RwLock<HashMap<SessionId, SessionState>>>,
}

impl SessionStore {
    /// Create a new, empty session store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Fetch the number of tracked sessions. Primarily used by tests.
    pub async fn len(&self) -> usize {
        self.inner.read().await.len()
    }

    /// Returns true if no sessions are currently tracked.
    pub async fn is_empty(&self) -> bool {
        self.inner.read().await.is_empty()
    }

    pub async fn insert(&self, state: SessionState) -> Option<SessionState> {
        self.inner
            .write()
            .await
            .insert(state.session_id.clone(), state)
    }

    pub async fn get(&self, session_id: &SessionId) -> Option<SessionState> {
        self.inner.read().await.get(session_id).cloned()
    }

    pub async fn update_permission(
        &self,
        session_id: &SessionId,
        mode: AcpPermissionMode,
    ) -> Option<SessionState> {
        let mut guard = self.inner.write().await;
        if let Some(state) = guard.get_mut(session_id) {
            state.permission_mode = mode;
            return Some(state.clone());
        }
        None
    }
}
