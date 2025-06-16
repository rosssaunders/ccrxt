use serde::{Deserialize, Serialize};
use websockets::VenueMessage;

/// Wrapper for all Deribit WebSocket messages following JSON-RPC 2.0 protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeribitMessage {
    /// JSON-RPC request
    Request(JsonRpcRequest),
    /// JSON-RPC response
    Response(JsonRpcResponse),
}

impl VenueMessage for DeribitMessage {}

/// JSON-RPC 2.0 request structure for Deribit API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request ID for matching responses
    pub id: u64,
    /// Method name (e.g., "public/unsubscribe_all")
    pub method: String,
    /// Method parameters (empty for unsubscribe_all)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

/// JSON-RPC 2.0 response structure for Deribit API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request ID from the original request
    pub id: u64,
    /// Result on success
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Error on failure
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
    /// Additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Request for unsubscribe_all method (no parameters)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeAllRequest {
    /// Always "2.0"
    pub jsonrpc: String,
    /// Request ID
    pub id: u64,
    /// Always "public/unsubscribe_all"
    pub method: String,
}

impl UnsubscribeAllRequest {
    /// Create a new unsubscribe_all request
    pub fn new(id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: "public/unsubscribe_all".to_string(),
        }
    }
}

/// Response for unsubscribe_all method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeAllResponse {
    /// Always "2.0"
    pub jsonrpc: String,
    /// Request ID from the original request
    pub id: u64,
    /// Result - "ok" on success
    pub result: String,
}

impl JsonRpcRequest {
    /// Create a new unsubscribe_all request
    pub fn unsubscribe_all(id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: "public/unsubscribe_all".to_string(),
            params: None,
        }
    }

    /// Create a new disable_heartbeat request
    pub fn disable_heartbeat(id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: "public/disable_heartbeat".to_string(),
            params: None,
        }
    }
}

impl JsonRpcResponse {
    /// Check if the response indicates success
    pub fn is_success(&self) -> bool {
        self.error.is_none()
    }

    /// Get the result as a string (typically "ok" for unsubscribe_all)
    pub fn result_as_string(&self) -> Option<String> {
        self.result
            .as_ref()
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_unsubscribe_all_request_serialization() {
        let request = UnsubscribeAllRequest::new(123);
        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"id\":123"));
        assert!(json.contains("\"method\":\"public/unsubscribe_all\""));
    }

    #[test]
    fn test_json_rpc_request_unsubscribe_all() {
        let request = JsonRpcRequest::unsubscribe_all(456);

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.id, 456);
        assert_eq!(request.method, "public/unsubscribe_all");
        assert!(request.params.is_none());
    }

    #[test]
    fn test_json_rpc_request_disable_heartbeat() {
        let request = JsonRpcRequest::disable_heartbeat(789);

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.id, 789);
        assert_eq!(request.method, "public/disable_heartbeat");
        assert!(request.params.is_none());
    }

    #[test]
    fn test_json_rpc_response_deserialization() {
        let json = r#"{"jsonrpc":"2.0","id":123,"result":"ok"}"#;
        let response: JsonRpcResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 123);
        assert!(response.is_success());
        assert_eq!(response.result_as_string(), Some("ok".to_string()));
    }

    #[test]
    fn test_json_rpc_error_response() {
        let json = r#"{"jsonrpc":"2.0","id":123,"error":{"code":-32601,"message":"Method not found"}}"#;
        let response: JsonRpcResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 123);
        assert!(!response.is_success());
        assert!(response.error.is_some());

        let error = response.error.unwrap();
        assert_eq!(error.code, -32601);
        assert_eq!(error.message, "Method not found");
    }

    #[test]
    fn test_deribit_message_enum() {
        let request = JsonRpcRequest::unsubscribe_all(789);
        let message = DeribitMessage::Request(request);

        let json = serde_json::to_string(&message).unwrap();
        let parsed: DeribitMessage = serde_json::from_str(&json).unwrap();

        match parsed {
            DeribitMessage::Request(req) => {
                assert_eq!(req.method, "public/unsubscribe_all");
                assert_eq!(req.id, 789);
            }
            _ => panic!("Expected Request variant"),
        }
    }
}
