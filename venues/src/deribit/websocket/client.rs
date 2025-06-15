use crate::deribit::rate_limit::{EndpointType, RateLimiter};
use crate::deribit::websocket::message::{DeribitMessage, JsonRpcRequest, JsonRpcResponse};
use async_trait::async_trait;
use futures::{SinkExt, Stream, StreamExt};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use websockets::{BoxResult, WebSocketConnection};

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

/// Deribit WebSocket client implementing the common WebSocket trait
pub struct WebSocketClient {
    /// WebSocket connection
    websocket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    /// Rate limiter for the connection
    rate_limiter: Arc<RateLimiter>,
    /// Connection state
    is_connected: AtomicBool,
    /// Request ID counter
    next_request_id: AtomicU64,
    /// Pending requests awaiting responses
    pending_requests: Arc<Mutex<HashMap<u64, tokio::sync::oneshot::Sender<JsonRpcResponse>>>>,
}

impl WebSocketClient {
    /// Create a new Deribit WebSocket client
    pub fn new(rate_limiter: RateLimiter) -> Self {
        Self {
            websocket: None,
            rate_limiter: Arc::new(rate_limiter),
            is_connected: AtomicBool::new(false),
            next_request_id: AtomicU64::new(1),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Send an unsubscribe_all request and wait for the response
    pub async fn unsubscribe_all(&mut self) -> Result<String, DeribitWebSocketError> {
        // Check rate limits for public endpoints (using NonMatchingEngine type)
        self.rate_limiter
            .check_limits(EndpointType::NonMatchingEngine)
            .await?;

        let request_id = self.next_request_id.fetch_add(1, Ordering::SeqCst);
        let request = JsonRpcRequest::unsubscribe_all(request_id);

        // Create a channel for the response (not used in this simplified implementation)
        let (_response_sender, _response_receiver) = tokio::sync::oneshot::channel::<JsonRpcResponse>();

        // Send the request
        if let Some(ref mut ws) = self.websocket {
            let message_text = serde_json::to_string(&request)?;
            ws.send(Message::Text(message_text.into())).await
                .map_err(|e| DeribitWebSocketError::Connection(e.to_string()))?;
        } else {
            return Err(DeribitWebSocketError::Connection("Not connected".to_string()));
        }

        // Record the request for rate limiting
        self.rate_limiter
            .record_request(EndpointType::NonMatchingEngine)
            .await;

        // Process messages until we get our response
        let mut response_result = None;
        let timeout = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            async {
                loop {
                    if let Some(ref mut ws) = self.websocket {
                        if let Some(msg) = ws.next().await {
                            match msg {
                                Ok(Message::Text(text)) => {
                                    if let Ok(deribit_msg) = serde_json::from_str::<DeribitMessage>(&text) {
                                        if let DeribitMessage::Response(response) = deribit_msg {
                                            if response.id == request_id {
                                                response_result = Some(response);
                                                break;
                                            }
                                        }
                                    }
                                }
                                Ok(Message::Close(_)) => {
                                    self.is_connected.store(false, Ordering::SeqCst);
                                    break;
                                }
                                Err(e) => {
                                    return Err(DeribitWebSocketError::Connection(e.to_string()));
                                }
                                _ => {} // Ignore other message types
                            }
                        }
                    }
                }
                Ok(())
            }
        ).await;

        // Clean up pending request
        {
            let mut pending = self.pending_requests.lock().await;
            pending.remove(&request_id);
        }

        match timeout {
            Err(_) => Err(DeribitWebSocketError::Timeout { id: request_id }),
            Ok(Err(e)) => Err(e),
            Ok(Ok(_)) => {
                if let Some(response) = response_result {
                    // Check for JSON-RPC errors
                    if let Some(error) = response.error {
                        return Err(DeribitWebSocketError::JsonRpc {
                            code: error.code,
                            message: error.message,
                        });
                    }

                    // Return the result
                    response.result_as_string()
                        .ok_or(DeribitWebSocketError::InvalidResponse { id: request_id })
                } else {
                    Err(DeribitWebSocketError::InvalidResponse { id: request_id })
                }
            }
        }
    }

    /// Send a disable_heartbeat request and wait for the response
    pub async fn disable_heartbeat(&mut self) -> Result<String, DeribitWebSocketError> {
        // Check rate limits for public endpoints (using NonMatchingEngine type)
        self.rate_limiter
            .check_limits(EndpointType::NonMatchingEngine)
            .await?;

        let request_id = self.next_request_id.fetch_add(1, Ordering::SeqCst);
        let request = JsonRpcRequest::disable_heartbeat(request_id);

        // Create a channel for the response (not used in this simplified implementation)
        let (_response_sender, _response_receiver) = tokio::sync::oneshot::channel::<JsonRpcResponse>();

        // Send the request
        if let Some(ref mut ws) = self.websocket {
            let message_text = serde_json::to_string(&request)?;
            ws.send(Message::Text(message_text.into())).await
                .map_err(|e| DeribitWebSocketError::Connection(e.to_string()))?;
        } else {
            return Err(DeribitWebSocketError::Connection("Not connected".to_string()));
        }

        // Record the request for rate limiting
        self.rate_limiter
            .record_request(EndpointType::NonMatchingEngine)
            .await;

        // Process messages until we get our response
        let mut response_result = None;
        let timeout = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            async {
                loop {
                    if let Some(ref mut ws) = self.websocket {
                        if let Some(msg) = ws.next().await {
                            match msg {
                                Ok(Message::Text(text)) => {
                                    if let Ok(deribit_msg) = serde_json::from_str::<DeribitMessage>(&text) {
                                        if let DeribitMessage::Response(response) = deribit_msg {
                                            if response.id == request_id {
                                                response_result = Some(response);
                                                break;
                                            }
                                        }
                                    }
                                }
                                Ok(Message::Close(_)) => {
                                    self.is_connected.store(false, Ordering::SeqCst);
                                    break;
                                }
                                Err(e) => {
                                    return Err(DeribitWebSocketError::Connection(e.to_string()));
                                }
                                _ => {} // Ignore other message types
                            }
                        }
                    }
                }
                Ok(())
            }
        ).await;

        // Clean up pending request
        {
            let mut pending = self.pending_requests.lock().await;
            pending.remove(&request_id);
        }

        match timeout {
            Err(_) => Err(DeribitWebSocketError::Timeout { id: request_id }),
            Ok(Err(e)) => Err(e),
            Ok(Ok(_)) => {
                if let Some(response) = response_result {
                    // Check for JSON-RPC errors
                    if let Some(error) = response.error {
                        return Err(DeribitWebSocketError::JsonRpc {
                            code: error.code,
                            message: error.message,
                        });
                    }

                    // Return the result
                    response.result_as_string()
                        .ok_or(DeribitWebSocketError::InvalidResponse { id: request_id })
                } else {
                    Err(DeribitWebSocketError::InvalidResponse { id: request_id })
                }
            }
        }
    }
}

#[async_trait]
impl WebSocketConnection<DeribitMessage> for WebSocketClient {
    async fn connect(&mut self) -> BoxResult<()> {
        // Deribit WebSocket URL for public endpoints
        let url = "wss://www.deribit.com/ws/api/v2";
        
        let (ws_stream, _) = connect_async(url).await
            .map_err(|e| format!("Failed to connect to Deribit WebSocket: {}", e))?;

        self.websocket = Some(ws_stream);
        self.is_connected.store(true, Ordering::SeqCst);

        Ok(())
    }

    async fn disconnect(&mut self) -> BoxResult<()> {
        if let Some(ref mut ws) = self.websocket {
            ws.close(None).await
                .map_err(|e| format!("Failed to close WebSocket connection: {}", e))?;
        }
        
        self.websocket = None;
        self.is_connected.store(false, Ordering::SeqCst);
        
        // Clear pending requests
        {
            let mut pending = self.pending_requests.lock().await;
            pending.clear();
        }
        
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.is_connected.load(Ordering::SeqCst)
    }

    fn message_stream(&mut self) -> Pin<Box<dyn Stream<Item = BoxResult<DeribitMessage>> + Send>> {
        // For simplicity, return an empty stream for now
        // In a full implementation, this would yield all incoming messages
        Box::pin(futures::stream::empty())
    }
}

impl Drop for WebSocketClient {
    fn drop(&mut self) {
        // Clean up resources
        self.is_connected.store(false, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::rate_limit::AccountTier;

    #[test]
    fn test_websocket_client_creation() {
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let client = WebSocketClient::new(rate_limiter);
        
        assert!(!client.is_connected());
        assert_eq!(client.next_request_id.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_request_id_increment() {
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let client = WebSocketClient::new(rate_limiter);
        
        let id1 = client.next_request_id.fetch_add(1, Ordering::SeqCst);
        let id2 = client.next_request_id.fetch_add(1, Ordering::SeqCst);
        
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[tokio::test]
    async fn test_unsubscribe_all_without_connection() {
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let mut client = WebSocketClient::new(rate_limiter);
        
        let result = client.unsubscribe_all().await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DeribitWebSocketError::Connection(_)));
    }

    #[tokio::test]
    async fn test_disable_heartbeat_without_connection() {
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let mut client = WebSocketClient::new(rate_limiter);
        
        let result = client.disable_heartbeat().await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DeribitWebSocketError::Connection(_)));
    }

    #[test]
    fn test_deribit_websocket_error_display() {
        let error = DeribitWebSocketError::JsonRpc {
            code: -32601,
            message: "Method not found".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("-32601"));
        assert!(display.contains("Method not found"));

        let timeout_error = DeribitWebSocketError::Timeout { id: 123 };
        let display = format!("{}", timeout_error);
        assert!(display.contains("123"));
    }
}