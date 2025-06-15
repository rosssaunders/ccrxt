//! JSON-RPC structures for Deribit API

use serde::{Deserialize, Serialize};

/// JSON-RPC request structure for Deribit API
#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcRequest<T> {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Unique request identifier
    pub id: u64,
    /// API method name
    pub method: String,
    /// Request parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<T>,
}

impl<T> JsonRpcRequest<T> {
    /// Create a new JSON-RPC request
    pub fn new(id: u64, method: String, params: Option<T>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method,
            params,
        }
    }
}

/// JSON-RPC response structure for Deribit API
#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcResponse<T> {
    /// JSON-RPC version (should be "2.0")
    pub jsonrpc: String,
    /// Request identifier (matches the request)
    pub id: u64,
    /// Result data (present on success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    /// Error data (present on error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC error structure
#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl<T> JsonRpcResponse<T> {
    /// Check if the response contains an error
    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }

    /// Get the result or return an error
    pub fn into_result(self) -> Result<T, crate::deribit::DeribitError> {
        if let Some(error) = self.error {
            return Err(crate::deribit::DeribitError::ApiError {
                code: error.code,
                message: error.message,
            });
        }

        self.result.ok_or_else(|| {
            crate::deribit::DeribitError::Error("Response missing both result and error".to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_jsonrpc_request_serialization() {
        let request = JsonRpcRequest::new(1, "public/status".to_string(), None::<()>);
        let json = serde_json::to_value(&request).unwrap();
        
        assert_eq!(json["jsonrpc"], "2.0");
        assert_eq!(json["id"], 1);
        assert_eq!(json["method"], "public/status");
        assert!(json.get("params").is_none());
    }

    #[test]
    fn test_jsonrpc_request_with_params() {
        #[derive(Serialize)]
        struct TestParams {
            test: String,
        }
        
        let params = TestParams { test: "value".to_string() };
        let request = JsonRpcRequest::new(2, "test/method".to_string(), Some(params));
        let json = serde_json::to_value(&request).unwrap();
        
        assert_eq!(json["jsonrpc"], "2.0");
        assert_eq!(json["id"], 2);
        assert_eq!(json["method"], "test/method");
        assert_eq!(json["params"]["test"], "value");
    }

    #[test]
    fn test_jsonrpc_response_success() {
        let response_json = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {"status": "ok"}
        });

        let response: JsonRpcResponse<serde_json::Value> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 1);
        assert!(response.result.is_some());
        assert!(response.error.is_none());
        assert!(!response.is_error());
    }

    #[test]
    fn test_jsonrpc_response_error() {
        let response_json = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32601,
                "message": "Method not found"
            }
        });

        let response: JsonRpcResponse<serde_json::Value> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 1);
        assert!(response.result.is_none());
        assert!(response.error.is_some());
        assert!(response.is_error());
        
        let error = response.error.unwrap();
        assert_eq!(error.code, -32601);
        assert_eq!(error.message, "Method not found");
    }

    #[test]
    fn test_jsonrpc_response_into_result_success() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct TestResult {
            value: String,
        }
        
        let response = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: 1,
            result: Some(TestResult { value: "test".to_string() }),
            error: None,
        };

        let result = response.into_result().unwrap();
        assert_eq!(result.value, "test");
    }

    #[test]
    fn test_jsonrpc_response_into_result_error() {
        let response: JsonRpcResponse<serde_json::Value> = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: 1,
            result: None,
            error: Some(JsonRpcError {
                code: -32601,
                message: "Method not found".to_string(),
                data: None,
            }),
        };

        let result = response.into_result();
        assert!(result.is_err());
        
        match result.unwrap_err() {
            crate::deribit::DeribitError::ApiError { code, message } => {
                assert_eq!(code, -32601);
                assert_eq!(message, "Method not found");
            }
            _ => panic!("Expected ApiError"),
        }
    }
}