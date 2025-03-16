use serde::{Deserialize};
use serde_json::{Value, json};
use futures::{SinkExt, StreamExt, stream::Stream};
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream, MaybeTlsStream};
use tokio::net::TcpStream;
use std::error::Error;
use std::pin::Pin;
use async_trait::async_trait;

use crate::websockets::WebSocketConnection;

const WEBSOCKET_BASE_URL: &str = "wss://fstream.binance.com/ws";

#[derive(Debug, Deserialize)]
pub struct OrderBookUpdate {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    #[serde(rename = "b")]
    pub bids: Vec<(String, String)>,
    #[serde(rename = "a")]
    pub asks: Vec<(String, String)>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum WebSocketMessage {
    OrderBook(OrderBookUpdate),
    Raw(Value),
}

impl crate::websockets::VenueMessage for WebSocketMessage {}

pub struct BinanceUsdMPublicWebSocket {
    url: String,
    ws_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    subscribed_channels: Vec<String>,
}

impl BinanceUsdMPublicWebSocket {
    pub fn new() -> Self {
        Self {
            url: WEBSOCKET_BASE_URL.to_string(),
            ws_stream: None,
            subscribed_channels: Vec::new(),
        }
    }
}

#[async_trait]
impl WebSocketConnection<WebSocketMessage> for BinanceUsdMPublicWebSocket {
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

impl Default for BinanceUsdMPublicWebSocket {
    fn default() -> Self {
        Self::new()
    }
} 