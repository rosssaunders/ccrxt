//! Deribit WebSocket API implementation
//!
//! This module provides WebSocket connectivity to the Deribit API using JSON-RPC 2.0 protocol.
//! Supports both public and private subscription methods.

pub mod messages;
pub mod client;

pub use messages::*;
pub use client::*;