//! Request and response structures for public/hello WebSocket endpoint
//!
//! This method is used to introduce the client software connected to Deribit platform over
//! websocket. Provided data may have an impact on the maintained connection and
//! will be collected for internal statistical purposes.
use crate::deribit::public::websocket::client::{DeribitWebSocketError, PrivateWebSocketClient};

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
    /// Result object containing API version information
    pub result: HelloResult,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct HelloResult {
    /// The API version
    pub version: String,
}

impl PrivateWebSocketClient {
    /// Send a hello message to introduce the client
    pub async fn send_hello(&mut self, hello_req: HelloRequest) -> Result<HelloResponse, DeribitWebSocketError> {
        self.send_serializable(&hello_req).await?;
        // Wait for the response
        let response_str = self.receive_response().await?;
        let response: HelloResponse = serde_json::from_str(&response_str)?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_hello_request_serialization() {
        let request = HelloRequest {
            client_name: "test_client".to_string(),
            client_version: "1.0.0".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"client_name":"test_client","client_version":"1.0.0"}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn test_hello_response_deserialization() {
        let response_json = r#"{"result":{"version":"1.2.26"}}"#;

        let response: HelloResponse = serde_json::from_str(response_json).unwrap();

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
}
