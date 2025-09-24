//! Provider adapter traits and utilities.
//!
//! RuntimeServer delegates provider-specific behavior (process spawning,
//! streaming, notify integration) to an implementation of `ProviderAdapter`.

use agent_client_protocol::{
    AgentCapabilities, CancelNotification, Error, PromptRequest, PromptResponse, SessionId,
    SessionNotification,
};
use async_trait::async_trait;
use tokio::sync::mpsc;

use super::{server::RuntimeConfig, session::SessionState};

/// Channel used by adapters to emit ACP session notifications.
pub type SessionNotifier = Option<mpsc::UnboundedSender<SessionNotification>>;

/// Provider-specific behavior required by the shared runtime.
#[async_trait(?Send)]
pub trait ProviderAdapter: Send + Sync {
    /// Capabilities advertised during initialize.
    fn agent_capabilities(&self) -> AgentCapabilities;

    /// Invoked after `RuntimeServer` records a new session.
    async fn on_session_created(&self, _session: &SessionState) -> Result<(), Error> {
        Ok(())
    }

    /// Process a prompt turn for the given session.
    async fn handle_prompt(
        &self,
        session: SessionState,
        request: PromptRequest,
        notifier: SessionNotifier,
        config: &RuntimeConfig,
    ) -> Result<PromptResponse, Error>;

    /// Handle cancellation notification from the client.
    async fn handle_cancel(&self, _notification: CancelNotification) -> Result<(), Error> {
        Ok(())
    }

    /// Invoked when a session mode is updated; adapters may adjust internal state.
    async fn on_permission_mode_changed(
        &self,
        _session_id: &SessionId,
        _session: &SessionState,
    ) -> Result<(), Error> {
        Ok(())
    }
}
