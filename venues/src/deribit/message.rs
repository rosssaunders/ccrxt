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

/// Request parameters for unsubscribe method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeParams {
    /// A list of channels to unsubscribe from
    pub channels: Vec<String>,
}

/// Request for unsubscribe method with channels parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeRequest {
    /// Always "2.0"
    pub jsonrpc: String,
    /// Request ID
    pub id: u64,
    /// Always "public/unsubscribe"
    pub method: String,
    /// Parameters containing channels to unsubscribe from
    pub params: UnsubscribeParams,
}

impl UnsubscribeRequest {
    /// Create a new unsubscribe request
    pub fn new(id: u64, channels: Vec<String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: "public/unsubscribe".to_string(),
            params: UnsubscribeParams { channels },
        }
    }
}

/// Response for unsubscribe method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeResponse {
    /// Always "2.0"
    pub jsonrpc: String,
    /// Request ID from the original request
    pub id: u64,
    /// Result - array of subscribed channels
    pub result: Vec<String>,
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

    /// Create a new unsubscribe request with channels
    pub fn unsubscribe(id: u64, channels: Vec<String>) -> Self {
        let params = UnsubscribeParams { channels };
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: "public/unsubscribe".to_string(),
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
    fn test_unsubscribe_request_serialization() {
        let channels = vec!["ticker.BTC-PERPETUAL".to_string(), "trades.ETH-PERPETUAL".to_string()];
        let request = UnsubscribeRequest::new(123, channels.clone());
        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"id\":123"));
        assert!(json.contains("\"method\":\"public/unsubscribe\""));
        assert!(json.contains("ticker.BTC-PERPETUAL"));
        assert!(json.contains("trades.ETH-PERPETUAL"));
    }

    #[test]
    fn test_unsubscribe_response_deserialization() {
        let json = r#"{"jsonrpc":"2.0","id":123,"result":["ticker.BTC-PERPETUAL","trades.ETH-PERPETUAL"]}"#;
        let response: UnsubscribeResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 123);
        assert_eq!(response.result.len(), 2);
        assert!(response.result.contains(&"ticker.BTC-PERPETUAL".to_string()));
        assert!(response.result.contains(&"trades.ETH-PERPETUAL".to_string()));
    }

    #[test]
    fn test_json_rpc_request_unsubscribe() {
        let channels = vec!["ticker.BTC-PERPETUAL".to_string()];
        let request = JsonRpcRequest::unsubscribe(456, channels);

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.id, 456);
        assert_eq!(request.method, "public/unsubscribe");
        assert!(request.params.is_some());
        
        // Verify the params contain the channels
        if let Some(params) = request.params {
            let params_obj: UnsubscribeParams = serde_json::from_value(params).unwrap();
            assert_eq!(params_obj.channels.len(), 1);
            assert_eq!(params_obj.channels[0], "ticker.BTC-PERPETUAL");
        }
    }

    #[test] 
    fn test_unsubscribe_end_to_end_serialization() {
        // Test full round-trip serialization for unsubscribe request
        let channels = vec![
            "ticker.BTC-PERPETUAL".to_string(), 
            "trades.ETH-PERPETUAL".to_string(),
            "book.SOL-PERPETUAL.100ms".to_string()
        ];
        let request = JsonRpcRequest::unsubscribe(789, channels.clone());
        
        // Serialize to JSON
        let json = serde_json::to_string(&request).unwrap();
        
        // Verify JSON contains expected content
        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"id\":789"));
        assert!(json.contains("\"method\":\"public/unsubscribe\""));
        assert!(json.contains("ticker.BTC-PERPETUAL"));
        assert!(json.contains("trades.ETH-PERPETUAL"));
        assert!(json.contains("book.SOL-PERPETUAL.100ms"));
        
        // Deserialize back and verify
        let parsed: JsonRpcRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.jsonrpc, "2.0");
        assert_eq!(parsed.id, 789);
        assert_eq!(parsed.method, "public/unsubscribe");
        
        // Extract and verify params
        if let Some(params) = parsed.params {
            let params_obj: UnsubscribeParams = serde_json::from_value(params).unwrap();
            assert_eq!(params_obj.channels.len(), 3);
            assert!(params_obj.channels.contains(&"ticker.BTC-PERPETUAL".to_string()));
            assert!(params_obj.channels.contains(&"trades.ETH-PERPETUAL".to_string()));
            assert!(params_obj.channels.contains(&"book.SOL-PERPETUAL.100ms".to_string()));
        }
    }

    #[test]
    fn test_unsubscribe_empty_channels() {
        // Test unsubscribe with empty channels list
        let channels = vec![];
        let request = JsonRpcRequest::unsubscribe(100, channels);
        
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.id, 100);
        assert_eq!(request.method, "public/unsubscribe");
        assert!(request.params.is_some());
        
        // Verify params contain empty channels array
        if let Some(params) = request.params {
            let params_obj: UnsubscribeParams = serde_json::from_value(params).unwrap();
            assert_eq!(params_obj.channels.len(), 0);
        }
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
