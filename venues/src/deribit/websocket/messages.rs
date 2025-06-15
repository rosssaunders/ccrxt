//! JSON-RPC 2.0 message structures for Deribit WebSocket API

use serde::{Deserialize, Serialize};

/// JSON-RPC 2.0 request structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcRequest<T> {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request method name
    pub method: String,
    /// Request parameters
    pub params: T,
    /// Request ID
    pub id: u64,
}

/// JSON-RPC 2.0 response structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcResponse<T> {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Response result (present on success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    /// Response error (present on failure)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    /// Request ID that this response corresponds to
    pub id: u64,
}

/// JSON-RPC 2.0 error structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Optional additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Parameters for public/subscribe method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscribeParams {
    /// List of channels to subscribe to
    pub channels: Vec<String>,
}

/// Result type for subscribe responses (list of subscribed channels)
pub type SubscribeResult = Vec<String>;

/// Public subscribe request type
pub type PublicSubscribeRequest = JsonRpcRequest<SubscribeParams>;

/// Public subscribe response type  
pub type PublicSubscribeResponse = JsonRpcResponse<SubscribeResult>;

impl<T> JsonRpcRequest<T> {
    /// Create a new JSON-RPC request
    pub fn new(method: String, params: T, id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method,
            params,
            id,
        }
    }
}

impl<T> JsonRpcResponse<T> {
    /// Create a successful JSON-RPC response
    pub fn success(result: T, id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    /// Create an error JSON-RPC response
    pub fn error(error: JsonRpcError, id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(error),
            id,
        }
    }
}

impl JsonRpcError {
    /// Create a new JSON-RPC error
    pub fn new(code: i32, message: String, data: Option<serde_json::Value>) -> Self {
        Self { code, message, data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_subscribe_request_serialization() {
        let params = SubscribeParams {
            channels: vec!["book.BTC-PERPETUAL.100ms".to_string()],
        };
        let request = PublicSubscribeRequest::new("public/subscribe".to_string(), params, 1);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let expected = r#"{"jsonrpc":"2.0","method":"public/subscribe","params":{"channels":["book.BTC-PERPETUAL.100ms"]},"id":1}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_subscribe_response_serialization() {
        let result = vec!["book.BTC-PERPETUAL.100ms".to_string()];
        let response = PublicSubscribeResponse::success(result, 1);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let expected = r#"{"jsonrpc":"2.0","result":["book.BTC-PERPETUAL.100ms"],"id":1}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_subscribe_request_deserialization() {
        let json = r#"{"jsonrpc":"2.0","method":"public/subscribe","params":{"channels":["book.BTC-PERPETUAL.100ms"]},"id":1}"#;
        let request: PublicSubscribeRequest = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "public/subscribe");
        assert_eq!(request.params.channels, vec!["book.BTC-PERPETUAL.100ms"]);
        assert_eq!(request.id, 1);
    }

    #[test]
    fn test_subscribe_response_deserialization() {
        let json = r#"{"jsonrpc":"2.0","result":["book.BTC-PERPETUAL.100ms"],"id":1}"#;
        let response: PublicSubscribeResponse = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, Some(vec!["book.BTC-PERPETUAL.100ms".to_string()]));
        assert_eq!(response.error, None);
        assert_eq!(response.id, 1);
    }

    #[test]
    fn test_error_response_serialization() {
        let error = JsonRpcError::new(-32602, "Invalid params".to_string(), None);
        let response = PublicSubscribeResponse::error(error, 1);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let expected = r#"{"jsonrpc":"2.0","error":{"code":-32602,"message":"Invalid params"},"id":1}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_multiple_channels_subscription() {
        let params = SubscribeParams {
            channels: vec![
                "book.BTC-PERPETUAL.100ms".to_string(),
                "ticker.BTC-PERPETUAL".to_string(),
                "trades.BTC-PERPETUAL".to_string(),
            ],
        };
        let request = PublicSubscribeRequest::new("public/subscribe".to_string(), params, 42);

        assert_eq!(request.params.channels.len(), 3);
        assert_eq!(request.id, 42);
        assert_eq!(request.method, "public/subscribe");
    }
}