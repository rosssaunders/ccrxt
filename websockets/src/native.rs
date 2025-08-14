use std::{collections::HashMap, pin::Pin, sync::Arc};

use async_trait::async_trait;
use futures::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream, Stream},
};
use tokio::{
    net::TcpStream,
    sync::{Mutex, mpsc},
};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{self, Message, handshake::client::Request, http},
};

// Type alias for the WebSocket stream type
type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WsWriteStream = SplitSink<WsStream, Message>;

use crate::{
    VenueMessage, WebSocketConnection,
    events::{ConnectionState, DisconnectReason, WebSocketError, WebSocketEvent},
};

/// Native WebSocket client implementation using tokio-tungstenite
pub struct NativeWebSocketClient<T: VenueMessage> {
    /// WebSocket URL
    url: String,

    /// Additional headers for the connection
    headers: HashMap<String, String>,

    /// Write half of the WebSocket stream
    write_stream: Option<Arc<Mutex<WsWriteStream>>>,

    /// Current connection state
    state: ConnectionState,

    /// Event channel sender
    event_tx: mpsc::UnboundedSender<WebSocketEvent<T>>,

    /// Event channel receiver
    event_rx: Option<mpsc::UnboundedReceiver<WebSocketEvent<T>>>,

    /// Task handle for the message reader
    read_task: Option<tokio::task::JoinHandle<()>>,
}

impl<T: VenueMessage + 'static> NativeWebSocketClient<T> {
    /// Create a new native WebSocket client
    pub fn new(url: impl Into<String>) -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        Self {
            url: url.into(),
            headers: HashMap::new(),
            write_stream: None,
            state: ConnectionState::Disconnected,
            event_tx,
            event_rx: Some(event_rx),
            read_task: None,
        }
    }

    /// Add a header to the WebSocket connection request
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Build the connection request with headers
    fn build_request(&self) -> Result<Request, WebSocketError> {
        let url = self
            .url
            .parse::<http::Uri>()
            .map_err(|e| WebSocketError::InvalidUrl(e.to_string()))?;

        let host = url
            .host()
            .ok_or_else(|| WebSocketError::InvalidUrl("Missing host in URL".to_string()))?;

        let mut request = Request::builder()
            .uri(&self.url)
            .header("Host", host)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header(
                "Sec-WebSocket-Key",
                tungstenite::handshake::client::generate_key(),
            );

        // Add custom headers
        for (key, value) in &self.headers {
            request = request.header(key.as_str(), value.as_str());
        }

        request
            .body(())
            .map_err(|e| WebSocketError::ConnectionFailed(e.to_string()))
    }

    /// Start the message reading task
    fn start_read_task(&mut self, mut read_stream: SplitStream<WsStream>) {
        let event_tx = self.event_tx.clone();

        let read_task = tokio::spawn(async move {
            while let Some(result) = read_stream.next().await {
                match result {
                    Ok(msg) => {
                        match msg {
                            tungstenite::Message::Text(text) => {
                                // Try to deserialize the message
                                match serde_json::from_str::<T>(&text) {
                                    Ok(venue_msg) => {
                                        let _ = event_tx
                                            .send(WebSocketEvent::Message { message: venue_msg });
                                    }
                                    Err(e) => {
                                        let _ = event_tx.send(WebSocketEvent::Error {
                                            error: WebSocketError::DeserializationError(
                                                e.to_string(),
                                            ),
                                        });
                                    }
                                }
                            }
                            tungstenite::Message::Binary(data) => {
                                // Try to deserialize binary message
                                match serde_json::from_slice::<T>(&data) {
                                    Ok(venue_msg) => {
                                        let _ = event_tx
                                            .send(WebSocketEvent::Message { message: venue_msg });
                                    }
                                    Err(e) => {
                                        let _ = event_tx.send(WebSocketEvent::Error {
                                            error: WebSocketError::DeserializationError(
                                                e.to_string(),
                                            ),
                                        });
                                    }
                                }
                            }
                            tungstenite::Message::Ping(data) => {
                                let _ = event_tx.send(WebSocketEvent::PingReceived {
                                    data: data.to_vec(),
                                });
                            }
                            tungstenite::Message::Pong(data) => {
                                let _ = event_tx.send(WebSocketEvent::PongReceived {
                                    data: data.to_vec(),
                                });
                            }
                            tungstenite::Message::Close(frame) => {
                                let reason = if let Some(frame) = frame {
                                    DisconnectReason::RemoteClosed {
                                        code: frame.code.into(),
                                        reason: frame.reason.to_string(),
                                    }
                                } else {
                                    DisconnectReason::RemoteClosed {
                                        code: 1000,
                                        reason: "Normal closure".to_string(),
                                    }
                                };
                                let _ = event_tx.send(WebSocketEvent::Disconnected { reason });
                                break;
                            }
                            tungstenite::Message::Frame(_) => {
                                // Raw frame - typically not used
                            }
                        }
                    }
                    Err(e) => {
                        let reason = match e {
                            tungstenite::Error::ConnectionClosed => {
                                DisconnectReason::RemoteClosed {
                                    code: 1006,
                                    reason: "Connection closed abnormally".to_string(),
                                }
                            }
                            tungstenite::Error::Protocol(err) => DisconnectReason::ProtocolError {
                                details: err.to_string(),
                            },
                            _ => DisconnectReason::NetworkError {
                                details: e.to_string(),
                            },
                        };
                        let _ = event_tx.send(WebSocketEvent::Disconnected { reason });
                        break;
                    }
                }
            }
        });

        self.read_task = Some(read_task);
    }
}

#[async_trait]
impl<T: VenueMessage + 'static> WebSocketConnection<T> for NativeWebSocketClient<T> {
    async fn connect(&mut self) -> Result<(), WebSocketError> {
        if self.state == ConnectionState::Connected {
            return Err(WebSocketError::AlreadyConnected);
        }

        self.state = ConnectionState::Connecting;

        // Build the request with headers
        let request = self.build_request()?;

        // Connect to the WebSocket
        let (ws_stream, _response) = connect_async(request)
            .await
            .map_err(|e| WebSocketError::ConnectionFailed(e.to_string()))?;

        // Split the stream
        let (write_half, read_half) = ws_stream.split();

        // Store the write half
        self.write_stream = Some(Arc::new(Mutex::new(write_half)));

        self.state = ConnectionState::Connected;

        // Start the message reading task with the read half
        self.start_read_task(read_half);

        // Send connected event
        let _ = self.event_tx.send(WebSocketEvent::Connected);

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), WebSocketError> {
        if self.state != ConnectionState::Connected {
            return Ok(());
        }

        self.state = ConnectionState::Disconnecting;

        // Cancel the read task
        if let Some(task) = self.read_task.take() {
            task.abort();
        }

        // Close the WebSocket connection
        if let Some(write_stream) = self.write_stream.take() {
            let mut writer = write_stream.lock().await;
            let _ = writer.close().await;
        }

        self.state = ConnectionState::Disconnected;

        // Send disconnected event
        let _ = self.event_tx.send(WebSocketEvent::Disconnected {
            reason: DisconnectReason::UserInitiated,
        });

        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.state == ConnectionState::Connected
    }

    fn connection_state(&self) -> ConnectionState {
        self.state
    }

    fn event_stream(&mut self) -> Pin<Box<dyn Stream<Item = WebSocketEvent<T>> + Send>> {
        // Take the receiver and create a stream from it
        if let Some(rx) = self.event_rx.take() {
            Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(rx))
        } else {
            // If receiver was already taken, create an empty stream
            Box::pin(futures::stream::empty())
        }
    }

    async fn send(&mut self, message: T) -> Result<(), WebSocketError> {
        if self.state != ConnectionState::Connected {
            return Err(WebSocketError::NotConnected);
        }

        // Serialize the message
        let json = serde_json::to_string(&message)
            .map_err(|e| WebSocketError::SerializationError(e.to_string()))?;

        // Send using the write stream
        if let Some(write_stream) = &self.write_stream {
            let mut writer = write_stream.lock().await;
            writer
                .send(tungstenite::Message::Text(json.into()))
                .await
                .map_err(|e| WebSocketError::SendFailed(e.to_string()))?;
        } else {
            return Err(WebSocketError::NotConnected);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct TestMessage {
        content: String,
    }

    impl VenueMessage for TestMessage {}

    #[test]
    fn test_client_creation() {
        let client: NativeWebSocketClient<TestMessage> =
            NativeWebSocketClient::new("wss://example.com");

        assert_eq!(client.connection_state(), ConnectionState::Disconnected);
        assert!(!client.is_connected());
    }

    #[test]
    fn test_client_with_headers() {
        let client: NativeWebSocketClient<TestMessage> =
            NativeWebSocketClient::new("wss://example.com")
                .with_header("Authorization".to_string(), "Bearer token".to_string());

        assert!(client.headers.contains_key("Authorization"));
    }
}
