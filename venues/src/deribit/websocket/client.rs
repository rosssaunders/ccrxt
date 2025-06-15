use crate::deribit::websocket::{
    DeribitMessage, DisableHeartbeatRequest, DisableHeartbeatResponse, JsonRpcResponse,
    RequestIdGenerator, WebSocketConfig, WebSocketError, WebSocketResult,
};
use async_trait::async_trait;
use futures::{stream::SplitSink, SinkExt, Stream, StreamExt};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::timeout;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use websockets::{BoxError, BoxResult, WebSocketConnection};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WsSink = SplitSink<WsStream, Message>;

/// Deribit WebSocket client implementing the common WebSocketConnection trait
pub struct WebSocketClient {
    /// WebSocket configuration
    config: WebSocketConfig,
    
    /// Request ID generator
    id_generator: RequestIdGenerator,
    
    /// WebSocket sink for sending messages
    sink: Arc<Mutex<Option<WsSink>>>,
    
    /// Connection state
    is_connected: Arc<RwLock<bool>>,
    
    /// Pending requests waiting for responses
    pending_requests: Arc<Mutex<HashMap<i64, mpsc::Sender<JsonRpcResponse>>>>,
    
    /// Message receiver for incoming messages
    message_receiver: Arc<Mutex<Option<mpsc::Receiver<BoxResult<DeribitMessage>>>>>,
    
    /// Background task handle for message processing
    task_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

impl WebSocketClient {
    /// Create a new Deribit WebSocket client
    pub fn new() -> Self {
        Self::with_config(WebSocketConfig::default())
    }
    
    /// Create a new Deribit WebSocket client with custom configuration
    pub fn with_config(config: WebSocketConfig) -> Self {
        Self {
            config,
            id_generator: RequestIdGenerator::new(),
            sink: Arc::new(Mutex::new(None)),
            is_connected: Arc::new(RwLock::new(false)),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            message_receiver: Arc::new(Mutex::new(None)),
            task_handle: Arc::new(Mutex::new(None)),
        }
    }
    
    /// Send a disable_heartbeat request and wait for response
    pub async fn disable_heartbeat(&self) -> WebSocketResult<DisableHeartbeatResponse> {
        if !self.is_connected().await {
            return Err(WebSocketError::ConnectionError("Not connected".to_string()));
        }
        
        let request_id = self.id_generator.next();
        let request = DisableHeartbeatRequest::new(request_id);
        
        // Serialize the request
        let request_json = serde_json::to_string(&request)?;
        
        // Create a channel to receive the response
        let (tx, mut rx) = mpsc::channel(1);
        
        // Register the request
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(request_id, tx);
        }
        
        // Send the request
        {
            let mut sink = self.sink.lock().await;
            if let Some(sink) = sink.as_mut() {
                sink.send(Message::Text(request_json.into()))
                    .await
                    .map_err(|e| WebSocketError::ConnectionError(format!("Failed to send message: {}", e)))?;
            } else {
                return Err(WebSocketError::ConnectionError("WebSocket sink not available".to_string()));
            }
        }
        
        // Wait for response with timeout
        let response = timeout(Duration::from_millis(self.config.request_timeout_ms), rx.recv())
            .await
            .map_err(|_| WebSocketError::Timeout)?
            .ok_or_else(|| WebSocketError::InvalidResponse("No response received".to_string()))?;
        
        // Check for errors
        if let Some(error) = response.error {
            return Err(WebSocketError::InvalidResponse(format!("Server error: {} - {}", error.code, error.message)));
        }
        
        // Extract result
        let result = response.result
            .ok_or_else(|| WebSocketError::InvalidResponse("No result in response".to_string()))?;
        
        let result_str = result.as_str()
            .ok_or_else(|| WebSocketError::InvalidResponse("Result is not a string".to_string()))?;
        
        Ok(DisableHeartbeatResponse {
            jsonrpc: response.jsonrpc,
            id: response.id,
            result: result_str.to_string(),
        })
    }
    
    /// Check if the WebSocket is connected
    async fn is_connected(&self) -> bool {
        *self.is_connected.read().await
    }
    
    /// Process incoming WebSocket messages
    async fn process_messages(
        mut stream: futures::stream::SplitStream<WsStream>,
        message_sender: mpsc::Sender<BoxResult<DeribitMessage>>,
        pending_requests: Arc<Mutex<HashMap<i64, mpsc::Sender<JsonRpcResponse>>>>,
        is_connected: Arc<RwLock<bool>>,
    ) {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str::<JsonRpcResponse>(&text) {
                        Ok(response) => {
                            // Check if this is a response to a pending request
                            let mut pending = pending_requests.lock().await;
                            if let Some(sender) = pending.remove(&response.id) {
                                let _ = sender.send(response).await;
                            } else {
                                // Convert to DeribitMessage and forward
                                let message = DeribitMessage::JsonRpcResponse(response);
                                if message_sender.send(Ok(message)).await.is_err() {
                                    break; // Receiver dropped
                                }
                            }
                        }
                        Err(e) => {
                            let error: BoxError = Box::new(WebSocketError::SerializationError(e));
                            if message_sender.send(Err(error)).await.is_err() {
                                break; // Receiver dropped
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    *is_connected.write().await = false;
                    break;
                }
                Ok(Message::Ping(data)) => {
                    // WebSocket library should handle pong automatically
                    tracing::debug!("Received ping with {} bytes", data.len());
                }
                Ok(Message::Pong(_)) => {
                    tracing::debug!("Received pong");
                }
                Ok(Message::Binary(_)) => {
                    // Deribit uses text messages for JSON-RPC
                    tracing::warn!("Received unexpected binary message");
                }
                Ok(Message::Frame(_)) => {
                    // Raw frame messages - typically handled by the library
                    tracing::debug!("Received raw frame message");
                }
                Err(e) => {
                    *is_connected.write().await = false;
                    let error: BoxError = Box::new(WebSocketError::ProtocolError(format!("WebSocket error: {}", e)));
                    if message_sender.send(Err(error)).await.is_err() {
                        break; // Receiver dropped
                    }
                    break;
                }
            }
        }
        
        *is_connected.write().await = false;
    }
}

impl Default for WebSocketClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WebSocketConnection<DeribitMessage> for WebSocketClient {
    async fn connect(&mut self) -> BoxResult<()> {
        // Connect to WebSocket
        let (ws_stream, _) = connect_async(&self.config.url)
            .await
            .map_err(|e| -> BoxError { Box::new(WebSocketError::ConnectionError(format!("Failed to connect: {}", e))) })?;
        
        let (sink, stream) = ws_stream.split();
        
        // Set up message processing
        let (message_sender, message_receiver) = mpsc::channel(100);
        
        // Store the sink
        *self.sink.lock().await = Some(sink);
        
        // Store the message receiver
        *self.message_receiver.lock().await = Some(message_receiver);
        
        // Update connection state
        *self.is_connected.write().await = true;
        
        // Start message processing task
        let pending_requests = Arc::clone(&self.pending_requests);
        let is_connected = Arc::clone(&self.is_connected);
        let handle = tokio::spawn(Self::process_messages(
            stream,
            message_sender,
            pending_requests,
            is_connected,
        ));
        
        *self.task_handle.lock().await = Some(handle);
        
        Ok(())
    }
    
    async fn disconnect(&mut self) -> BoxResult<()> {
        // Set connection state to false
        *self.is_connected.write().await = false;
        
        // Close WebSocket connection
        if let Some(mut sink) = self.sink.lock().await.take() {
            let _ = sink.close().await;
        }
        
        // Cancel background task
        if let Some(handle) = self.task_handle.lock().await.take() {
            handle.abort();
        }
        
        // Clear pending requests
        self.pending_requests.lock().await.clear();
        
        // Clear message receiver
        *self.message_receiver.lock().await = None;
        
        Ok(())
    }
    
    fn is_connected(&self) -> bool {
        // Use try_read to avoid blocking - if we can't get the lock immediately, assume not connected
        // This is safe for the trait interface since connection state doesn't change frequently
        match self.is_connected.try_read() {
            Ok(connected) => *connected,
            Err(_) => false, // If we can't read the state, assume not connected
        }
    }
    
    fn message_stream(&mut self) -> Pin<Box<dyn Stream<Item = BoxResult<DeribitMessage>> + Send>> {
        let receiver = self.message_receiver.clone();
        
        Box::pin(async_stream::stream! {
            loop {
                let mut receiver_guard = receiver.lock().await;
                if let Some(receiver) = receiver_guard.as_mut() {
                    match receiver.recv().await {
                        Some(message) => yield message,
                        None => break, // Channel closed
                    }
                } else {
                    break; // No receiver available
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = WebSocketClient::new();
        assert!(!client.is_connected().await);
        
        let custom_config = WebSocketConfig {
            url: "wss://test.example.com".to_string(),
            request_timeout_ms: 5000,
            ..Default::default()
        };
        
        let client = WebSocketClient::with_config(custom_config);
        assert!(!client.is_connected().await);
    }

    #[tokio::test]
    async fn test_disable_heartbeat_request_creation() {
        let client = WebSocketClient::new();
        let request_id = client.id_generator.next();
        let request = DisableHeartbeatRequest::new(request_id);
        
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.id, request_id);
        assert_eq!(request.method, "public/disable_heartbeat");
    }

    #[test]
    fn test_default_config() {
        let config = WebSocketConfig::default();
        assert_eq!(config.url, "wss://www.deribit.com/ws/api/v2");
        assert_eq!(config.request_timeout_ms, 30000);
        assert!(config.auto_reconnect);
    }

    #[tokio::test]
    async fn test_disconnect_when_not_connected() {
        let mut client = WebSocketClient::new();
        // Should not panic when disconnecting while not connected
        assert!(client.disconnect().await.is_ok());
    }

    #[test]
    fn test_request_id_generator_sequence() {
        let generator = RequestIdGenerator::new();
        assert_eq!(generator.next(), 1);
        assert_eq!(generator.next(), 2);
        assert_eq!(generator.next(), 3);
    }
}