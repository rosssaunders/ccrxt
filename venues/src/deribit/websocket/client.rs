//! Deribit WebSocket client implementation

use crate::deribit::websocket::messages::{DeribitMessage, JsonRpcRequest};
use async_trait::async_trait;
use futures::{stream::SplitSink, SinkExt, Stream, StreamExt};
use serde_json::Value;
use std::pin::Pin;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use websockets::{BoxResult, WebSocketConnection};

/// Deribit WebSocket client
pub struct DeribitWebSocketClient {
    /// WebSocket URL
    url: String,
    /// WebSocket sender
    sender: Option<Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>>,
    /// Connection status
    is_connected: bool,
    /// Request ID counter for JSON-RPC
    next_id: Arc<AtomicI64>,
}

impl DeribitWebSocketClient {
    /// Create a new Deribit WebSocket client
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            sender: None,
            is_connected: false,
            next_id: Arc::new(AtomicI64::new(1)),
        }
    }

    /// Create a new client with default Deribit WebSocket URL
    pub fn new_default() -> Self {
        Self::new("wss://www.deribit.com/ws/api/v2")
    }

    /// Generate the next request ID
    pub fn next_request_id(&self) -> i64 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }

    /// Send a JSON-RPC request and return the ID for tracking the response
    pub async fn send_request(
        &self,
        method: impl Into<String>,
        params: Option<Value>,
    ) -> BoxResult<i64> {
        if !self.is_connected {
            return Err("WebSocket not connected".into());
        }

        let id = self.next_request_id();
        let request = JsonRpcRequest::new(id, method, params);
        let message_text = serde_json::to_string(&request)?;
        let websocket_message = Message::Text(message_text.into());

        if let Some(sender) = &self.sender {
            let mut sender_guard = sender.lock().await;
            sender_guard.send(websocket_message).await?;
            Ok(id)
        } else {
            Err("WebSocket sender not available".into())
        }
    }
}

#[async_trait]
impl WebSocketConnection<DeribitMessage> for DeribitWebSocketClient {
    async fn connect(&mut self) -> BoxResult<()> {
        let (ws_stream, _) = connect_async(&self.url).await?;
        let (sender, _receiver) = ws_stream.split();

        self.sender = Some(Arc::new(Mutex::new(sender)));
        self.is_connected = true;

        // Store receiver for message_stream method
        // Note: In a real implementation, you'd want to store the receiver
        // in a way that can be accessed by message_stream()
        // For now, we'll handle this in the stream implementation

        Ok(())
    }

    async fn disconnect(&mut self) -> BoxResult<()> {
        if let Some(sender) = &self.sender {
            let mut sender_guard = sender.lock().await;
            sender_guard.send(Message::Close(None)).await?;
        }
        
        self.sender = None;
        self.is_connected = false;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.is_connected
    }

    fn message_stream(&mut self) -> Pin<Box<dyn Stream<Item = BoxResult<DeribitMessage>> + Send>> {
        // Note: This is a simplified implementation
        // In a real implementation, you would need to properly handle the receiver
        // from the connect() method and convert raw messages to DeribitMessage
        Box::pin(futures::stream::empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = DeribitWebSocketClient::new("wss://test.example.com");
        assert_eq!(client.url, "wss://test.example.com");
        assert!(!client.is_connected());
        assert_eq!(client.next_request_id(), 1);
        assert_eq!(client.next_request_id(), 2);
    }

    #[test]
    fn test_default_client() {
        let client = DeribitWebSocketClient::new_default();
        assert_eq!(client.url, "wss://www.deribit.com/ws/api/v2");
        assert!(!client.is_connected());
    }

    #[test]
    fn test_request_id_generation() {
        let client = DeribitWebSocketClient::new_default();
        
        // Test that IDs are generated sequentially
        let id1 = client.next_request_id();
        let id2 = client.next_request_id();
        let id3 = client.next_request_id();
        
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
    }

    #[tokio::test]
    async fn test_send_request_when_not_connected() {
        let client = DeribitWebSocketClient::new_default();
        
        let result = client.send_request("test/method", None).await;
        assert!(result.is_err());
        assert!(result.expect_err("Should error").to_string().contains("not connected"));
    }
}