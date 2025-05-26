use async_trait::async_trait;
use futures::stream::Stream;
use futures::{SinkExt, StreamExt};
use serde_json::json;
use std::error::Error;
use std::pin::Pin;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use websockets::BoxError;

use super::types::WebSocketMessage;
use crate::websockets::{BoxResult, WebSocketConnection};

const BINANCE_SPOT_WS_URL: &str = "wss://stream.binance.com:9443/ws";

pub struct BinanceSpotPublicWebSocket {
    url: String,
    ws_stream: Option<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    >,
    subscribed_channels: Vec<String>,
}

impl BinanceSpotPublicWebSocket {
    pub fn new() -> Self {
        Self {
            url: BINANCE_SPOT_WS_URL.to_string(),
            ws_stream: None,
            subscribed_channels: Vec::new(),
        }
    }

    pub async fn subscribe_depth(&mut self, symbol: &str) -> BoxResult<()> {
        let channel = format!("{}@depth", symbol.to_lowercase());
        self.subscribe(vec![channel]).await
    }

    pub async fn subscribe(&mut self, channels: Vec<String>) -> BoxResult<()> {
        if let Some(ws) = self.ws_stream.as_mut() {
            let subscribe_msg = json!({
                "method": "SUBSCRIBE",
                "params": channels,
                "id": 1
            });
            ws.send(Message::Text(subscribe_msg.to_string().into()))
                .await?;
            self.subscribed_channels.extend(channels);
        }
        Ok(())
    }
}

#[async_trait]
impl WebSocketConnection<WebSocketMessage> for BinanceSpotPublicWebSocket {
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

    fn message_stream(
        &mut self,
    ) -> Pin<Box<dyn Stream<Item = BoxResult<WebSocketMessage>> + Send>> {
        let stream = self.ws_stream.take().expect("WebSocket not connected");

        Box::pin(stream.filter_map(|message| async move {
            match message {
                Ok(Message::Text(text)) => match serde_json::from_str(&text) {
                    Ok(msg) => Some(Ok(msg)),
                    Err(e) => Some(Err(BoxError::from(e))),
                },
                Ok(Message::Close(_)) => None,
                Ok(_) => None,
                Err(e) => Some(Err(BoxError::from(e))),
            }
        }))
    }
}
