//! Request and response structures for public/hello WebSocket endpoint
//!
//! This method is used to introduce the client software connected to Deribit platform over
//! websocket. Provided data may have an impact on the maintained connection and
//! will be collected for internal statistical purposes.
use crate::deribit::public::websocket::client::{DeribitWebSocketClient, DeribitWebSocketError};

use futures::SinkExt;
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
#[derive(Debug, Clone, Deserialize, Default)]
pub struct HelloResponse {
    /// The id that was sent in the request
    pub id: i32,

    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,

    /// Result object containing API version information
    pub result: HelloResult,
}

#[derive(Debug, Clone, Deserialize, Default)]
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

impl<T> JsonRpcRequest<T> {
    /// Create a new JSON-RPC request with arbitrary method and params
    pub fn new(id: i32, method: String, params: T) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method,
            params,
        }
    }
}

impl DeribitWebSocketClient {
    /// Send a hello message to introduce the client
    pub async fn send_hello(&mut self, client_name: String, client_version: String) -> Result<HelloResponse, DeribitWebSocketError> {
        if !self.is_connected() {
            return Err(DeribitWebSocketError::Connection(
                "Not connected".to_string(),
            ));
        }
        let req_id = self.next_request_id() as i32;
        let hello_req = JsonRpcRequest::new_hello(req_id, client_name, client_version);
        let req_json = serde_json::to_string(&hello_req)?;
        if let Some(ws) = &mut self.websocket {
            ws.send(tokio_tungstenite::tungstenite::Message::Text(
                req_json.into(),
            ))
            .await
            .map_err(|e| DeribitWebSocketError::Connection(e.to_string()))?;
            // Wait for the response
            let response_str = self.receive_response().await?;
            let response: HelloResponse = serde_json::from_str(&response_str)?;
            Ok(response)
        } else {
            Err(DeribitWebSocketError::Connection(
                "WebSocket not connected".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_hello_request_serialization() {
        let request = JsonRpcRequest::new_hello(1, "test_client".to_string(), "1.0.0".to_string());

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
        let request = JsonRpcRequest::new_hello(42, "rust_client".to_string(), "0.1.0".to_string());

        assert_eq!(request.id, 42);
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "public/hello");
        assert_eq!(request.params.client_name, "rust_client");
        assert_eq!(request.params.client_version, "0.1.0");
    }
}
