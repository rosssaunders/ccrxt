//! Request and response structures for public/subscribe WebSocket endpoint
//!
//! This method is used to subscribe to one or more public channels.
//! This is the same method as /private/subscribe, but it can only be used for 'public' channels.

use crate::deribit::public::websocket::client::{DeribitWebSocketClient, DeribitWebSocketError};
use futures::SinkExt;
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
    /// The id that was sent in the request
    pub id: i32,

    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,

    /// A list of subscribed channels
    pub result: Vec<String>,
}

/// JSON-RPC 2.0 request structure for WebSocket communication
#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcRequest<T> {
    /// JSON-RPC version, always "2.0"
    pub jsonrpc: String,
    /// Request ID for tracking
    pub id: i32,
    /// Method name
    pub method: String,
    /// Parameters for the method
    pub params: T,
}

impl JsonRpcRequest<SubscribeRequest> {
    /// Create a new subscribe request with the given channels
    pub fn new_subscribe(id: i32, channels: Vec<String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: "public/subscribe".to_string(),
            params: SubscribeRequest { channels },
        }
    }
}

impl<T> JsonRpcRequest<T> {
    /// Create a new JSON-RPC request with arbitrary method and params
    pub fn new(id: i32, method: String, params: T) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method,
            params,
        }
    }
}

impl DeribitWebSocketClient {
    /// Send a subscribe request and wait for the response
    pub async fn subscribe(&mut self, channels: Vec<String>) -> Result<SubscribeResponse, DeribitWebSocketError> {
        if !self.is_connected() {
            return Err(DeribitWebSocketError::Connection(
                "Not connected".to_string(),
            ));
        }
        let req_id = self.next_request_id() as i32;
        let req = JsonRpcRequest::new_subscribe(req_id, channels);
        let msg = serde_json::to_string(&req)?;
        if let Some(ws) = &mut self.websocket {
            ws.send(tokio_tungstenite::tungstenite::Message::Text(msg.into()))
                .await
                .map_err(|e| DeribitWebSocketError::Connection(e.to_string()))?;
            // Wait for the response
            let response_str = self.receive_response().await?;
            let response: SubscribeResponse = serde_json::from_str(&response_str)?;
            Ok(response)
        } else {
            Err(DeribitWebSocketError::Connection(
                "WebSocket not connected".to_string(),
            ))
        }
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
        let request = JsonRpcRequest::new_subscribe(1, channels);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["jsonrpc"], "2.0");
        assert_eq!(parsed["id"], 1);
        assert_eq!(parsed["method"], "public/subscribe");
        assert_eq!(parsed["params"]["channels"][0], "book.BTC-PERPETUAL.100ms");
        assert_eq!(parsed["params"]["channels"][1], "trades.BTC-PERPETUAL");
    }

    #[test]
    fn test_subscribe_request_single_channel() {
        let channels = vec!["ticker.BTC-PERPETUAL".to_string()];
        let request = JsonRpcRequest::new_subscribe(42, channels);

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"jsonrpc":"2.0","id":42,"method":"public/subscribe","params":{"channels":["ticker.BTC-PERPETUAL"]}}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn test_subscribe_response_deserialization() {
        let response_json = r#"{"id":1,"jsonrpc":"2.0","result":["book.BTC-PERPETUAL.100ms","trades.BTC-PERPETUAL"]}"#;

        let response: SubscribeResponse = serde_json::from_str(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.len(), 2);
        assert_eq!(response.result[0], "book.BTC-PERPETUAL.100ms");
        assert_eq!(response.result[1], "trades.BTC-PERPETUAL");
    }

    #[test]
    fn test_subscribe_response_single_channel() {
        let response_json = r#"{"id":123,"jsonrpc":"2.0","result":["ticker.ETH-PERPETUAL"]}"#;

        let response: SubscribeResponse = serde_json::from_str(response_json).unwrap();

        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
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
    fn test_json_rpc_request_structure() {
        let channels = vec!["test.channel".to_string()];
        let request = JsonRpcRequest::new_subscribe(99, channels.clone());

        assert_eq!(request.id, 99);
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "public/subscribe");
        assert_eq!(request.params.channels, channels);
    }

    #[test]
    fn test_subscribe_request_empty_channels() {
        let channels: Vec<String> = vec![];
        let request = JsonRpcRequest::new_subscribe(1, channels);

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"jsonrpc":"2.0","id":1,"method":"public/subscribe","params":{"channels":[]}}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn test_subscribe_response_empty_result() {
        let response_json = r#"{"id":1,"jsonrpc":"2.0","result":[]}"#;

        let response: SubscribeResponse = serde_json::from_str(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.len(), 0);
    }
}
