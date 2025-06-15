use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};
use websockets::{BoxResult, VenueMessage, WebSocketConnection};
use rest::secrets::ExposableSecret;
use std::pin::Pin;
use futures::Stream;

/// Deribit WebSocket message type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeribitMessage {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request/Response ID
    pub id: Option<u64>,
    /// Method name for requests
    pub method: Option<String>,
    /// Parameters for requests
    pub params: Option<serde_json::Value>,
    /// Result for responses
    pub result: Option<serde_json::Value>,
    /// Error for error responses
    pub error: Option<DeribitError>,
}

impl VenueMessage for DeribitMessage {}

/// Deribit JSON-RPC error structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeribitError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

/// Request parameters for unsubscribe method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeParams {
    /// List of channels to unsubscribe from
    pub channels: Vec<String>,
}

/// Response result for unsubscribe method
pub type UnsubscribeResult = Vec<String>;

/// Private WebSocket client for Deribit
pub struct PrivateWebSocketClient {
    /// WebSocket stream
    ws_stream: Option<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    /// API key for authentication
    api_key: Box<dyn ExposableSecret>,
    /// API secret for authentication  
    api_secret: Box<dyn ExposableSecret>,
    /// Base WebSocket URL
    base_url: String,
    /// Request ID counter for JSON-RPC
    request_id: u64,
    /// Connection status
    is_connected: bool,
}

impl PrivateWebSocketClient {
    /// Create a new Deribit private WebSocket client
    ///
    /// # Arguments
    /// * `api_key` - The API key for authentication
    /// * `api_secret` - The API secret for authentication
    /// * `base_url` - The WebSocket base URL (default: "wss://www.deribit.com/ws/api/v2")
    pub fn new(
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
        base_url: Option<String>,
    ) -> Self {
        Self {
            ws_stream: None,
            api_key,
            api_secret,
            base_url: base_url.unwrap_or_else(|| "wss://www.deribit.com/ws/api/v2".to_string()),
            request_id: 0,
            is_connected: false,
        }
    }

    /// Get next request ID
    fn next_id(&mut self) -> u64 {
        self.request_id += 1;
        self.request_id
    }

    /// Authenticate with the Deribit WebSocket API
    async fn authenticate(&mut self) -> BoxResult<()> {
        // For Deribit, we typically use client_credentials method for authentication
        let auth_request = DeribitMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(self.next_id()),
            method: Some("public/auth".to_string()),
            params: Some(serde_json::json!({
                "grant_type": "client_credentials",
                "client_id": self.api_key.expose_secret(),
                "client_secret": self.api_secret.expose_secret()
            })),
            result: None,
            error: None,
        };

        if let Some(ref mut ws) = self.ws_stream {
            let message = serde_json::to_string(&auth_request)?;
            ws.send(Message::Text(message.into())).await?;
            
            // Wait for authentication response
            if let Some(msg) = ws.next().await {
                let msg = msg?;
                let response_text = msg.to_text()?.to_string();
                let _auth_response: DeribitMessage = serde_json::from_str(&response_text)?;
                // In a real implementation, you'd validate the auth response
            }
        }

        Ok(())
    }

    /// Unsubscribe from one or more channels
    ///
    /// # Arguments
    /// * `channels` - List of channels to unsubscribe from
    ///
    /// # Returns
    /// A list of remaining subscribed channels
    pub async fn unsubscribe(&mut self, channels: Vec<String>) -> BoxResult<UnsubscribeResult> {
        if !self.is_connected {
            return Err("WebSocket not connected".into());
        }

        let request = DeribitMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(self.next_id()),
            method: Some("private/unsubscribe".to_string()),
            params: Some(serde_json::to_value(UnsubscribeParams { channels })?),
            result: None,
            error: None,
        };

        if let Some(ref mut ws) = self.ws_stream {
            let message = serde_json::to_string(&request)?;
            ws.send(Message::Text(message.into())).await?;

            // Wait for response
            if let Some(msg) = ws.next().await {
                let msg = msg?;
                let response_text = msg.to_text()?.to_string();
                let response: DeribitMessage = serde_json::from_str(&response_text)?;
                
                if let Some(error) = response.error {
                    return Err(format!("Deribit error {}: {}", error.code, error.message).into());
                }

                if let Some(result) = response.result {
                    let channels: UnsubscribeResult = serde_json::from_value(result)?;
                    return Ok(channels);
                }
            }
        }

        Err("Failed to get unsubscribe response".into())
    }
}

#[async_trait]
impl WebSocketConnection<DeribitMessage> for PrivateWebSocketClient {
    async fn connect(&mut self) -> BoxResult<()> {
        let (ws_stream, _response) = connect_async(&self.base_url).await?;
        self.ws_stream = Some(ws_stream);
        self.is_connected = true;
        
        // Authenticate after connecting
        self.authenticate().await?;
        
        Ok(())
    }

    async fn disconnect(&mut self) -> BoxResult<()> {
        if let Some(mut ws) = self.ws_stream.take() {
            ws.close(None).await?;
        }
        self.is_connected = false;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.is_connected
    }

    fn message_stream(&mut self) -> Pin<Box<dyn Stream<Item = BoxResult<DeribitMessage>> + Send>> {
        // Create a static empty stream since the WebSocket implementation needs to handle async lifetime issues
        // In a production implementation, you would need a more sophisticated approach to handle the stream
        Box::pin(futures::stream::empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rest::secrets::ExposableSecret;

    #[derive(Debug)]
    struct TestSecret {
        secret: String,
    }

    impl TestSecret {
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    impl ExposableSecret for TestSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    #[test]
    fn test_deribit_message_serialization() {
        let message = DeribitMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(1),
            method: Some("private/unsubscribe".to_string()),
            params: Some(serde_json::json!({
                "channels": ["user.orders.BTC-PERPETUAL.raw"]
            })),
            result: None,
            error: None,
        };

        let serialized = serde_json::to_string(&message).expect("Should serialize");
        let deserialized: DeribitMessage = serde_json::from_str(&serialized).expect("Should deserialize");
        
        assert_eq!(deserialized.jsonrpc, "2.0");
        assert_eq!(deserialized.id, Some(1));
        assert_eq!(deserialized.method, Some("private/unsubscribe".to_string()));
    }

    #[test]
    fn test_unsubscribe_params_serialization() {
        let params = UnsubscribeParams {
            channels: vec!["user.orders.BTC-PERPETUAL.raw".to_string(), "user.trades.BTC-PERPETUAL.raw".to_string()],
        };

        let serialized = serde_json::to_value(&params).expect("Should serialize");
        let expected = serde_json::json!({
            "channels": ["user.orders.BTC-PERPETUAL.raw", "user.trades.BTC-PERPETUAL.raw"]
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_client_creation() {
        let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        
        let client = PrivateWebSocketClient::new(api_key, api_secret, None);
        
        assert!(!client.is_connected());
        assert_eq!(client.base_url, "wss://www.deribit.com/ws/api/v2");
    }

    #[test]
    fn test_client_creation_with_custom_url() {
        let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let custom_url = "wss://test.deribit.com/ws/api/v2".to_string();
        
        let client = PrivateWebSocketClient::new(api_key, api_secret, Some(custom_url.clone()));
        
        assert_eq!(client.base_url, custom_url);
    }

    #[test]
    fn test_deribit_error_structure() {
        let error = DeribitError {
            code: 10001,
            message: "Invalid request".to_string(),
            data: Some(serde_json::json!({"details": "Channel not found"})),
        };

        let serialized = serde_json::to_string(&error).expect("Should serialize");
        let deserialized: DeribitError = serde_json::from_str(&serialized).expect("Should deserialize");

        assert_eq!(deserialized.code, 10001);
        assert_eq!(deserialized.message, "Invalid request");
        assert!(deserialized.data.is_some());
    }
}