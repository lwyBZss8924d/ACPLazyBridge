//! acp-lazy-core: shared utilities for building ACP agent servers in Rust.
//!
//! This crate provides core functionality for implementing ACP (Agent Client Protocol)
//! servers, including:
//! - JSON-RPC 2.0 protocol handling
//! - Process transport and stdio communication
//! - Permission mapping for Codex integration
//! - Connection management following Zed's patterns

pub mod permissions;
pub mod protocol;
pub mod transport;

pub mod logging {
    /// Initialize tracing with environment-based filtering.
    /// 
    /// Uses RUST_LOG environment variable to control log levels.
    /// Defaults to "info" if not set.
    pub fn init() {
        use tracing_subscriber::prelude::*;
        let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);
        let filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
        tracing_subscriber::registry().with(filter).with(fmt_layer).init();
    }
}

// Re-export commonly used types
pub use permissions::{AcpPermissionMode, CodexTurnOverrides, map_acp_to_codex};
pub use protocol::{Error, ErrorCode, Request, Response, Notification, IncomingMessage, MessageType};
pub use transport::{ProcessTransport, MessageQueue, read_lines, write_line};

