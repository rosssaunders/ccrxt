use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures::stream::Stream;
use serde_json::json;
use std::error::Error;
use super::{
    types::WebSocketMessage,
    api_errors::{BinanceCoinMError, BinanceCoinMResult},
};

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

    pub async fn subscribe_depth(&mut self, symbol: &str) -> BinanceCoinMResult<()> {
        let channel = format!("{}@depth", symbol.to_lowercase());
        self.subscribe(vec![channel]).await
    }

    pub async fn subscribe_ticker(&mut self, symbol: &str) -> BinanceCoinMResult<()> {
        let channel = format!("{}@ticker", symbol.to_lowercase());
        self.subscribe(vec![channel]).await
    }

    pub async fn subscribe_trades(&mut self, symbol: &str) -> BinanceCoinMResult<()> {
        let channel = format!("{}@trade", symbol.to_lowercase());
        self.subscribe(vec![channel]).await
    }

    async fn subscribe(&mut self, channels: Vec<String>) -> BinanceCoinMResult<()> {
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
}

#[async_trait]
impl crate::websockets::WebSocketConnection<WebSocketMessage> for BinanceCoinMPublicWebSocket {
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

    fn message_stream(&mut self) -> std::pin::Pin<Box<dyn Stream<Item = Result<WebSocketMessage, Box<dyn Error + Send + Sync>>> + Send>> {
        let stream = self.ws_stream.take().expect("WebSocket not connected");
        
        Box::pin(stream.filter_map(|message| async move {
            match message {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str(&text) {
                        Ok(msg) => Some(Ok(msg)),
                        Err(e) => Some(Err(Box::new(BinanceCoinMError::SerializationError(e)))),
                    }
                }
                Ok(Message::Close(_)) => None,
                Ok(_) => None,
                Err(e) => Some(Err(Box::new(BinanceCoinMError::WebSocketError(e)))),
            }
        }))
    }
} 