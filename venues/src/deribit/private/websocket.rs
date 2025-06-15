//! Deribit Private WebSocket API client
//!
//! This module provides WebSocket connectivity for Deribit's private API endpoints.
//! All private methods require authentication and use JSON-RPC 2.0 protocol.
//!
//! # Authentication
//!
//! Before calling private methods, you must authenticate using your API credentials.
//! The client supports client credentials authentication which is suitable for
//! automated trading applications.
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use venues::deribit::{WebSocketClient, SubscribeRequest};
//! use websockets::WebSocketConnection;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut client = WebSocketClient::new();
//!     
//!     // Connect to WebSocket
//!     client.connect().await?;
//!     
//!     // Authenticate with API credentials
//!     client.authenticate("your_client_id", "your_client_secret").await?;
//!     
//!     // Subscribe to private channels
//!     let channels = vec!["user.portfolio.btc".to_string()];
//!     let response = client.subscribe(channels, Some("trading".to_string())).await?;
//!     
//!     println!("Subscribed to channels: {:?}", response.channels);
//!     
//!     Ok(())
//! }
//! ```

use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream, MaybeTlsStream};
use websockets::{BoxResult, VenueMessage, WebSocketConnection};
use futures::Stream;

/// JSON-RPC 2.0 request structure for Deribit API
#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcRequest<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: T,
}

/// JSON-RPC 2.0 response structure for Deribit API
#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC error structure
#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

/// Request parameters for the /public/auth method
#[derive(Debug, Clone, Serialize)]
pub struct AuthRequest {
    /// The grant type, must be "client_credentials"
    pub grant_type: String,
    /// Your application's client ID
    pub client_id: String,
    /// Your application's client secret 
    pub client_secret: String,
}

/// Response for the /public/auth method
#[derive(Debug, Clone, Deserialize)]
pub struct AuthResponse {
    /// Access token for authenticated requests
    pub access_token: String,
    /// Token type, typically "bearer"
    pub token_type: String,
    /// Token expiration time in seconds
    pub expires_in: u64,
    /// Refresh token for getting new access tokens
    pub refresh_token: String,
    /// Scope of the token
    pub scope: String,
}

/// Request parameters for the /private/subscribe method
#[derive(Debug, Clone, Serialize)]
pub struct SubscribeRequest {
    /// A list of channels to subscribe to
    pub channels: Vec<String>,
    /// Optional label which will be added to notifications of private channels (max 16 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Access token for authentication (required for private methods)
    pub access_token: String,
}

/// Response for the /private/subscribe method
#[derive(Debug, Clone, Deserialize)]
pub struct SubscribeResponse {
    /// A list of subscribed channels
    pub channels: Vec<String>,
}

/// Deribit WebSocket message wrapper
#[derive(Debug, Clone)]
pub enum DeribitMessage {
    /// JSON-RPC response message
    Response(JsonRpcResponse<serde_json::Value>),
    /// Notification message (subscription data)
    Notification(serde_json::Value),
}

impl VenueMessage for DeribitMessage {}

/// WebSocket client for Deribit private API
pub struct WebSocketClient {
    /// WebSocket stream connection
    ws_stream: Option<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>,
    /// Connection status
    connected: bool,
    /// Base WebSocket URL for Deribit
    base_url: String,
    /// Current request ID counter for JSON-RPC
    request_id: u64,
}

impl WebSocketClient {
    /// Create a new WebSocket client for Deribit
    pub fn new() -> Self {
        Self {
            ws_stream: None,
            connected: false,
            base_url: "wss://www.deribit.com/ws/api/v2".to_string(),
            request_id: 1,
        }
    }

    /// Create a new WebSocket client with custom URL (for testing)
    pub fn with_url(url: String) -> Self {
        Self {
            ws_stream: None,
            connected: false,
            base_url: url,
            request_id: 1,
        }
    }

    /// Subscribe to one or more channels
    pub async fn subscribe(&mut self, channels: Vec<String>, label: Option<String>) -> BoxResult<SubscribeResponse> {
        // Validate label length if provided
        if let Some(ref lbl) = label {
            if lbl.len() > 16 {
                return Err("Label must be 16 characters or less".into());
            }
        }

        if !self.connected {
            return Err("Not connected to WebSocket".into());
        }

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: self.request_id,
            method: "private/subscribe".to_string(),
            params: SubscribeRequest { channels, label },
        };

        self.request_id = self.request_id.saturating_add(1);

        let message = serde_json::to_string(&request)?;
        
        if let Some(ref mut ws) = self.ws_stream {
            ws.send(Message::Text(message.into())).await?;
            
            // Wait for response
            while let Some(msg) = ws.next().await {
                let msg = msg?;
                if let Message::Text(text) = msg {
                    let response: JsonRpcResponse<SubscribeResponse> = serde_json::from_str(&text)?;
                    
                    if response.id == request.id {
                        if let Some(error) = response.error {
                            return Err(format!("JSON-RPC error {}: {}", error.code, error.message).into());
                        }
                        
                        if let Some(result) = response.result {
                            return Ok(result);
                        } else {
                            return Err("No result in response".into());
                        }
                    }
                }
            }
        }

        Err("Failed to get response".into())
    }


}

#[async_trait]
impl WebSocketConnection<DeribitMessage> for WebSocketClient {
    async fn connect(&mut self) -> BoxResult<()> {
        let (ws_stream, _) = connect_async(&self.base_url).await?;
        self.ws_stream = Some(ws_stream);
        self.connected = true;
        Ok(())
    }

    async fn disconnect(&mut self) -> BoxResult<()> {
        if let Some(mut ws) = self.ws_stream.take() {
            ws.close(None).await?;
        }
        self.connected = false;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    fn message_stream(&mut self) -> Pin<Box<dyn Stream<Item = BoxResult<DeribitMessage>> + Send>> {
        // Take ownership of the websocket stream to avoid lifetime issues
        let ws_stream = self.ws_stream.take();
        if ws_stream.is_some() {
            self.connected = false; // Connection is now owned by the stream
        }
        
        let stream = async_stream::stream! {
            if let Some(mut ws) = ws_stream {
                while let Some(msg) = ws.next().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            // Try to parse as JSON-RPC response first
                            if let Ok(response) = serde_json::from_str::<JsonRpcResponse<serde_json::Value>>(&text) {
                                yield Ok(DeribitMessage::Response(response));
                            } else if let Ok(notification) = serde_json::from_str::<serde_json::Value>(&text) {
                                yield Ok(DeribitMessage::Notification(notification));
                            } else {
                                yield Err(format!("Failed to parse message: {}", text).into());
                            }
                        },
                        Ok(Message::Close(_)) => {
                            break;
                        },
                        Ok(_) => {
                            // Ignore other message types (binary, ping, pong)
                        },
                        Err(e) => {
                            yield Err(e.into());
                        }
                    }
                }
            }
        };

        Box::pin(stream)
    }
}

impl Default for WebSocketClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscribe_request_structure() {
        let request = SubscribeRequest {
            channels: vec!["user.portfolio.btc".to_string(), "user.orders.any.any.raw".to_string()],
            label: Some("test".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized["channels"].as_array().unwrap().len(), 2);
        assert_eq!(serialized["label"].as_str().unwrap(), "test");
    }

    #[test]
    fn test_subscribe_request_without_label() {
        let request = SubscribeRequest {
            channels: vec!["user.portfolio.btc".to_string()],
            label: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized["channels"].as_array().unwrap().len(), 1);
        assert!(serialized.get("label").is_none());
    }

    #[test]
    fn test_json_rpc_request_structure() {
        let params = SubscribeRequest {
            channels: vec!["user.portfolio.btc".to_string()],
            label: None,
        };

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "private/subscribe".to_string(),
            params,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized["jsonrpc"].as_str().unwrap(), "2.0");
        assert_eq!(serialized["id"].as_u64().unwrap(), 1);
        assert_eq!(serialized["method"].as_str().unwrap(), "private/subscribe");
        assert!(serialized["params"].is_object());
    }

    #[test]
    fn test_subscribe_response_deserialization() {
        let response_json = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": ["user.portfolio.btc", "user.orders.any.any.raw"]
        });

        let response: JsonRpcResponse<Vec<String>> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 1);
        assert!(response.result.is_some());
        assert_eq!(response.result.unwrap().len(), 2);
    }

    #[test]
    fn test_websocket_client_creation() {
        let client = WebSocketClient::new();
        assert!(!client.is_connected());
        assert_eq!(client.base_url, "wss://www.deribit.com/ws/api/v2");
    }

    #[test]
    fn test_websocket_client_with_custom_url() {
        let custom_url = "wss://test.deribit.com/ws/api/v2".to_string();
        let client = WebSocketClient::with_url(custom_url.clone());
        assert!(!client.is_connected());
        assert_eq!(client.base_url, custom_url);
    }

    #[tokio::test]
    async fn test_subscribe_validation_label_length() {
        let mut client = WebSocketClient::new();
        
        // Test label too long
        let long_label = "this_label_is_too_long".to_string(); // 21 characters
        let result = client.subscribe(vec!["test".to_string()], Some(long_label)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("16 characters or less"));
    }

    #[tokio::test] 
    async fn test_subscribe_not_connected() {
        let mut client = WebSocketClient::new();
        
        let result = client.subscribe(vec!["test".to_string()], None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Not connected"));
    }
}