//! WebSocket endpoints for Deribit public API

pub mod client;
pub mod hello;
pub mod subscribe;
pub mod unsubscribe;
pub mod unsubscribe_all;

// Only connection management and client struct from client.rs
pub use client::{PrivateWebSocketClient, DeribitWebSocketError};
// Endpoint request/response types and methods
pub use hello::{HelloRequest, HelloResponse, HelloResult};
pub use subscribe::{SubscribeRequest, SubscribeResponse};
pub use unsubscribe::{UnsubscribeRequest, UnsubscribeResponse};
pub use unsubscribe_all::{UnsubscribeAllRequest, UnsubscribeAllResponse};
