//! Platform-agnostic WebSocket abstraction.
//!
//! This crate provides a unified `WebSocketClient` trait with a native
//! implementation and utilities for connection and request/response
//! correlation. Host code controls connection lifecycle and reconnection.

// Re-export commonly used items
pub use crate::client::{IncomingMessage, WebSocketClient, WebSocketError, WebSocketResult};
pub use crate::connection::{ConnectionState, WebSocketEvent};
pub use crate::message::{RequestId, RequestIdGenerator, RequestManager};

// Legacy types removed.

pub mod client;
pub mod connection;
pub mod message;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;
