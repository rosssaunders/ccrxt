//! BitMart public WebSocket client
//!
//! This module provides a WebSocket client for BitMart public market data streams.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// BitMart WebSocket URL for public channels
pub const BITMART_WS_PUBLIC_URL: &str = "wss://ws-manager-compress.bitmart.com/api?protocol=1.1";

/// WebSocket client for BitMart public channels
pub struct WsClient {
    url: String,
}

impl WsClient {
    /// Creates a new WebSocket client for public channels
    pub fn new() -> Self {
        Self {
            url: BITMART_WS_PUBLIC_URL.to_string(),
        }
    }

    /// Creates a new WebSocket client with a custom URL
    pub fn with_url(url: String) -> Self {
        Self { url }
    }

    /// Gets the WebSocket URL
    pub fn url(&self) -> &str {
        &self.url
    }
}

impl Default for WsClient {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket operation types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    Subscribe,
    Unsubscribe,
    Login,
}

/// WebSocket message sent to the server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    /// Operation type
    pub op: Operation,
    /// Arguments (topics for subscribe/unsubscribe, credentials for login)
    pub args: Vec<String>,
}

/// WebSocket response from the server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WsResponse {
    /// Event response (login, subscribe, unsubscribe confirmation)
    Event(EventResponse),
    /// Data response (actual market data)
    Data(DataResponse),
    /// Error response
    Error(ErrorResponse),
}

/// Event response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventResponse {
    /// Event type
    pub event: Operation,
    /// Topic (for subscribe/unsubscribe responses)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

/// Data response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataResponse {
    /// Table/topic name
    pub table: String,
    /// Data payload
    pub data: Vec<serde_json::Value>,
}

/// Error response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error event
    pub event: String,
    /// Error code
    #[serde(rename = "errorCode")]
    pub error_code: String,
    /// Error message
    pub message: String,
}

/// WebSocket errors
#[derive(Error, Debug)]
pub enum WsError {
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("API error: {code} - {message}")]
    Api { code: String, message: String },
}

/// WebSocket channel types for public data
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicChannel {
    /// Ticker data
    Ticker,
    /// Order book data
    Depth,
    /// Recent trades
    Trade,
    /// Kline/candlestick data
    Kline,
}

impl PublicChannel {
    /// Convert channel to string format for topic construction
    pub fn as_str(&self) -> &'static str {
        match self {
            PublicChannel::Ticker => "spot/ticker",
            PublicChannel::Depth => "spot/depth",
            PublicChannel::Trade => "spot/trade",
            PublicChannel::Kline => "spot/kline",
        }
    }

    /// Create a topic string for a specific trading pair
    ///
    /// # Arguments
    /// * `symbol` - Trading pair symbol (e.g., "BTC_USDT")
    /// * `params` - Additional parameters for some channels (e.g., kline interval)
    pub fn topic(&self, symbol: &str, params: Option<&str>) -> String {
        match params {
            Some(p) => format!("{}{}:{}", self.as_str(), p, symbol),
            None => format!("{}:{}", self.as_str(), symbol),
        }
    }
}

impl WsMessage {
    /// Create a subscribe message
    pub fn subscribe(topics: Vec<String>) -> Self {
        Self {
            op: Operation::Subscribe,
            args: topics,
        }
    }

    /// Create an unsubscribe message
    pub fn unsubscribe(topics: Vec<String>) -> Self {
        Self {
            op: Operation::Unsubscribe,
            args: topics,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_client_creation() {
        let client = WsClient::new();
        assert_eq!(client.url(), BITMART_WS_PUBLIC_URL);

        let custom_client = WsClient::with_url("wss://custom.url".to_string());
        assert_eq!(custom_client.url(), "wss://custom.url");
    }

    #[test]
    fn test_public_channel_topics() {
        assert_eq!(
            PublicChannel::Ticker.topic("BTC_USDT", None),
            "spot/ticker:BTC_USDT"
        );
        assert_eq!(
            PublicChannel::Depth.topic("ETH_USDT", None),
            "spot/depth:ETH_USDT"
        );
        assert_eq!(
            PublicChannel::Kline.topic("BTC_USDT", Some("1m")),
            "spot/kline1m:BTC_USDT"
        );
    }

    #[test]
    fn test_ws_message_creation() {
        let subscribe_msg = WsMessage::subscribe(vec!["spot/ticker:BTC_USDT".to_string()]);
        assert_eq!(subscribe_msg.op, Operation::Subscribe);
        assert_eq!(subscribe_msg.args, vec!["spot/ticker:BTC_USDT"]);

        let unsubscribe_msg = WsMessage::unsubscribe(vec!["spot/ticker:BTC_USDT".to_string()]);
        assert_eq!(unsubscribe_msg.op, Operation::Unsubscribe);
        assert_eq!(unsubscribe_msg.args, vec!["spot/ticker:BTC_USDT"]);
    }

    #[test]
    fn test_message_serialization() {
        let msg = WsMessage::subscribe(vec!["spot/ticker:BTC_USDT".to_string()]);
        let json = serde_json::to_string(&msg).expect("Failed to serialize");
        let expected = r#"{"op":"subscribe","args":["spot/ticker:BTC_USDT"]}"#;
        assert_eq!(json, expected);
    }
}
