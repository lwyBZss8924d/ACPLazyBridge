//! Core runtime module built around the official Agent Client Protocol runtime.
//!
//! This module will host the `RuntimeServer` orchestration layer, session
//! management, and provider adapter traits used by ACPLazyBridge agent servers.

pub mod adapter;
pub mod server;
pub mod session;

pub use adapter::{ProviderAdapter, SessionNotifier};
pub use server::{RuntimeConfig, RuntimeServer};
pub use session::{SessionState, SessionStore};
