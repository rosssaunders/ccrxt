//! Deribit WebSocket API implementation
//!
//! This module provides WebSocket connectivity to the Deribit exchange.
//! It implements the common WebSocket trait and provides access to all
//! public and private WebSocket endpoints.

pub mod client;
pub mod messages;
pub mod public;

// Re-export main types
pub use client::DeribitWebSocketClient;
pub use messages::{DeribitMessage, JsonRpcRequest, JsonRpcResponse, JsonRpcError};