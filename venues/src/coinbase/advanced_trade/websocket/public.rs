use async_trait::async_trait;
use futures::{StreamExt, stream::Stream};
use std::error::Error;
use std::pin::Pin;
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::websockets::{WebSocketConnection, BoxResult};
use super::base::BaseWebSocket;
use super::types::WebSocketMessage;
use super::error::CoinbaseAdvancedTradeError;

const COINBASE_ADVANCED_TRADE_WS_URL: &str = "wss://ws-feed.exchange.coinbase.com";

pub struct CoinbaseAdvancedTradeWebSocket {
    base: BaseWebSocket,
}

impl CoinbaseAdvancedTradeWebSocket {
    pub fn new() -> Self {
        Self {
            base: BaseWebSocket {
                url: COINBASE_ADVANCED_TRADE_WS_URL.to_string(),
                ws_stream: None,
            },
        }
    }

    pub async fn subscribe_orderbook(&mut self, product_ids: Vec<String>) -> BoxResult<()> {
        let subscription = json!({
            "type": "subscribe",
            "channels": [
                {
                    "name": "level2",
                    "product_ids": product_ids
                }
            ]
        });

        self.base.send_message(subscription).await
    }

    pub async fn unsubscribe_orderbook(&mut self, product_ids: Vec<String>) -> BoxResult<()> {
        let subscription = json!({
            "type": "unsubscribe",
            "channel": "level2",
            "product_ids": product_ids
        });

        self.base.send_message(subscription).await
    }

    pub async fn subscribe_ticker(&mut self, product_ids: Vec<String>) -> BoxResult<()> {
        let subscription = json!({
            "type": "subscribe",
            "channels": [
                {
                    "name": "ticker",
                    "product_ids": product_ids
                }
            ]
        });

        self.base.send_message(subscription).await
    }

    pub async fn unsubscribe_ticker(&mut self, product_ids: Vec<String>) -> BoxResult<()> {
        let subscription = json!({
            "type": "unsubscribe",
            "channel": "ticker",
            "product_ids": product_ids
        });

        self.base.send_message(subscription).await
    }

    pub async fn subscribe_ticker_batch(&mut self, product_ids: Vec<String>) -> BoxResult<()> {
        let subscription = json!({
            "type": "subscribe",
            "channel": "ticker_batch",
            "product_ids": product_ids
        });

        self.base.send_message(subscription).await
    }

    pub async fn unsubscribe_ticker_batch(&mut self, product_ids: Vec<String>) -> BoxResult<()> {
        let subscription = json!({
            "type": "unsubscribe",
            "channel": "ticker_batch",
            "product_ids": product_ids
        });

        self.base.send_message(subscription).await
    }

    pub async fn subscribe_market_trades(&mut self, product_ids: Vec<String>) -> BoxResult<()> {
        let subscription = json!({
            "type": "subscribe",
            "channels": [
                {
                    "name": "market_trades",
                    "product_ids": product_ids
                }
            ]
        });

        self.base.send_message(subscription).await
    }

    pub async fn unsubscribe_market_trades(&mut self, product_ids: Vec<String>) -> BoxResult<()> {
        let subscription = json!({
            "type": "unsubscribe",
            "channel": "market_trades",
            "product_ids": product_ids
        });

        self.base.send_message(subscription).await
    }

    pub async fn subscribe_candles(&mut self, product_ids: Vec<String>) -> BoxResult<()> {
        let subscription = json!({
            "type": "subscribe",
            "channel": "candles",
            "product_ids": product_ids
        });

        self.base.send_message(subscription).await
    }

    pub async fn unsubscribe_candles(&mut self, product_ids: Vec<String>) -> BoxResult<()> {
        let subscription = json!({
            "type": "unsubscribe",
            "channel": "candles",
            "product_ids": product_ids
        });

        self.base.send_message(subscription).await
    }

    pub async fn subscribe_status(&mut self, product_ids: Vec<String>) -> BoxResult<()> {
        let subscription = json!({
            "type": "subscribe",
            "channel": "status",
            "product_ids": product_ids
        });

        self.base.send_message(subscription).await
    }

    pub async fn unsubscribe_status(&mut self, product_ids: Vec<String>) -> BoxResult<()> {
        let subscription = json!({
            "type": "unsubscribe",
            "channel": "status",
            "product_ids": product_ids
        });

        self.base.send_message(subscription).await
    }

    pub async fn subscribe_heartbeats(&mut self) -> BoxResult<()> {
        let subscription = json!({
            "type": "subscribe",
            "channels": [
                {
                    "name": "heartbeats"
                }
            ]
        });

        self.base.send_message(subscription).await
    }

    pub async fn unsubscribe_heartbeats(&mut self) -> BoxResult<()> {
        let subscription = json!({
            "type": "unsubscribe",
            "channel": "heartbeats"
        });

        self.base.send_message(subscription).await
    }

    pub async fn ping(&mut self) -> BoxResult<()> {
        let ping = json!({
            "type": "ping"
        });

        self.base.send_message(ping).await
    }
}

#[async_trait]
impl WebSocketConnection<WebSocketMessage> for CoinbaseAdvancedTradeWebSocket {
    async fn connect(&mut self) -> BoxResult<()> {
        let (ws_stream, _) = connect_async(&self.base.url).await?;
        self.base.ws_stream = Some(ws_stream);
        Ok(())
    }

    async fn disconnect(&mut self) -> BoxResult<()> {
        if let Some(ws) = self.base.ws_stream.as_mut() {
            ws.close(None).await?;
        }
        self.base.ws_stream = None;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.base.ws_stream.is_some()
    }

    fn message_stream(&mut self) -> Pin<Box<dyn Stream<Item = BoxResult<WebSocketMessage>> + Send>> {
        let stream = self.base.ws_stream.take().expect("WebSocket not connected");
        
        Box::pin(stream.filter_map(|message| async move {
            match message {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str(&text) {
                        Ok(msg) => Some(Ok(msg)),
                        Err(e) => Some(Err(Box::new(CoinbaseAdvancedTradeError::ParseError(e.to_string())) as Box<dyn Error + Send + Sync>)),
                    }
                }
                Ok(Message::Close(_)) => None,
                Ok(_) => None,
                Err(e) => Some(Err(Box::new(CoinbaseAdvancedTradeError::WebSocketError(e.to_string())) as Box<dyn Error + Send + Sync>)),
            }
        }))
    }
}