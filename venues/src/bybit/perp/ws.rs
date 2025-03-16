use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures::stream::Stream;
use std::error::Error;
use std::pin::Pin;
use std::time::{Duration, Instant};
use serde_json::json;
use tokio::time::interval;

use super::types::WebSocketMessage;
use crate::websockets::WebSocketConnection;

const BYBIT_PERP_WS_URL: &str = "wss://stream.bybit.com/v5/public/linear";
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(20);

pub struct BybitPerpPublicWebSocket {
    url: String,
    ws_stream: Option<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    subscribed_channels: Vec<String>,
    last_heartbeat: Instant,
}

impl BybitPerpPublicWebSocket {
    pub fn new() -> Self {
        Self {
            url: BYBIT_PERP_WS_URL.to_string(),
            ws_stream: None,
            subscribed_channels: Vec::new(),
            last_heartbeat: Instant::now(),
        }
    }
    
    async fn send_heartbeat(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(ws) = self.ws_stream.as_mut() {
            let ping_msg = json!({
                "req_id": "heartbeat",
                "op": "ping"
            });
            ws.send(Message::Text(ping_msg.to_string().into())).await?;
            self.last_heartbeat = Instant::now();
        }
        Ok(())
    }
}

#[async_trait]
impl WebSocketConnection<WebSocketMessage> for BybitPerpPublicWebSocket {
    async fn connect(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (ws_stream, _) = connect_async(&self.url).await?;
        self.ws_stream = Some(ws_stream);
        self.last_heartbeat = Instant::now();
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
                "op": "subscribe",
                "args": channels
            });
            ws.send(Message::Text(subscribe_msg.to_string().into())).await?;
            self.subscribed_channels.extend(channels);
            
            // Send initial heartbeat
            self.send_heartbeat().await?;
        }
        Ok(())
    }

    async fn unsubscribe(&mut self, channels: Vec<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(ws) = self.ws_stream.as_mut() {
            let unsubscribe_msg = json!({
                "op": "unsubscribe",
                "args": channels
            });
            ws.send(Message::Text(unsubscribe_msg.to_string().into())).await?;
            self.subscribed_channels.retain(|c| !channels.contains(c));
        }
        Ok(())
    }

    fn message_stream(&mut self) -> Pin<Box<dyn Stream<Item = Result<WebSocketMessage, Box<dyn Error + Send + Sync>>> + Send>> {
        let stream = self.ws_stream.take().expect("WebSocket not connected");
        let mut heartbeat_interval = interval(HEARTBEAT_INTERVAL);
        let mut last_heartbeat = self.last_heartbeat;
        
        Box::pin(stream.filter_map(move |message| {
            let now = Instant::now();
            
            // Check if we need to send a heartbeat
            if now.duration_since(last_heartbeat) >= HEARTBEAT_INTERVAL {
                last_heartbeat = now;
            }
            
            async move {
                match message {
                    Ok(Message::Text(text)) => {
                        match serde_json::from_str::<WebSocketMessage>(&text) {
                            Ok(msg) => Some(Ok(msg)),
                            Err(_) => {
                                // Try to deserialize as raw Value as fallback
                                match serde_json::from_str(&text) {
                                    Ok(raw_value) => Some(Ok(WebSocketMessage::Raw(raw_value))),
                                    Err(e) => Some(Err(Box::new(e) as Box<dyn Error + Send + Sync>)),
                                }
                            },
                        }
                    }
                    Ok(Message::Close(_)) => None,
                    Ok(Message::Ping(_)) => None,
                    Ok(Message::Pong(_)) => None,
                    Ok(_) => None,
                    Err(e) => Some(Err(Box::new(e) as Box<dyn Error + Send + Sync>)),
                }
            }
        }))
    }
}

impl Default for BybitPerpPublicWebSocket {
    fn default() -> Self {
        Self::new()
    }
} 