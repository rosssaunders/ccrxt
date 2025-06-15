//! Public unsubscribe endpoint implementation
//!
//! This module implements the `/public/unsubscribe` WebSocket endpoint for Deribit.
//! This method is only available via WebSockets and allows unsubscribing from one or more channels.

use crate::deribit::websocket::client::DeribitWebSocketClient;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use websockets::BoxResult;

/// Request parameters for public/unsubscribe
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnsubscribeRequest {
    /// A list of channels to unsubscribe from
    pub channels: Vec<String>,
}

impl UnsubscribeRequest {
    /// Create a new unsubscribe request
    pub fn new(channels: Vec<String>) -> Self {
        Self { channels }
    }

    /// Create a new unsubscribe request for a single channel
    pub fn single_channel(channel: impl Into<String>) -> Self {
        Self {
            channels: vec![channel.into()],
        }
    }

    /// Add a channel to the unsubscribe request
    pub fn add_channel(&mut self, channel: impl Into<String>) {
        self.channels.push(channel.into());
    }

    /// Convert to JSON-RPC params
    pub fn to_params(&self) -> Value {
        json!({ "channels": self.channels })
    }
}

/// Response from public/unsubscribe
/// 
/// This represents the content of the "result" field in the JSON-RPC response.
/// The result is an array of strings representing the remaining subscribed channels.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnsubscribeResponse(pub Vec<String>);

impl UnsubscribeResponse {
    /// Create a new unsubscribe response
    pub fn new(channels: Vec<String>) -> Self {
        Self(channels)
    }

    /// Get the channels
    pub fn channels(&self) -> &Vec<String> {
        &self.0
    }

    /// Check if the response contains any remaining subscriptions
    pub fn has_remaining_subscriptions(&self) -> bool {
        !self.0.is_empty()
    }

    /// Get the number of remaining subscriptions
    pub fn remaining_subscription_count(&self) -> usize {
        self.0.len()
    }
}

impl TryFrom<Value> for UnsubscribeResponse {
    type Error = serde_json::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let channels: Vec<String> = serde_json::from_value(value)?;
        Ok(UnsubscribeResponse(channels))
    }
}

/// Errors that can occur during unsubscribe operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum UnsubscribeError {
    /// WebSocket connection error
    #[error("WebSocket error: {0}")]
    WebSocketError(String),
    
    /// JSON-RPC error from server
    #[error("JSON-RPC error {code}: {message}")]
    JsonRpcError { code: i32, message: String },
    
    /// Invalid channel name
    #[error("Invalid channel: {channel}")]
    InvalidChannel { channel: String },
    
    /// Channel not subscribed
    #[error("Not subscribed to channel: {channel}")]
    NotSubscribed { channel: String },
    
    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl DeribitWebSocketClient {
    /// Unsubscribe from one or more channels
    ///
    /// # Arguments
    /// * `request` - The unsubscribe request containing channels to unsubscribe from
    ///
    /// # Returns
    /// * `Ok(i64)` - The request ID for tracking the response
    /// * `Err(BoxError)` - If the request could not be sent
    ///
    /// # Example
    /// ```rust,ignore
    /// use venues::deribit::websocket::{DeribitWebSocketClient, UnsubscribeRequest};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = DeribitWebSocketClient::new_default();
    /// client.connect().await?;
    ///
    /// let request = UnsubscribeRequest::new(vec![
    ///     "deribit_price_index.btc_usd".to_string(),
    ///     "trades.BTC-PERPETUAL.raw".to_string(),
    /// ]);
    ///
    /// let request_id = client.public_unsubscribe(&request).await?;
    /// println!("Sent unsubscribe request with ID: {}", request_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn public_unsubscribe(&self, request: &UnsubscribeRequest) -> BoxResult<i64> {
        let params = request.to_params();
        self.send_request("public/unsubscribe", Some(params)).await
    }

    /// Unsubscribe from a single channel (convenience method)
    ///
    /// # Arguments
    /// * `channel` - The channel to unsubscribe from
    ///
    /// # Returns
    /// * `Ok(i64)` - The request ID for tracking the response
    /// * `Err(BoxError)` - If the request could not be sent
    pub async fn public_unsubscribe_single(&self, channel: impl Into<String>) -> BoxResult<i64> {
        let request = UnsubscribeRequest::single_channel(channel);
        self.public_unsubscribe(&request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsubscribe_request_creation() {
        let channels = vec![
            "deribit_price_index.btc_usd".to_string(),
            "trades.BTC-PERPETUAL.raw".to_string(),
        ];
        let request = UnsubscribeRequest::new(channels.clone());
        
        assert_eq!(request.channels, channels);
    }

    #[test]
    fn test_unsubscribe_request_single_channel() {
        let request = UnsubscribeRequest::single_channel("test.channel");
        
        assert_eq!(request.channels.len(), 1);
        assert_eq!(request.channels[0], "test.channel");
    }

    #[test]
    fn test_unsubscribe_request_add_channel() {
        let mut request = UnsubscribeRequest::single_channel("channel1");
        request.add_channel("channel2");
        request.add_channel("channel3");
        
        assert_eq!(request.channels.len(), 3);
        assert_eq!(request.channels, vec!["channel1", "channel2", "channel3"]);
    }

    #[test]
    fn test_unsubscribe_request_to_params() {
        let request = UnsubscribeRequest::new(vec![
            "deribit_price_index.btc_usd".to_string(),
            "trades.BTC-PERPETUAL.raw".to_string(),
        ]);
        
        let params = request.to_params();
        let expected = json!({
            "channels": [
                "deribit_price_index.btc_usd",
                "trades.BTC-PERPETUAL.raw"
            ]
        });
        
        assert_eq!(params, expected);
    }

    #[test]
    fn test_unsubscribe_request_serialization() {
        let request = UnsubscribeRequest::new(vec![
            "test.channel1".to_string(),
            "test.channel2".to_string(),
        ]);
        
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: UnsubscribeRequest = serde_json::from_str(&json)
            .expect("Failed to deserialize");
        
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_unsubscribe_response_creation() {
        let channels = vec![
            "remaining.channel1".to_string(),
            "remaining.channel2".to_string(),
        ];
        let response = UnsubscribeResponse::new(channels.clone());
        
        assert_eq!(response.channels(), &channels);
        assert!(response.has_remaining_subscriptions());
        assert_eq!(response.remaining_subscription_count(), 2);
    }

    #[test]
    fn test_unsubscribe_response_empty() {
        let response = UnsubscribeResponse::new(vec![]);
        
        assert!(!response.has_remaining_subscriptions());
        assert_eq!(response.remaining_subscription_count(), 0);
    }

    #[test]
    fn test_unsubscribe_response_from_value() {
        let value = json!(["channel1", "channel2"]);
        let response = UnsubscribeResponse::try_from(value)
            .expect("Failed to convert from Value");
        
        assert_eq!(response.channels(), &vec!["channel1", "channel2"]);
    }

    #[test]
    fn test_unsubscribe_response_serialization() {
        let response = UnsubscribeResponse::new(vec![
            "test.channel1".to_string(),
            "test.channel2".to_string(),
        ]);
        
        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: UnsubscribeResponse = serde_json::from_str(&json)
            .expect("Failed to deserialize");
        
        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_unsubscribe_error_display() {
        let error = UnsubscribeError::InvalidChannel {
            channel: "invalid.channel".to_string(),
        };
        
        let error_string = format!("{}", error);
        assert!(error_string.contains("Invalid channel: invalid.channel"));
    }

    #[test]
    fn test_unsubscribe_error_json_rpc() {
        let error = UnsubscribeError::JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
        };
        
        let error_string = format!("{}", error);
        assert!(error_string.contains("JSON-RPC error -32602: Invalid params"));
    }

    #[tokio::test]
    async fn test_client_unsubscribe_when_not_connected() {
        let client = DeribitWebSocketClient::new_default();
        let request = UnsubscribeRequest::single_channel("test.channel");
        
        let result = client.public_unsubscribe(&request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_client_unsubscribe_single_when_not_connected() {
        let client = DeribitWebSocketClient::new_default();
        
        let result = client.public_unsubscribe_single("test.channel").await;
        assert!(result.is_err());
    }
}