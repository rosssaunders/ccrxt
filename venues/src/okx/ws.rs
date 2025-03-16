use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures::stream::Stream;
use std::error::Error;
use std::pin::Pin;

use super::types::{WebSocketMessage, WebSocketRequest, WebSocketChannel};
use crate::websockets::WebSocketConnection;

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
}

#[async_trait]
impl WebSocketConnection<WebSocketMessage> for OkxPublicWebSocket {
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
            // OKX requires both channel and instrument ID
            // Format expected: "channel:instId"
            let ws_channels: Vec<WebSocketChannel> = channels.iter()
                .filter_map(|channel| {
                    let parts: Vec<&str> = channel.split(':').collect();
                    if parts.len() == 2 {
                        Some(WebSocketChannel {
                            channel: parts[0].to_string(),
                            inst_id: parts[1].to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect();

            if !ws_channels.is_empty() {
                let subscribe_msg = WebSocketRequest {
                    op: "subscribe".to_string(),
                    args: ws_channels.clone(),
                };
                
                ws.send(Message::Text(serde_json::to_string(&subscribe_msg)?.into())).await?;
                self.subscribed_channels.extend(ws_channels);
            }
        }
        Ok(())
    }

    async fn unsubscribe(&mut self, channels: Vec<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(ws) = self.ws_stream.as_mut() {
            // Parse channels to unsubscribe
            let channels_to_remove: Vec<WebSocketChannel> = channels.iter()
                .filter_map(|channel| {
                    let parts: Vec<&str> = channel.split(':').collect();
                    if parts.len() == 2 {
                        Some(WebSocketChannel {
                            channel: parts[0].to_string(),
                            inst_id: parts[1].to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect();

            if !channels_to_remove.is_empty() {
                let unsubscribe_msg = WebSocketRequest {
                    op: "unsubscribe".to_string(),
                    args: channels_to_remove.clone(),
                };
                
                ws.send(Message::Text(serde_json::to_string(&unsubscribe_msg)?.into())).await?;
                
                // Remove unsubscribed channels
                self.subscribed_channels.retain(|c| {
                    !channels_to_remove.iter().any(|rc| 
                        rc.channel == c.channel && rc.inst_id == c.inst_id
                    )
                });
            }
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
                        Err(e) => {
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