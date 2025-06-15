//! Request and response structures for public/hello WebSocket endpoint
//!
//! This method is used to introduce the client software connected to Deribit platform over
//! websocket. Provided data may have an impact on the maintained connection and
//! will be collected for internal statistical purposes.

use serde::{Deserialize, Serialize};

/// Request parameters for the public/hello endpoint.
///
/// This method is used to introduce the client software connected to Deribit platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloRequest {
    /// Client software name
    pub client_name: String,
    /// Client software version
    pub client_version: String,
}

/// Response for public/hello endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct HelloResponse {
    /// The id that was sent in the request
    pub id: i32,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Result object containing API version information
    pub result: HelloResult,
}

/// Result data for hello response.
#[derive(Debug, Clone, Deserialize)]
pub struct HelloResult {
    /// The API version
    pub version: String,
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

impl JsonRpcRequest<HelloRequest> {
    /// Create a new hello request with the given parameters
    pub fn new_hello(id: i32, client_name: String, client_version: String) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: "public/hello".to_string(),
            params: HelloRequest {
                client_name,
                client_version,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_hello_request_serialization() {
        let request = JsonRpcRequest::new_hello(
            1,
            "test_client".to_string(),
            "1.0.0".to_string(),
        );

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"jsonrpc":"2.0","id":1,"method":"public/hello","params":{"client_name":"test_client","client_version":"1.0.0"}}"#;
        
        assert_eq!(json, expected);
    }

    #[test]
    fn test_hello_response_deserialization() {
        let response_json = r#"{"id":1,"jsonrpc":"2.0","result":{"version":"1.2.26"}}"#;
        
        let response: HelloResponse = serde_json::from_str(response_json).unwrap();
        
        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.version, "1.2.26");
    }

    #[test]
    fn test_hello_request_structure() {
        let hello_req = HelloRequest {
            client_name: "my_client".to_string(),
            client_version: "2.1.0".to_string(),
        };

        assert_eq!(hello_req.client_name, "my_client");
        assert_eq!(hello_req.client_version, "2.1.0");
    }

    #[test]
    fn test_json_rpc_request_structure() {
        let request = JsonRpcRequest::new_hello(
            42,
            "rust_client".to_string(),
            "0.1.0".to_string(),
        );

        assert_eq!(request.id, 42);
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "public/hello");
        assert_eq!(request.params.client_name, "rust_client");
        assert_eq!(request.params.client_version, "0.1.0");
    }
}