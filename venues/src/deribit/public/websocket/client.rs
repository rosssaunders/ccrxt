//! Deribit WebSocket client implementation

use crate::deribit::rate_limit::{EndpointType, RateLimiter};
use crate::deribit::public::websocket::hello::{HelloResponse, JsonRpcRequest};
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use serde_json;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use websockets::{BoxError, BoxResult, VenueMessage, WebSocketConnection};

/// Deribit WebSocket message types
#[derive(Debug, Clone)]
pub enum DeribitMessage {
    /// Hello response message
    Hello(HelloResponse),
    /// Raw message for debugging/other purposes
    Raw(String),
}

impl VenueMessage for DeribitMessage {}

/// WebSocket client for Deribit
pub struct DeribitWebSocketClient {
    /// WebSocket connection
    websocket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    /// Connection status
    connected: Arc<AtomicBool>,
    /// Rate limiter for API calls
    rate_limiter: RateLimiter,
    /// Request ID counter for JSON-RPC
    request_id: Arc<AtomicI32>,
    /// WebSocket URL
    url: String,
}

impl DeribitWebSocketClient {
    /// Create a new Deribit WebSocket client
    pub fn new(url: Option<String>, rate_limiter: RateLimiter) -> Self {
        let default_url = "wss://www.deribit.com/ws/api/v2".to_string();
        Self {
            websocket: None,
            connected: Arc::new(AtomicBool::new(false)),
            rate_limiter,
            request_id: Arc::new(AtomicI32::new(1)),
            url: url.unwrap_or(default_url),
        }
    }

    /// Send a hello message to introduce the client
    pub async fn send_hello(
        &mut self,
        client_name: String,
        client_version: String,
    ) -> BoxResult<HelloResponse> {
        // Check rate limits
        self.rate_limiter
            .check_limits(EndpointType::PublicHello)
            .await
            .map_err(|e| -> BoxError { Box::new(e) })?;

        if !self.is_connected() {
            return Err("WebSocket not connected".into());
        }

        let id = self.request_id.fetch_add(1, Ordering::SeqCst);
        let request = JsonRpcRequest::new_hello(id, client_name, client_version);
        
        let message = serde_json::to_string(&request)?;
        
        if let Some(ref mut ws) = self.websocket {
            ws.send(Message::Text(message.into())).await?;
            
            // Record the request
            self.rate_limiter
                .record_request(EndpointType::PublicHello)
                .await;

            // Wait for response (simplified - in production you'd want proper request/response tracking)
            while let Some(msg) = ws.next().await {
                match msg? {
                    Message::Text(text) => {
                        let text_str = text.to_string();
                        if let Ok(response) = serde_json::from_str::<HelloResponse>(&text_str) {
                            if response.id == id {
                                return Ok(response);
                            }
                        }
                    }
                    Message::Close(_) => {
                        self.connected.store(false, Ordering::SeqCst);
                        return Err("WebSocket connection closed".into());
                    }
                    _ => {}
                }
            }
        }

        Err("Failed to receive hello response".into())
    }

    /// Get the next request ID
    pub fn next_request_id(&self) -> i32 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
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

    fn message_stream(
        &mut self,
    ) -> std::pin::Pin<Box<dyn futures::Stream<Item = BoxResult<DeribitMessage>> + Send>> {
        use futures::stream;
        
        if let Some(ws) = self.websocket.take() {
            let connected = Arc::clone(&self.connected);
            
            let stream = ws.map(move |msg| -> BoxResult<DeribitMessage> {
                match msg? {
                    Message::Text(text) => {
                        let text_str = text.to_string();
                        // Try to parse as HelloResponse first
                        if let Ok(hello) = serde_json::from_str::<HelloResponse>(&text_str) {
                            Ok(DeribitMessage::Hello(hello))
                        } else {
                            Ok(DeribitMessage::Raw(text_str))
                        }
                    }
                    Message::Close(_) => {
                        connected.store(false, Ordering::SeqCst);
                        Err("WebSocket connection closed".into())
                    }
                    _ => Ok(DeribitMessage::Raw("Non-text message".to_string())),
                }
            });
            
            Box::pin(stream)
        } else {
            Box::pin(stream::empty())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::rate_limit::AccountTier;

    #[test]
    fn test_deribit_websocket_client_creation() {
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let client = DeribitWebSocketClient::new(None, rate_limiter);
        
        assert!(!client.is_connected());
        assert_eq!(client.url, "wss://www.deribit.com/ws/api/v2");
    }

    #[test]
    fn test_deribit_websocket_client_custom_url() {
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let custom_url = "wss://test.deribit.com/ws/api/v2".to_string();
        let client = DeribitWebSocketClient::new(Some(custom_url.clone()), rate_limiter);
        
        assert_eq!(client.url, custom_url);
    }

    #[test]
    fn test_request_id_generation() {
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let client = DeribitWebSocketClient::new(None, rate_limiter);
        
        let id1 = client.next_request_id();
        let id2 = client.next_request_id();
        
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[test]
    fn test_deribit_message_types() {
        let hello_response = HelloResponse {
            id: 1,
            jsonrpc: "2.0".to_string(),
            result: crate::deribit::public::HelloResult {
                version: "1.2.26".to_string(),
            },
        };
        
        let msg = DeribitMessage::Hello(hello_response.clone());
        match msg {
            DeribitMessage::Hello(response) => {
                assert_eq!(response.id, 1);
                assert_eq!(response.result.version, "1.2.26");
            }
            _ => panic!("Expected Hello message"),
        }
        
        let raw_msg = DeribitMessage::Raw("test message".to_string());
        match raw_msg {
            DeribitMessage::Raw(text) => {
                assert_eq!(text, "test message");
            }
            _ => panic!("Expected Raw message"),
        }
    }
}