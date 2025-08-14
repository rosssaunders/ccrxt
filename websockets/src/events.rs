use crate::VenueMessage;

/// All possible WebSocket events
#[derive(Debug)]
pub enum WebSocketEvent<T: VenueMessage> {
    /// Connection established successfully
    Connected,

    /// Connection closed (with reason)
    Disconnected { reason: DisconnectReason },

    /// Error occurred (connection may still be active)
    Error { error: WebSocketError },

    /// Message received from server
    Message { message: T },

    /// Ping received (for venues that expose ping/pong)
    PingReceived { data: Vec<u8> },

    /// Pong received
    PongReceived { data: Vec<u8> },
}

/// Reasons for WebSocket disconnection
#[derive(Debug, Clone)]
pub enum DisconnectReason {
    /// User called disconnect()
    UserInitiated,

    /// Server closed connection
    RemoteClosed { code: u16, reason: String },

    /// Network error
    NetworkError { details: String },

    /// Protocol error
    ProtocolError { details: String },

    /// Invalid message received
    InvalidMessage { details: String },
}

/// WebSocket connection states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Not connected
    Disconnected,

    /// Connection in progress
    Connecting,

    /// Connected and ready
    Connected,

    /// Disconnection in progress
    Disconnecting,
}

/// WebSocket errors
#[derive(Debug, thiserror::Error)]
pub enum WebSocketError {
    /// Connection failed
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// Already connected
    #[error("Already connected")]
    AlreadyConnected,

    /// Not connected
    #[error("Not connected")]
    NotConnected,

    /// Send failed
    #[error("Send failed: {0}")]
    SendFailed(String),

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    /// Platform-specific error
    #[error("Platform error: {0}")]
    PlatformError(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}
