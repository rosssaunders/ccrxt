//! Deribit Public WebSocket client: connection management only
//!
//! This file MUST NOT contain endpoint logic. It only manages the connection and delegates to endpoint modules.
//! client.rs MUST only contain WebSocket client struct and connection management.
//! All message construction, serialization, and endpoint logic must be in separate files.
//! This file should not contain endpoint-specific logic or message types.

use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use async_trait::async_trait;
use futures::SinkExt;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use websockets::{BoxResult, WebSocketConnection};

use crate::deribit::message::DeribitMessage;
use crate::deribit::rate_limit::RateLimiter;

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

#[derive(serde::Deserialize)]
struct JsonRpcEnvelope<R> {
    id: Option<i32>,
    jsonrpc: Option<String>,
    result: R,
}

/// WebSocket client for Deribit public endpoints
///
/// This struct manages the connection and delegates endpoint logic to endpoint modules.
pub struct PrivateWebSocketClient {
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

impl PrivateWebSocketClient {
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

    /// Get the next request ID
    pub fn next_request_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Receive a response for a sent request (used internally by endpoint modules)
    pub(crate) async fn receive_response(&mut self) -> Result<String, DeribitWebSocketError> {
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

    /// Check if the client is connected
    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    /// Send any serializable request struct over the websocket
    pub async fn send_serializable<T: serde::Serialize + ?Sized>(&mut self, req: &T) -> Result<(), DeribitWebSocketError> {
        if !self.is_connected() {
            return Err(DeribitWebSocketError::Connection(
                "Not connected".to_string(),
            ));
        }
        let req_json = serde_json::to_string(req)?;
        self.websocket
            .as_mut()
            .ok_or_else(|| DeribitWebSocketError::Connection("WebSocket not connected".to_string()))?
            .send(tokio_tungstenite::tungstenite::Message::Text(
                req_json.into(),
            ))
            .await
            .map_err(|e| DeribitWebSocketError::Connection(e.to_string()))?;
        Ok(())
    }

    /// Generic send and receive for request/response types
    /// Generic send and receive for request/response types, extracting the `result` field
    pub async fn send_and_receive<T: Serialize, R: for<'de> Deserialize<'de>>(&mut self, request: &T) -> Result<R, DeribitWebSocketError> {
        self.send_serializable(request).await?;
        let response_str = self.receive_response().await?;
        let envelope: JsonRpcEnvelope<R> = serde_json::from_str(&response_str)?;
        Ok(envelope.result)
    }
}

#[async_trait]
impl WebSocketConnection<DeribitMessage> for PrivateWebSocketClient {
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

    fn message_stream(&mut self) -> Pin<Box<dyn futures::Stream<Item = websockets::BoxResult<DeribitMessage>> + Send>> {
        // Not implemented for public client (not used in this context)
        unimplemented!("message_stream is not implemented for DeribitWebSocketClient public client")
    }
}
