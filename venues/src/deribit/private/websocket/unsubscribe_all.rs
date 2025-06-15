//! Implementation of the `/private/unsubscribe_all` endpoint
//!
//! This endpoint unsubscribes from all channels that have been subscribed to.
//! It is only available via websockets and requires authentication.

use serde::{Deserialize, Serialize};

/// Request for the unsubscribe_all endpoint
/// 
/// This method takes no parameters as per the API documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeAllRequest {
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Request ID for correlation
    pub id: u64,
    /// Method name
    pub method: String,
}

impl Default for UnsubscribeAllRequest {
    fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "private/unsubscribe_all".to_string(),
        }
    }
}

impl UnsubscribeAllRequest {
    /// Create a new unsubscribe_all request with the specified ID
    pub fn new(id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: "private/unsubscribe_all".to_string(),
        }
    }
}

/// Response from the unsubscribe_all endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeAllResponse {
    /// The ID that was sent in the request
    pub id: u64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Result of method execution. "ok" in case of success
    pub result: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_unsubscribe_all_request_serialization() {
        let request = UnsubscribeAllRequest::new(42);
        
        let json = serde_json::to_string(&request).unwrap_or_else(|_| "".to_string());
        let expected = r#"{"jsonrpc":"2.0","id":42,"method":"private/unsubscribe_all"}"#;
        
        assert_eq!(json, expected);
    }

    #[test]
    fn test_unsubscribe_all_request_default() {
        let request = UnsubscribeAllRequest::default();
        
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.id, 1);
        assert_eq!(request.method, "private/unsubscribe_all");
    }

    #[test]
    fn test_unsubscribe_all_response_deserialization() {
        let json = r#"{"id":42,"jsonrpc":"2.0","result":"ok"}"#;
        
        let response: Result<UnsubscribeAllResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok());
        
        if let Ok(response) = response {
            assert_eq!(response.id, 42);
            assert_eq!(response.jsonrpc, "2.0");
            assert_eq!(response.result, "ok");
        }
    }

    #[test]
    fn test_unsubscribe_all_response_serialization() {
        let response = UnsubscribeAllResponse {
            id: 123,
            jsonrpc: "2.0".to_string(),
            result: "ok".to_string(),
        };
        
        let json = serde_json::to_string(&response).unwrap_or_else(|_| "".to_string());
        let expected = r#"{"id":123,"jsonrpc":"2.0","result":"ok"}"#;
        
        assert_eq!(json, expected);
    }
}