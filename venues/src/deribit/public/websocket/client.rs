//! Deribit WebSocket client implementation

use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use async_trait::async_trait;
use futures::SinkExt;
use serde_json;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};
use websockets::{BoxResult, VenueMessage, WebSocketConnection};

use crate::deribit::public::websocket::hello::HelloResponse;
use crate::deribit::public::websocket::subscribe::SubscribeResponse;
use crate::deribit::message::JsonRpcRequest;
use crate::deribit::rate_limit::RateLimiter;

/// Deribit WebSocket message types
#[derive(Debug, Clone)]
pub enum DeribitMessage {
    /// Hello response message
    Hello(HelloResponse),
    /// Subscribe response message
    Subscribe(SubscribeResponse),
    /// Raw message for debugging/other purposes
    Raw(String),
}

impl VenueMessage for DeribitMessage {}

/// Errors specific to Deribit WebSocket operations
#[derive(Error, Debug)]
pub enum DeribitWebSocketError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("JSON-RPC error {code}: {message}")]
    JsonRpc { code: i32, message: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Rate limit error: {0}")]
    RateLimit(#[from] crate::deribit::rate_limit::RateLimitError),

    #[error("Request timeout for id {id}")]
    Timeout { id: u64 },

    #[error("Invalid response for request id {id}")]
    InvalidResponse { id: u64 },
}

/// WebSocket client for Deribit
pub struct DeribitWebSocketClient {
    /// WebSocket connection
    pub(crate) websocket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    /// Connection status
    connected: Arc<AtomicBool>,
    /// Rate limiter for API calls
    rate_limiter: Arc<RateLimiter>,
    /// Request ID counter for JSON-RPC
    pub(crate) request_id: Arc<AtomicU64>,
    /// Pending requests awaiting responses
    pending_requests: Arc<Mutex<HashMap<u64, tokio::sync::oneshot::Sender<serde_json::Value>>>>,
    /// WebSocket URL
    url: String,
}

impl DeribitWebSocketClient {
    /// Create a new Deribit WebSocket client
    pub fn new(url: Option<String>, rate_limiter: RateLimiter) -> Self {
        Self {
            websocket: None,
            connected: Arc::new(AtomicBool::new(false)),
            rate_limiter: Arc::new(rate_limiter),
            request_id: Arc::new(AtomicU64::new(1)),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            url: url.unwrap_or_else(|| "wss://www.deribit.com/ws/api/v2".to_string()),
        }
    }

    /// Send an unsubscribe_all request and wait for the response
    pub async fn unsubscribe_all(&mut self) -> Result<String, DeribitWebSocketError> {
        if !self.is_connected() {
            return Err(DeribitWebSocketError::Connection(
                "Not connected".to_string(),
            ));
        }
        let req = JsonRpcRequest::unsubscribe_all(
            self.next_request_id()
        );
        let msg = serde_json::to_string(&req)?;
        if let Some(ws) = &mut self.websocket {
            ws.send(Message::Text(msg.into()))
                .await
                .map_err(|e| DeribitWebSocketError::Connection(e.to_string()))?;
            // Wait for the response
            let response = self.receive_response().await?;
            Ok(response)
        } else {
            Err(DeribitWebSocketError::Connection(
                "WebSocket not connected".to_string(),
            ))
        }
    }

    /// Send an unsubscribe request with specific channels and wait for the response
    pub async fn unsubscribe(&mut self, channels: Vec<String>) -> Result<String, DeribitWebSocketError> {
        if !self.is_connected() {
            return Err(DeribitWebSocketError::Connection(
                "Not connected".to_string(),
            ));
        }
        let req = JsonRpcRequest::unsubscribe(
            self.next_request_id(),
            channels
        );
        let msg = serde_json::to_string(&req)?;
        if let Some(ws) = &mut self.websocket {
            ws.send(Message::Text(msg.into()))
                .await
                .map_err(|e| DeribitWebSocketError::Connection(e.to_string()))?;
            // Wait for the response
            let response = self.receive_response().await?;
            Ok(response)
        } else {
            Err(DeribitWebSocketError::Connection(
                "WebSocket not connected".to_string(),
            ))
        }
    }

    /// Send a disable_heartbeat request and wait for the response
    pub async fn disable_heartbeat(&mut self) -> Result<String, DeribitWebSocketError> {
        if !self.is_connected() {
            return Err(DeribitWebSocketError::Connection(
                "Not connected".to_string(),
            ));
        }
        let req = JsonRpcRequest::disable_heartbeat(
            self.next_request_id()
        );
        let msg = serde_json::to_string(&req)?;
        if let Some(ws) = &mut self.websocket {
            ws.send(Message::Text(msg.into()))
                .await
                .map_err(|e| DeribitWebSocketError::Connection(e.to_string()))?;
            // Wait for the response
            let response = self.receive_response().await?;
            Ok(response)
        } else {
            Err(DeribitWebSocketError::Connection(
                "WebSocket not connected".to_string(),
            ))
        }
    }

    /// Send a subscribe request and wait for the response
    pub async fn subscribe(&mut self, channels: Vec<String>) -> Result<Vec<String>, DeribitWebSocketError> {
        if !self.is_connected() {
            return Err(DeribitWebSocketError::Connection(
                "Not connected".to_string(),
            ));
        }
        let req = crate::deribit::public::websocket::subscribe::JsonRpcRequest::new_subscribe(self.next_request_id().try_into().unwrap(), channels);
        let msg = serde_json::to_string(&req)?;
        if let Some(ws) = &mut self.websocket {
            ws.send(Message::Text(msg.into()))
                .await
                .map_err(|e| DeribitWebSocketError::Connection(e.to_string()))?;
            // Wait for the response
            let response_str = self.receive_response().await?;
            let response: SubscribeResponse = serde_json::from_str(&response_str)?;
            Ok(response.result)
        } else {
            Err(DeribitWebSocketError::Connection(
                "WebSocket not connected".to_string(),
            ))
        }
    }

    /// Get the next request ID
    pub fn next_request_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Receive a response for a sent request
    async fn receive_response(&mut self) -> Result<String, DeribitWebSocketError> {
        let mut pending = self.pending_requests.lock().await;
        let (tx, rx) = tokio::sync::oneshot::channel();
        let id = self.next_request_id();
        pending.insert(id, tx);
        drop(pending);
        // Wait for the response or timeout
        let response: serde_json::Value = tokio::select! {
            res = rx => res.map_err(|_| DeribitWebSocketError::Timeout { id })?,
            _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
                return Err(DeribitWebSocketError::Timeout { id });
            }
        };
        let response_str = serde_json::to_string(&response).map_err(DeribitWebSocketError::Serialization)?;
        Ok(response_str)
    }
}

#[async_trait]
impl WebSocketConnection<DeribitMessage> for DeribitWebSocketClient {
    async fn connect(&mut self) -> BoxResult<()> {
        let (ws_stream, _) = connect_async(&self.url).await?;
        self.websocket = Some(ws_stream);
        self.connected.store(true, Ordering::SeqCst);
        Ok(())
    }

    async fn disconnect(&mut self) -> BoxResult<()> {
        if let Some(mut ws) = self.websocket.take() {
            ws.close(None).await?;
        }
        self.connected.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    fn message_stream(&mut self) -> Pin<Box<dyn futures::Stream<Item = BoxResult<DeribitMessage>> + Send>> {
        // ...implementation for message stream...
        Box::pin(futures::stream::empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::rate_limit::AccountTier;

    #[test]
    fn test_deribit_websocket_client_creation() {
        let rl = RateLimiter::new(AccountTier::default());
        let client = DeribitWebSocketClient::new(None, rl);
        assert!(!client.is_connected());
    }

    #[test]
    fn test_deribit_websocket_client_custom_url() {
        let rl = RateLimiter::new(AccountTier::default());
        let url = "wss://test.deribit.com/ws/api/v2".to_string();
        let client = DeribitWebSocketClient::new(Some(url.clone()), rl);
        assert_eq!(client.url, url);
    }

    #[test]
    fn test_request_id_generation() {
        let rl = RateLimiter::new(AccountTier::default());
        let client = DeribitWebSocketClient::new(None, rl);
        let id1 = client.next_request_id();
        let id2 = client.next_request_id();
        assert_eq!(id2, id1 + 1);
    }

    #[test]
    fn test_deribit_message_types() {
        let hello = HelloResponse::default();
        let msg = DeribitMessage::Hello(hello);
        match msg {
            DeribitMessage::Hello(_) => (),
            _ => panic!("Expected Hello variant"),
        }

        let subscribe = SubscribeResponse::default();
        let msg = DeribitMessage::Subscribe(subscribe);
        match msg {
            DeribitMessage::Subscribe(_) => (),
            _ => panic!("Expected Subscribe variant"),
        }
    }

    #[test]
    fn test_subscribe_message_type() {
        use crate::deribit::public::websocket::subscribe::SubscribeResponse;

        let subscribe_response = SubscribeResponse {
            id: 1,
            jsonrpc: "2.0".to_string(),
            result: vec!["book.BTC-PERPETUAL.100ms".to_string()],
        };

        let msg = DeribitMessage::Subscribe(subscribe_response.clone());
        match msg {
            DeribitMessage::Subscribe(response) => {
                assert_eq!(response.id, 1);
                assert_eq!(response.jsonrpc, "2.0");
                assert_eq!(response.result.len(), 1);
                assert_eq!(response.result[0], "book.BTC-PERPETUAL.100ms");
            }
            _ => panic!("Expected Subscribe variant"),
        }
    }

    #[test]
    fn test_unsubscribe_method_signature() {
        // Test that the unsubscribe method has the correct signature
        // This is a compile-time test to ensure the method signature is correct
        let rl = RateLimiter::new(AccountTier::default());
        let mut client = DeribitWebSocketClient::new(None, rl);
        
        // This is a compile test - we don't actually call the async method
        // Just verify the signature exists
        let channels = vec!["ticker.BTC-PERPETUAL".to_string()];
        let _future = client.unsubscribe(channels);
        // Don't await the future since we're not connected
    }
}
