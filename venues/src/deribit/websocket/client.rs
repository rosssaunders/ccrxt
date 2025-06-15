//! Deribit WebSocket client implementation

use super::messages::*;
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use std::sync::atomic::{AtomicU64, Ordering};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream,
};
use websockets::{BoxError, BoxResult, VenueMessage, WebSocketConnection};

/// Deribit WebSocket client errors
#[derive(Error, Debug)]
pub enum DeribitWsError {
    #[error("WebSocket connection error: {0}")]
    ConnectionError(String),
    #[error("JSON-RPC error: {error:?}")]
    JsonRpcError { error: JsonRpcError },
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Not connected")]
    NotConnected,
    #[error("Invalid response: expected ID {expected}, got {actual}")]
    InvalidResponseId { expected: u64, actual: u64 },
}

/// Deribit message type for the VenueMessage trait
#[derive(Debug, Clone)]
pub enum DeribitMessage {
    /// JSON-RPC response
    JsonRpc(serde_json::Value),
    /// Raw text message
    Text(String),
    /// Connection status update
    Status(String),
}

impl VenueMessage for DeribitMessage {}

/// Deribit WebSocket client
pub struct DeribitWebSocketClient {
    /// WebSocket stream
    stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    /// Base URL for WebSocket connections
    base_url: String,
    /// Request ID counter
    request_id: AtomicU64,
    /// Connection status
    connected: bool,
}

impl DeribitWebSocketClient {
    /// Create a new Deribit WebSocket client
    pub fn new() -> Self {
        Self {
            stream: None,
            base_url: "wss://www.deribit.com/ws/api/v2".to_string(),
            request_id: AtomicU64::new(1),
            connected: false,
        }
    }

    /// Create a new Deribit WebSocket client with custom URL (for testing)
    pub fn with_url(url: String) -> Self {
        Self {
            stream: None,
            base_url: url,
            request_id: AtomicU64::new(1),
            connected: false,
        }
    }

    /// Generate next request ID
    fn next_request_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Subscribe to public channels
    pub async fn public_subscribe(&mut self, channels: Vec<String>) -> Result<SubscribeResult, DeribitWsError> {
        if !self.connected {
            return Err(DeribitWsError::NotConnected);
        }

        let params = SubscribeParams { channels };
        let request_id = self.next_request_id();
        let request = PublicSubscribeRequest::new("public/subscribe".to_string(), params, request_id);

        // Serialize request
        let request_json = serde_json::to_string(&request)
            .map_err(|e| DeribitWsError::SerializationError(format!("Failed to serialize request: {}", e)))?;

        let stream = self.stream.as_mut()
            .ok_or(DeribitWsError::NotConnected)?;

        // Send request
        stream.send(Message::Text(request_json.into())).await
            .map_err(|e| DeribitWsError::ConnectionError(format!("Failed to send message: {}", e)))?;

        // Wait for response
        let stream = self.stream.as_mut()
            .ok_or(DeribitWsError::NotConnected)?;
            
        while let Some(message) = stream.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    let text_str = text.to_string();
                    let response: PublicSubscribeResponse = serde_json::from_str(&text_str)
                        .map_err(|e| DeribitWsError::SerializationError(format!("Failed to parse response: {}", e)))?;

                    if response.id != request_id {
                        continue; // Not our response, keep waiting
                    }

                    if let Some(error) = response.error {
                        return Err(DeribitWsError::JsonRpcError { error });
                    }

                    return response.result
                        .ok_or_else(|| DeribitWsError::SerializationError("Response missing result field".to_string()));
                }
                Ok(Message::Close(_)) => {
                    self.connected = false;
                    return Err(DeribitWsError::ConnectionError("Connection closed".to_string()));
                }
                Ok(_) => {
                    // Ignore other message types (binary, ping, pong)
                    continue;
                }
                Err(e) => {
                    return Err(DeribitWsError::ConnectionError(format!("WebSocket error: {}", e)));
                }
            }
        }

        Err(DeribitWsError::ConnectionError("Stream ended without response".to_string()))
    }
}

impl Default for DeribitWebSocketClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WebSocketConnection<DeribitMessage> for DeribitWebSocketClient {
    async fn connect(&mut self) -> BoxResult<()> {
        let (ws_stream, _) = connect_async(&self.base_url).await
            .map_err(|e| -> BoxError { format!("Failed to connect to {}: {}", self.base_url, e).into() })?;

        self.stream = Some(ws_stream);
        self.connected = true;
        Ok(())
    }

    async fn disconnect(&mut self) -> BoxResult<()> {
        if let Some(mut stream) = self.stream.take() {
            let _ = stream.close(None).await; // Ignore close errors
        }
        self.connected = false;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    fn message_stream(&mut self) -> std::pin::Pin<Box<dyn futures::Stream<Item = BoxResult<DeribitMessage>> + Send>> {
        use futures::stream;

        // For now, return an empty stream to satisfy the trait
        // In a real implementation, this would use channels or similar to provide a 'static stream
        Box::pin(stream::empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = DeribitWebSocketClient::new();
        assert!(!client.is_connected());
        assert_eq!(client.base_url, "wss://www.deribit.com/ws/api/v2");
    }

    #[test]
    fn test_client_with_custom_url() {
        let custom_url = "wss://test.example.com/ws".to_string();
        let client = DeribitWebSocketClient::with_url(custom_url.clone());
        assert_eq!(client.base_url, custom_url);
    }

    #[test]
    fn test_request_id_generation() {
        let client = DeribitWebSocketClient::new();
        let id1 = client.next_request_id();
        let id2 = client.next_request_id();
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[tokio::test]
    async fn test_public_subscribe_not_connected() {
        let mut client = DeribitWebSocketClient::new();
        let channels = vec!["book.BTC-PERPETUAL.100ms".to_string()];
        
        let result = client.public_subscribe(channels).await;
        assert!(matches!(result, Err(DeribitWsError::NotConnected)));
    }

    #[test]
    fn test_error_types() {
        let error = DeribitWsError::ConnectionError("test error".to_string());
        assert!(error.to_string().contains("test error"));

        let json_rpc_error = JsonRpcError::new(-32602, "Invalid params".to_string(), None);
        let error = DeribitWsError::JsonRpcError { error: json_rpc_error };
        assert!(error.to_string().contains("Invalid params"));
    }

    #[test]
    fn test_deribit_message_debug() {
        let msg = DeribitMessage::Text("test".to_string());
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("Text"));
        assert!(debug_str.contains("test"));
    }
}