use serde::{Deserialize, Serialize};
use websockets::VenueMessage;

/// Represents all possible messages that can be received from Deribit WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeribitMessage {
    /// JSON-RPC request message (for server-initiated messages)
    /// Must come first since it has the required "method" field
    JsonRpcRequest(JsonRpcRequest),
    /// JSON-RPC response message  
    JsonRpcResponse(JsonRpcResponse),
}

impl VenueMessage for DeribitMessage {}

/// JSON-RPC request structure used by Deribit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version, always "2.0"
    pub jsonrpc: String,
    /// Request ID for matching request/response pairs
    pub id: i64,
    /// Method name to call
    pub method: String,
    /// Method parameters (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

/// JSON-RPC response structure used by Deribit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version, always "2.0"
    pub jsonrpc: String,
    /// Request ID that this response corresponds to
    pub id: i64,
    /// Result of the method call (present on success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Error information (present on failure)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC error structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Additional error data (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Request for the disable_heartbeat method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisableHeartbeatRequest {
    /// JSON-RPC version, always "2.0"
    pub jsonrpc: String,
    /// Request ID for matching request/response pairs
    pub id: i64,
    /// Method name, always "public/disable_heartbeat"
    pub method: String,
}

impl DisableHeartbeatRequest {
    /// Create a new disable_heartbeat request
    pub fn new(id: i64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: "public/disable_heartbeat".to_string(),
        }
    }
}

/// Response for the disable_heartbeat method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisableHeartbeatResponse {
    /// JSON-RPC version, always "2.0"
    pub jsonrpc: String,
    /// Request ID that this response corresponds to
    pub id: i64,
    /// Result of the method call, "ok" on success
    pub result: String,
}

impl DisableHeartbeatResponse {
    /// Check if the response indicates success
    pub fn is_success(&self) -> bool {
        self.result == "ok"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disable_heartbeat_request_creation() {
        let request = DisableHeartbeatRequest::new(123);
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.id, 123);
        assert_eq!(request.method, "public/disable_heartbeat");
    }

    #[test]
    fn test_disable_heartbeat_request_serialization() {
        let request = DisableHeartbeatRequest::new(456);
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let expected = r#"{"jsonrpc":"2.0","id":456,"method":"public/disable_heartbeat"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_disable_heartbeat_response_deserialization() {
        let json = r#"{"jsonrpc":"2.0","id":123,"result":"ok"}"#;
        let response: DisableHeartbeatResponse = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 123);
        assert_eq!(response.result, "ok");
        assert!(response.is_success());
    }

    #[test]
    fn test_json_rpc_error_deserialization() {
        let json = r#"{"jsonrpc":"2.0","id":123,"error":{"code":-32601,"message":"Method not found"}}"#;
        let response: JsonRpcResponse = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 123);
        assert!(response.result.is_none());
        assert!(response.error.is_some());
        
        let error = response.error.unwrap();
        assert_eq!(error.code, -32601);
        assert_eq!(error.message, "Method not found");
    }

    #[test]
    fn test_deribit_message_enum() {
        // Test response variant
        let response_json = r#"{"jsonrpc":"2.0","id":123,"result":"ok"}"#;
        let message: DeribitMessage = serde_json::from_str(response_json).expect("Failed to deserialize");
        if let DeribitMessage::JsonRpcResponse(response) = message {
            assert_eq!(response.id, 123);
            assert!(response.result.is_some());
        } else {
            panic!("Expected JsonRpcResponse variant");
        }

        // Test request variant - Need to ensure this looks like a request not a response
        let request_json = r#"{"jsonrpc":"2.0","id":456,"method":"public/test"}"#;
        let message: DeribitMessage = serde_json::from_str(request_json).expect("Failed to deserialize");
        if let DeribitMessage::JsonRpcRequest(request) = message {
            assert_eq!(request.id, 456);
            assert_eq!(request.method, "public/test");
        } else {
            panic!("Expected JsonRpcRequest variant");
        }
    }
}