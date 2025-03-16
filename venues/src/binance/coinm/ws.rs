use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures::stream::Stream;
use std::error::Error;
use std::pin::Pin;
use serde_json::json;

use super::types::WebSocketMessage;
use crate::websockets::WebSocketConnection;

const BINANCE_COINM_WS_URL: &str = "wss://dstream.binance.com/ws";

pub struct BinanceCoinMPublicWebSocket {
    url: String,
    ws_stream: Option<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    subscribed_channels: Vec<String>,
}

impl BinanceCoinMPublicWebSocket {
    pub fn new() -> Self {
        Self {
            url: BINANCE_COINM_WS_URL.to_string(),
            ws_stream: None,
            subscribed_channels: Vec::new(),
        }
    }
}

#[async_trait]
impl WebSocketConnection<WebSocketMessage> for BinanceCoinMPublicWebSocket {
    async fn connect(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (ws_stream, _) = connect_async(&self.url).await?;
        self.ws_stream = Some(ws_stream);
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(ws) = self.ws_stream.as_mut() {
            ws.close(None).await?;
        }
        self.ws_stream = None;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.ws_stream.is_some()
    }

    async fn subscribe(&mut self, channels: Vec<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(ws) = self.ws_stream.as_mut() {
            let subscribe_msg = json!({
                "method": "SUBSCRIBE",
                "params": channels,
                "id": 1
            });
            ws.send(Message::Text(subscribe_msg.to_string().into())).await?;
            self.subscribed_channels.extend(channels);
        }
        Ok(())
    }

    async fn unsubscribe(&mut self, channels: Vec<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(ws) = self.ws_stream.as_mut() {
            let unsubscribe_msg = json!({
                "method": "UNSUBSCRIBE",
                "params": channels,
                "id": 1
            });
            ws.send(Message::Text(unsubscribe_msg.to_string().into())).await?;
            self.subscribed_channels.retain(|c| !channels.contains(c));
        }
        Ok(())
    }

    fn message_stream(&mut self) -> Pin<Box<dyn Stream<Item = Result<WebSocketMessage, Box<dyn Error + Send + Sync>>> + Send>> {
        let stream = self.ws_stream.take().expect("WebSocket not connected");
        
        Box::pin(stream.filter_map(|message| async move {
            match message {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str(&text) {
                        Ok(msg) => Some(Ok(msg)),
                        Err(e) => Some(Err(Box::new(e) as Box<dyn Error + Send + Sync>)),
                    }
                }
                Ok(Message::Close(_)) => None,
                Ok(_) => None,
                Err(e) => Some(Err(Box::new(e) as Box<dyn Error + Send + Sync>)),
            }
        }))
    }
} 