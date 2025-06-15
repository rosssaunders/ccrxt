//! Deribit public WebSocket API implementation
//!
//! This module implements the public WebSocket endpoints for Deribit,
//! including the set_heartbeat functionality.

use crate::deribit::websocket::{
    DeribitMessage, JsonRpcRequest, JsonRpcResponse, SetHeartbeatParams, SetHeartbeatResult,
};
use async_trait::async_trait;
use futures::{stream::Stream, SinkExt};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream, MaybeTlsStream};
use websockets::{BoxResult, WebSocketConnection};

/// Errors specific to Deribit WebSocket operations
#[derive(Error, Debug)]
pub enum DeribitWebSocketError {
    #[error("Connection not established")]
    NotConnected,
    #[error("Invalid heartbeat interval: {interval}. Minimum interval is 10 seconds")]
    InvalidHeartbeatInterval { interval: u32 },
    #[error("JSON-RPC error: {code} - {message}")]
    JsonRpcError { code: i32, message: String },
    #[error("Request timeout")]
    RequestTimeout,
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

type DeribitWebSocket = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;

/// Deribit WebSocket client for public API endpoints
pub struct DeribitWebSocketClient {
    /// WebSocket connection
    ws_stream: Option<DeribitWebSocket>,
    /// Base URL for WebSocket connections
    base_url: String,
    /// Request ID counter
    request_id_counter: Arc<AtomicU64>,
    /// Pending requests waiting for responses
    pending_requests: Arc<RwLock<HashMap<u64, mpsc::UnboundedSender<JsonRpcResponse<serde_json::Value>>>>>,
}

impl DeribitWebSocketClient {
    /// Create a new Deribit WebSocket client
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            ws_stream: None,
            base_url: base_url.into(),
            request_id_counter: Arc::new(AtomicU64::new(1)),
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new client with default Deribit WebSocket URL
    pub fn new_default() -> Self {
        Self::new("wss://www.deribit.com/ws/api/v2")
    }

    /// Generate the next request ID
    fn next_request_id(&self) -> u64 {
        self.request_id_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Send a JSON-RPC request and wait for the response
    async fn send_request<T, R>(
        &mut self,
        method: &str,
        params: T,
    ) -> Result<R, DeribitWebSocketError>
    where
        T: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        if self.ws_stream.is_none() {
            return Err(DeribitWebSocketError::NotConnected);
        }

        let request_id = self.next_request_id();
        let request = JsonRpcRequest::new(method.to_string(), params, request_id);

        // Create a channel to receive the response
        let (tx, mut rx) = mpsc::unbounded_channel();
        {
            let mut pending = self.pending_requests.write().await;
            pending.insert(request_id, tx);
        }

        // Send the request
        let message = serde_json::to_string(&request)?;
        if let Some(ref mut ws) = self.ws_stream {
            ws.send(Message::Text(message.into())).await?;
        }

        // Wait for the response
        let response = tokio::time::timeout(
            tokio::time::Duration::from_secs(30),
            rx.recv(),
        )
        .await
        .map_err(|_| DeribitWebSocketError::RequestTimeout)?
        .ok_or(DeribitWebSocketError::RequestTimeout)?;

        // Clean up pending request
        {
            let mut pending = self.pending_requests.write().await;
            pending.remove(&request_id);
        }

        // Check for JSON-RPC error
        if let Some(error) = response.error {
            return Err(DeribitWebSocketError::JsonRpcError {
                code: error.code,
                message: error.message,
            });
        }

        // Deserialize the result
        let result = response.result.ok_or_else(|| {
            DeribitWebSocketError::JsonRpcError {
                code: -32603,
                message: "Missing result in response".to_string(),
            }
        })?;

        Ok(serde_json::from_value(result)?)
    }

    /// Set heartbeat interval for the WebSocket connection
    pub async fn set_heartbeat(&mut self, interval: u32) -> Result<String, DeribitWebSocketError> {
        if interval < 10 {
            return Err(DeribitWebSocketError::InvalidHeartbeatInterval { interval });
        }

        let params = SetHeartbeatParams { interval };
        self.send_request::<SetHeartbeatParams, SetHeartbeatResult>("public/set_heartbeat", params)
            .await
    }

    /// Process incoming messages and route responses to pending requests
    pub async fn process_message(&self, message: DeribitMessage) -> Result<Option<DeribitMessage>, DeribitWebSocketError> {
        // If it's a response, route it to the pending request
        if let DeribitMessage::Response(ref response) = message {
            let pending = self.pending_requests.read().await;
            if let Some(sender) = pending.get(&response.id) {
                let _ = sender.send(response.clone());
                return Ok(None); // Don't pass responses to the message stream
            }
        }
        
        Ok(Some(message))
    }
}

#[async_trait]
impl WebSocketConnection<DeribitMessage> for DeribitWebSocketClient {
    async fn connect(&mut self) -> BoxResult<()> {
        let (ws_stream, _) = connect_async(&self.base_url).await?;
        self.ws_stream = Some(ws_stream);
        Ok(())
    }

    async fn disconnect(&mut self) -> BoxResult<()> {
        if let Some(mut ws) = self.ws_stream.take() {
            ws.close(None).await?;
        }
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.ws_stream.is_some()
    }

    fn message_stream(&mut self) -> Pin<Box<dyn Stream<Item = BoxResult<DeribitMessage>> + Send>> {
        // Return an empty stream for now - in a real implementation, 
        // you would need to split the WebSocket and handle messages properly
        let (_, rx) = mpsc::unbounded_channel();
        Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(rx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = DeribitWebSocketClient::new("wss://test.example.com");
        assert_eq!(client.base_url, "wss://test.example.com");
        assert!(!client.is_connected());
    }

    #[test]
    fn test_default_client_creation() {
        let client = DeribitWebSocketClient::new_default();
        assert_eq!(client.base_url, "wss://www.deribit.com/ws/api/v2");
        assert!(!client.is_connected());
    }

    #[test]
    fn test_request_id_generation() {
        let client = DeribitWebSocketClient::new_default();
        assert_eq!(client.next_request_id(), 1);
        assert_eq!(client.next_request_id(), 2);
        assert_eq!(client.next_request_id(), 3);
    }

    #[tokio::test]
    async fn test_set_heartbeat_validation() {
        let mut client = DeribitWebSocketClient::new_default();
        
        // Test invalid interval (less than 10)
        let result = client.set_heartbeat(5).await;
        assert!(matches!(
            result,
            Err(DeribitWebSocketError::InvalidHeartbeatInterval { interval: 5 })
        ));
        
        // Test not connected error
        let result = client.set_heartbeat(15).await;
        assert!(matches!(result, Err(DeribitWebSocketError::NotConnected)));
    }

    #[test]
    fn test_error_display() {
        let error = DeribitWebSocketError::InvalidHeartbeatInterval { interval: 5 };
        let message = format!("{}", error);
        assert!(message.contains("Invalid heartbeat interval: 5"));
        assert!(message.contains("Minimum interval is 10 seconds"));
    }

    #[tokio::test]
    async fn test_message_processing() {
        let client = DeribitWebSocketClient::new_default();
        
        // Test processing a regular message (should pass through)
        let request_msg = DeribitMessage::Request(JsonRpcRequest::new(
            "public/test".to_string(),
            serde_json::Value::Null,
            1,
        ));
        
        let result = client.process_message(request_msg.clone()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }
}