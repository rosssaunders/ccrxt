use serde::{Deserialize, Serialize};

/// Generic JSON-RPC 2.0 result structure for Deribit REST API responses
#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcResult<T> {
    /// The id that was sent in the request
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0)
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result object containing the actual response data
    #[serde(rename = "result")]
    pub result: T,
}

/// Wrapper for all Deribit WebSocket messages following JSON-RPC 2.0 protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeribitMessage {
    /// JSON-RPC request
    Request(JsonRpcRequest),
    /// JSON-RPC response
    Response(JsonRpcResponse),
}

impl websockets::VenueMessage for DeribitMessage {}

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

impl JsonRpcRequest {
    /// Create a new disable_heartbeat request
    pub fn disable_heartbeat(id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: "public/disable_heartbeat".to_string(),
            params: None,
        }
    }

    /// Create a new private trading request (e.g., private/buy, private/sell, etc.)
    pub fn private_trading<M: Into<String>, P: serde::Serialize>(
        id: u64,
        method: M,
        params: P,
    ) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: method.into(),
            params: serde_json::to_value(params).ok(),
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
        let json =
            r#"{"jsonrpc":"2.0","id":123,"error":{"code":-32601,"message":"Method not found"}}"#;
        let response: JsonRpcResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 123);
        assert!(!response.is_success());
        assert!(response.error.is_some());

        let error = response.error.unwrap();
        assert_eq!(error.code, -32601);
        assert_eq!(error.message, "Method not found");
    }
}
