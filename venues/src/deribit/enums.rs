use serde::{Deserialize, Serialize};

/// JSON-RPC 2.0 request structure for Deribit API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest<T> {
    pub jsonrpc: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<T>,
    pub id: u64,
}

impl<T> JsonRpcRequest<T> {
    pub fn new(method: String, params: Option<T>, id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method,
            params,
            id,
        }
    }
}

/// JSON-RPC 2.0 response structure for Deribit API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<crate::deribit::errors::JsonRpcError>,
}

/// Endpoint types for rate limiting categorization  
pub use crate::deribit::rate_limit::EndpointType;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_rpc_request_creation() {
        let request = JsonRpcRequest::new(
            "public/get_time".to_string(),
            None::<()>,
            1,
        );

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "public/get_time");
        assert_eq!(request.id, 1);
        assert!(request.params.is_none());
    }

    #[test]
    fn test_json_rpc_response_serialization() {
        let response = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: 1,
            result: Some(1234567890u64),
            error: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"result\":1234567890"));
    }

    #[test]
    fn test_json_rpc_error_response() {
        let error = crate::deribit::errors::JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        };

        let response = JsonRpcResponse::<()> {
            jsonrpc: "2.0".to_string(),
            id: 1,
            result: None,
            error: Some(error),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"error\""));
        assert!(json.contains("\"code\":-32602"));
    }
}