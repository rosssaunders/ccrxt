//! Deribit WebSocket message types
//!
//! This module defines the JSON-RPC 2.0 message format used by Deribit's WebSocket API

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use websockets::VenueMessage;

/// A Deribit WebSocket message implementing the VenueMessage trait
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeribitMessage {
    /// JSON-RPC 2.0 request message
    Request(JsonRpcRequest),
    /// JSON-RPC 2.0 response message  
    Response(JsonRpcResponse),
    /// JSON-RPC 2.0 error response
    Error(JsonRpcErrorResponse),
    /// Subscription notification
    Notification(JsonRpcNotification),
}

impl VenueMessage for DeribitMessage {}

impl fmt::Display for DeribitMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeribitMessage::Request(req) => write!(f, "Request: {}", req.method),
            DeribitMessage::Response(resp) => write!(f, "Response: id={}", resp.id),
            DeribitMessage::Error(err) => write!(f, "Error: {}", err.error.message),
            DeribitMessage::Notification(notif) => write!(f, "Notification: {}", notif.method),
        }
    }
}

/// JSON-RPC 2.0 request message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version, always "2.0"
    pub jsonrpc: String,
    /// Request ID for matching responses
    pub id: i64,
    /// Method name to call
    pub method: String,
    /// Method parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl JsonRpcRequest {
    /// Create a new JSON-RPC request
    pub fn new(id: i64, method: impl Into<String>, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: method.into(),
            params,
        }
    }
}

/// JSON-RPC 2.0 response message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version, always "2.0"
    pub jsonrpc: String,
    /// Request ID that this response corresponds to
    pub id: i64,
    /// Response result
    pub result: Value,
}

/// JSON-RPC 2.0 error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcErrorResponse {
    /// JSON-RPC version, always "2.0"
    pub jsonrpc: String,
    /// Request ID that this error corresponds to
    pub id: Option<i64>,
    /// Error details
    pub error: JsonRpcError,
}

/// JSON-RPC 2.0 error object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// JSON-RPC 2.0 notification (no response expected)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    /// JSON-RPC version, always "2.0"
    pub jsonrpc: String,
    /// Method name
    pub method: String,
    /// Method parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_json_rpc_request_serialization() {
        let request = JsonRpcRequest::new(
            1,
            "public/unsubscribe",
            Some(json!({"channels": ["deribit_price_index.btc_usd"]})),
        );

        let serialized = serde_json::to_string(&request).expect("Failed to serialize request");
        assert!(serialized.contains("\"jsonrpc\":\"2.0\""));
        assert!(serialized.contains("\"id\":1"));
        assert!(serialized.contains("\"method\":\"public/unsubscribe\""));
        assert!(serialized.contains("\"channels\""));
    }

    #[test]
    fn test_json_rpc_request_without_params() {
        let request = JsonRpcRequest::new(2, "public/get_time", None);
        
        let serialized = serde_json::to_string(&request).expect("Failed to serialize request");
        assert!(serialized.contains("\"jsonrpc\":\"2.0\""));
        assert!(serialized.contains("\"id\":2"));
        assert!(serialized.contains("\"method\":\"public/get_time\""));
        assert!(!serialized.contains("\"params\""));
    }

    #[test]
    fn test_json_rpc_response_deserialization() {
        let response_json = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": ["deribit_price_index.btc_usd"]
        });

        let response: JsonRpcResponse = serde_json::from_value(response_json)
            .expect("Failed to deserialize response");
        
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 1);
        assert!(response.result.is_array());
    }

    #[test]
    fn test_json_rpc_error_deserialization() {
        let error_json = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32602,
                "message": "Invalid params"
            }
        });

        let error: JsonRpcErrorResponse = serde_json::from_value(error_json)
            .expect("Failed to deserialize error");
        
        assert_eq!(error.jsonrpc, "2.0");
        assert_eq!(error.id, Some(1));
        assert_eq!(error.error.code, -32602);
        assert_eq!(error.error.message, "Invalid params");
    }

    #[test]
    fn test_deribit_message_enum() {
        let request = DeribitMessage::Request(JsonRpcRequest::new(
            1,
            "public/unsubscribe",
            Some(json!({"channels": ["test"]})),
        ));

        match request {
            DeribitMessage::Request(req) => {
                assert_eq!(req.method, "public/unsubscribe");
                assert_eq!(req.id, 1);
            }
            _ => panic!("Expected request message"),
        }
    }

    #[test]
    fn test_deribit_message_display() {
        let request = DeribitMessage::Request(JsonRpcRequest::new(
            1,
            "public/unsubscribe",
            None,
        ));
        
        let display_str = format!("{}", request);
        assert!(display_str.contains("Request: public/unsubscribe"));
    }
}