use async_trait::async_trait;
use bytes::Bytes;
use thiserror::Error;

/// Result alias for WebSocket operations
pub type WebSocketResult<T> = Result<T, WebSocketError>;

/// Platform-agnostic WebSocket error type
#[derive(Debug, Error)]
pub enum WebSocketError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Send error: {0}")]
    Send(String),

    #[error("Receive error: {0}")]
    Receive(String),

    #[error("Timeout")]
    Timeout,

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("Not connected")]
    NotConnected,

    #[error("Other: {0}")]
    Other(String),
}

/// Incoming message from the WebSocket stream.
#[derive(Debug, Clone)]
pub enum IncomingMessage {
    Binary(Bytes),
    Text(String),
}

// Native environments must be Send + Sync to allow multithreaded usage.
#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
pub trait WebSocketClient: Send + Sync {
    /// Establish the connection to the given URL and remain ready to send/receive.
    async fn connect(&mut self, url: &str) -> WebSocketResult<()>;

    /// Close the connection gracefully.
    async fn disconnect(&mut self) -> WebSocketResult<()>;

    /// Send a binary message. Callers should handle serialization.
    async fn send(&mut self, message: Bytes) -> WebSocketResult<()>;

    /// Receive the next message if available. Returns Ok(None) if the stream finished.
    async fn receive(&mut self) -> WebSocketResult<Option<IncomingMessage>>;

    /// Whether the underlying connection is established.
    fn is_connected(&self) -> bool;
}
