#![cfg(not(target_arch = "wasm32"))]

use std::time::Duration;

use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;

use crate::client::{IncomingMessage, WebSocketClient, WebSocketError, WebSocketResult};

/// A simple native WebSocket client backed by tokio-tungstenite.
pub struct NativeWebSocketClient {
    ws: Option<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>,
    receive_timeout: Duration,
}

impl Default for NativeWebSocketClient {
    fn default() -> Self {
        Self {
            ws: None,
            receive_timeout: Duration::from_secs(60),
        }
    }
}

impl NativeWebSocketClient {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl WebSocketClient for NativeWebSocketClient {
    async fn connect(&mut self, url: &str) -> WebSocketResult<()> {
        let (ws, _resp) = connect_async(url)
            .await
            .map_err(|e| WebSocketError::Connection(e.to_string()))?;
        self.ws = Some(ws);
        Ok(())
    }

    async fn disconnect(&mut self) -> WebSocketResult<()> {
        if let Some(mut ws) = self.ws.take() {
            let _ = ws.close(None).await; // ignore close errors
        }
        Ok(())
    }

    async fn send(&mut self, message: bytes::Bytes) -> WebSocketResult<()> {
        if let Some(ws) = &mut self.ws {
            ws.send(WsMessage::Binary(message))
                .await
                .map_err(|e| WebSocketError::Send(e.to_string()))
        } else {
            Err(WebSocketError::NotConnected)
        }
    }

    async fn receive(&mut self) -> WebSocketResult<Option<IncomingMessage>> {
        if let Some(ws) = &mut self.ws {
            match timeout(self.receive_timeout, ws.next()).await {
                Ok(Some(Ok(WsMessage::Binary(b)))) => Ok(Some(IncomingMessage::Binary(b))),
                Ok(Some(Ok(WsMessage::Text(t)))) => Ok(Some(IncomingMessage::Text(t.to_string()))),
                Ok(Some(Ok(WsMessage::Ping(_)))) => Ok(None),
                Ok(Some(Ok(WsMessage::Pong(_)))) => Ok(None),
                Ok(Some(Ok(WsMessage::Close(_)))) => Ok(None),
                Ok(Some(Ok(WsMessage::Frame(_)))) => Ok(None),
                Ok(Some(Err(e))) => Err(WebSocketError::Receive(e.to_string())),
                Ok(None) => Ok(None),
                Err(_) => Err(WebSocketError::Timeout),
            }
        } else {
            Err(WebSocketError::NotConnected)
        }
    }

    fn is_connected(&self) -> bool {
        self.ws.is_some()
    }
}
