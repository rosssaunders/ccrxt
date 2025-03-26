use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures::stream::Stream;
use std::error::Error;
use std::pin::Pin;
use serde_json::json;

use super::types::{WebSocketMessage, WebSocketRequest, WebSocketChannel};
use crate::websockets::{WebSocketConnection, BoxResult};

const OKX_WS_URL: &str = "wss://ws.okx.com:8443/ws/v5/public";

pub struct OkxPublicWebSocket {
    url: String,
    ws_stream: Option<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    subscribed_channels: Vec<WebSocketChannel>,
}

impl OkxPublicWebSocket {
    pub fn new() -> Self {
        Self {
            url: OKX_WS_URL.to_string(),
            ws_stream: None,
            subscribed_channels: Vec::new(),
        }
    }

    pub async fn subscribe(&mut self, channels: Vec<String>) -> BoxResult<()> {
        if let Some(ws) = self.ws_stream.as_mut() {
            let ws_channels: Vec<WebSocketChannel> = channels.iter()
                .map(|channel| {
                    let parts: Vec<&str> = channel.split(':').collect();
                    WebSocketChannel {
                        channel: parts[0].to_string(),
                        inst_id: parts[1].to_string(),
                    }
                })
                .collect();

            let subscribe_msg = json!({
                "op": "subscribe",
                "args": ws_channels
            });
            ws.send(Message::Text(subscribe_msg.to_string().into())).await?;
            self.subscribed_channels.extend(ws_channels);
        }
        Ok(())
    }
}

#[async_trait]
impl WebSocketConnection<WebSocketMessage> for OkxPublicWebSocket {
    async fn connect(&mut self) -> BoxResult<()> {
        let (ws_stream, _) = connect_async(&self.url).await?;
        self.ws_stream = Some(ws_stream);
        Ok(())
    }

    async fn disconnect(&mut self) -> BoxResult<()> {
        if let Some(ws) = self.ws_stream.as_mut() {
            ws.close(None).await?;
        }
        self.ws_stream = None;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.ws_stream.is_some()
    }

    fn message_stream(&mut self) -> Pin<Box<dyn Stream<Item = BoxResult<WebSocketMessage>> + Send>> {
        let stream = self.ws_stream.take().expect("WebSocket not connected");
        
        Box::pin(stream.filter_map(|message| async move {
            match message {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str(&text) {
                        Ok(msg) => Some(Ok(msg)),
                        Err(_e) => {
                            // Try to deserialize as raw Value as fallback
                            match serde_json::from_str(&text) {
                                Ok(raw_value) => Some(Ok(WebSocketMessage::Raw(raw_value))),
                                Err(e2) => Some(Err(Box::new(e2) as Box<dyn Error + Send + Sync>)),
                            }
                        },
                    }
                }
                Ok(Message::Close(_)) => None,
                Ok(_) => None,
                Err(e) => Some(Err(Box::new(e) as Box<dyn Error + Send + Sync>)),
            }
        }))
    }
} 