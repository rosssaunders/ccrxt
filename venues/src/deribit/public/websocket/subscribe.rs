//! Request and response structures for public/subscribe WebSocket endpoint
//!
//! This method is used to subscribe to one or more public channels.
//! This is the same method as /private/subscribe, but it can only be used for 'public' channels.

use crate::deribit::public::websocket::client::{DeribitWebSocketError, PrivateWebSocketClient};
use serde::{Deserialize, Serialize};

/// Request parameters for the public/subscribe endpoint.
///
/// Subscribe to one or more public channels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeRequest {
    /// A list of channels to subscribe to
    pub channels: Vec<String>,
}

/// Response for public/subscribe endpoint.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SubscribeResponse {
    /// A list of subscribed channels
    pub result: Vec<String>,
}

impl PrivateWebSocketClient {
    /// Send a subscribe request and wait for the response
    pub async fn subscribe(&mut self, request: SubscribeRequest) -> Result<SubscribeResponse, DeribitWebSocketError> {
        self.send_and_receive::<SubscribeRequest, SubscribeResponse>(&request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_subscribe_request_serialization() {
        let channels = vec![
            "book.BTC-PERPETUAL.100ms".to_string(),
            "trades.BTC-PERPETUAL".to_string(),
        ];
        let request = SubscribeRequest { channels };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["channels"][0], "book.BTC-PERPETUAL.100ms");
        assert_eq!(parsed["channels"][1], "trades.BTC-PERPETUAL");
    }

    #[test]
    fn test_subscribe_request_single_channel() {
        let channels = vec!["ticker.BTC-PERPETUAL".to_string()];
        let request = SubscribeRequest { channels };

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"channels":["ticker.BTC-PERPETUAL"]}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn test_subscribe_response_deserialization() {
        let response_json = r#"{"id":1,"jsonrpc":"2.0","result":["book.BTC-PERPETUAL.100ms","trades.BTC-PERPETUAL"]}"#;

        let response: SubscribeResponse = serde_json::from_str(response_json).unwrap();

        assert_eq!(response.result.len(), 2);
        assert_eq!(response.result[0], "book.BTC-PERPETUAL.100ms");
        assert_eq!(response.result[1], "trades.BTC-PERPETUAL");
    }

    #[test]
    fn test_subscribe_response_single_channel() {
        let response_json = r#"{"id":123,"jsonrpc":"2.0","result":["ticker.ETH-PERPETUAL"]}"#;

        let response: SubscribeResponse = serde_json::from_str(response_json).unwrap();

        assert_eq!(response.result.len(), 1);
        assert_eq!(response.result[0], "ticker.ETH-PERPETUAL");
    }

    #[test]
    fn test_subscribe_request_structure() {
        let channels = vec!["channel1".to_string(), "channel2".to_string()];
        let subscribe_req = SubscribeRequest {
            channels: channels.clone(),
        };

        assert_eq!(subscribe_req.channels, channels);
    }

    #[test]
    fn test_subscribe_request_empty_channels() {
        let channels: Vec<String> = vec![];
        let request = SubscribeRequest { channels };

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"channels":[]}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn test_subscribe_response_empty_result() {
        let response_json = r#"{"id":1,"jsonrpc":"2.0","result":[]}"#;

        let response: SubscribeResponse = serde_json::from_str(response_json).unwrap();

        assert_eq!(response.result.len(), 0);
    }
}
