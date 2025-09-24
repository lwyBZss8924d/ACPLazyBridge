//! acp-lazy-core: shared utilities for building ACP agent servers in Rust.
//!
//! This crate provides core functionality for implementing ACP (Agent Client Protocol)
//! servers, including:
//! - JSON-RPC 2.0 protocol handling
//! - Process transport and stdio communication
//! - Shared runtime orchestration built on the Agent Client Protocol
//! - Permission mapping for Codex integration
//! - Connection management following Zed's patterns

pub mod permissions;
pub mod protocol;
pub mod runtime;
pub mod transport;

pub mod logging {
    /// Initialize tracing with environment-based filtering.
    ///
    /// Uses RUST_LOG environment variable to control log levels.
    /// Defaults to "info" if not set.
    ///
    /// Important: We configure the writer to stderr so stdout remains
    /// strictly reserved for JSON-RPC output.
    pub fn init() {
        use tracing_subscriber::prelude::*;
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_target(false)
            .with_writer(std::io::stderr);
        let filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
    }
}

// Re-export commonly used types
pub use permissions::{map_acp_to_codex, AcpPermissionMode, CodexTurnOverrides};
pub use protocol::{
    Error, ErrorCode, IncomingMessage, MessageType, Notification, Request, Response,
};
pub use transport::{read_lines, write_line, MessageQueue, ProcessTransport};
