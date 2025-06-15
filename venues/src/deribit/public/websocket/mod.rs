//! WebSocket endpoints for Deribit public API

pub mod client;
pub mod hello;

pub use client::{DeribitMessage, DeribitWebSocketClient};
pub use hello::{HelloRequest, HelloResponse, HelloResult, JsonRpcRequest};