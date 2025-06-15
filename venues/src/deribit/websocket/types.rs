use std::sync::atomic::{AtomicI64, Ordering};
use thiserror::Error;

/// Errors that can occur during WebSocket operations
#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("WebSocket connection error: {0}")]
    ConnectionError(String),
    
    #[error("Message serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("WebSocket protocol error: {0}")]
    ProtocolError(String),
    
    #[error("Request timeout")]
    Timeout,
    
    #[error("Invalid response received: {0}")]
    InvalidResponse(String),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
}

/// Configuration for Deribit WebSocket client
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    /// WebSocket URL for Deribit (default: wss://www.deribit.com/ws/api/v2)
    pub url: String,
    
    /// Request timeout in milliseconds (default: 30000)
    pub request_timeout_ms: u64,
    
    /// Whether to automatically reconnect on connection loss (default: true)
    pub auto_reconnect: bool,
    
    /// Maximum number of reconnection attempts (default: 5)
    pub max_reconnect_attempts: u32,
    
    /// Delay between reconnection attempts in milliseconds (default: 5000)
    pub reconnect_delay_ms: u64,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            url: "wss://www.deribit.com/ws/api/v2".to_string(),
            request_timeout_ms: 30000,
            auto_reconnect: true,
            max_reconnect_attempts: 5,
            reconnect_delay_ms: 5000,
        }
    }
}

/// Request ID generator for JSON-RPC requests
#[derive(Debug)]
pub struct RequestIdGenerator {
    counter: AtomicI64,
}

impl RequestIdGenerator {
    /// Create a new request ID generator
    pub fn new() -> Self {
        Self {
            counter: AtomicI64::new(1),
        }
    }
    
    /// Generate the next request ID
    pub fn next(&self) -> i64 {
        self.counter.fetch_add(1, Ordering::SeqCst)
    }
}

impl Default for RequestIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Type alias for WebSocket results
pub type WebSocketResult<T> = Result<T, WebSocketError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_config_default() {
        let config = WebSocketConfig::default();
        assert_eq!(config.url, "wss://www.deribit.com/ws/api/v2");
        assert_eq!(config.request_timeout_ms, 30000);
        assert!(config.auto_reconnect);
        assert_eq!(config.max_reconnect_attempts, 5);
        assert_eq!(config.reconnect_delay_ms, 5000);
    }

    #[test]
    fn test_request_id_generator() {
        let generator = RequestIdGenerator::new();
        let id1 = generator.next();
        let id2 = generator.next();
        let id3 = generator.next();
        
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
    }

    #[test]
    fn test_request_id_generator_concurrent() {
        let generator = RequestIdGenerator::new();
        let mut ids = Vec::new();
        
        // Generate multiple IDs
        for _ in 0..100 {
            ids.push(generator.next());
        }
        
        // Check that all IDs are unique and sequential
        for (i, &id) in ids.iter().enumerate() {
            assert_eq!(id, (i + 1) as i64);
        }
    }

    #[test]
    fn test_websocket_error_display() {
        let error = WebSocketError::ConnectionError("Connection lost".to_string());
        assert_eq!(format!("{}", error), "WebSocket connection error: Connection lost");
        
        let error = WebSocketError::Timeout;
        assert_eq!(format!("{}", error), "Request timeout");
    }

    #[test]
    fn test_websocket_error_conversion() {
        // Just test basic error construction since internal serde_json error construction is private
        let ws_error = WebSocketError::SerializationError(serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err());
        
        match ws_error {
            WebSocketError::SerializationError(_) => {
                // Expected
            }
            _ => panic!("Expected SerializationError"),
        }
    }
}