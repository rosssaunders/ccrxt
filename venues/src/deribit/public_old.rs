//! Deribit public WebSocket API implementation
//!
//! This module implements the public WebSocket endpoints for Deribit,
//! including the set_heartbeat functionality.

use crate::deribit::websocket::{
    DeribitMessage, JsonRpcRequest, JsonRpcResponse, SetHeartbeatParams, SetHeartbeatResult,
};
use async_trait::async_trait;
use futures::{stream::Stream, SinkExt, StreamExt};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};
use websockets::{BoxError, BoxResult, WebSocketConnection};

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

/// Deribit WebSocket client for public API endpoints
pub struct DeribitWebSocketClient {
    /// WebSocket connection
    ws_stream: Option<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    /// Base URL for WebSocket connections
    base_url: String,
    /// Request ID counter
    request_id_counter: Arc<AtomicU64>,
    /// Pending requests waiting for responses
    pending_requests: Arc<RwLock<HashMap<u64, mpsc::UnboundedSender<JsonRpcResponse<serde_json::Value>>>>>,
    /// Message receiver for incoming messages
    message_receiver: Option<mpsc::UnboundedReceiver<BoxResult<DeribitMessage>>>,
}

impl DeribitWebSocketClient {
    /// Create a new Deribit WebSocket client
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            ws_stream: None,
            base_url: base_url.into(),
            request_id_counter: Arc::new(AtomicU64::new(1)),
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
            message_receiver: None,
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

    /// Handle incoming WebSocket messages
    async fn handle_message(&self, message: Message) -> BoxResult<Option<DeribitMessage>> {
        match message {
            Message::Text(text) => {
                let deribit_msg: DeribitMessage = serde_json::from_str(&text)?;
                
                // If it's a response, route it to the pending request
                if let DeribitMessage::Response(ref response) = deribit_msg {
                    let pending = self.pending_requests.read().await;
                    if let Some(sender) = pending.get(&response.id) {
                        let _ = sender.send(response.clone());
                        return Ok(None); // Don't pass responses to the message stream
                    }
                }
                
                Ok(Some(deribit_msg))
            }
            Message::Ping(data) => {
                // Echo pings as pongs - this should be handled at the connection level
                // For now, we'll just ignore pings in the message stream
                tracing::debug!("Received ping: {:?}", data);
                Ok(None)
            }
            Message::Pong(_) => {
                // Ignore pongs
                Ok(None)
            }
            Message::Close(_) => {
                tracing::info!("WebSocket connection closed");
                Ok(None)
            }
            _ => {
                tracing::warn!("Received unexpected message type: {:?}", message);
                Ok(None)
            }
        }
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
        self.message_receiver = None;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.ws_stream.is_some()
    }

    fn message_stream(&mut self) -> Pin<Box<dyn Stream<Item = BoxResult<DeribitMessage>> + Send>> {
        let (tx, rx) = mpsc::unbounded_channel();
        
        if let Some(ref mut ws_stream) = self.ws_stream {
            let pending_requests = self.pending_requests.clone();
            
            // Spawn a task to handle incoming messages
            let mut ws_receiver = ws_stream.by_ref();
            tokio::spawn(async move {
                while let Some(message_result) = ws_receiver.next().await {
                    match message_result {
                        Ok(message) => {
                            // Handle the message - this is a simplified version
                            // In a real implementation, we'd need better message routing
                            if let Message::Text(text) = message {
                                match serde_json::from_str::<DeribitMessage>(&text) {
                                    Ok(deribit_msg) => {
                                        // Route responses to pending requests
                                        if let DeribitMessage::Response(ref response) = deribit_msg {
                                            let pending = pending_requests.read().await;
                                            if let Some(sender) = pending.get(&response.id) {
                                                let _ = sender.send(response.clone());
                                                continue; // Don't send to stream
                                            }
                                        }
                                        
                                        // Send other messages to the stream
                                        if tx.send(Ok(deribit_msg)).is_err() {
                                            break;
                                        }
                                    }
                                    Err(e) => {
                                        if tx.send(Err(Box::new(e) as BoxError)).is_err() {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if tx.send(Err(Box::new(e) as BoxError)).is_err() {
                                break;
                            }
                        }
                    }
                }
            });
        }
        
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
}