use async_trait::async_trait;
use futures::{stream::Stream, StreamExt};
use serde_json::json;
use std::error::Error;
use std::pin::Pin;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use super::base::BaseWebSocket;
use super::error::CoinbaseAdvancedTradeError;
use super::types::WebSocketMessage;
use crate::websockets::{BoxResult, WebSocketConnection};

const COINBASE_ADVANCED_TRADE_AUTH_WS_URL: &str = "wss://advanced-trade-ws-user.coinbase.com";

pub struct CoinbaseAdvancedTradeWebSocketAuthenticated {
    base: BaseWebSocket,
    jwt: String,
}

impl CoinbaseAdvancedTradeWebSocketAuthenticated {
    pub fn new(jwt: String) -> Self {
        Self {
            base: BaseWebSocket {
                url: COINBASE_ADVANCED_TRADE_AUTH_WS_URL.to_string(),
                ws_stream: None,
            },
            jwt,
        }
    }

    pub async fn subscribe_user(&mut self, product_ids: Option<Vec<String>>) -> BoxResult<()> {
        let mut subscription = json!({
            "type": "subscribe",
            "channel": "user",
            "jwt": self.jwt
        });

        if let Some(ids) = product_ids {
            subscription["product_ids"] = json!(ids);
        }

        self.base.send_message(subscription).await
    }

    pub async fn unsubscribe_user(&mut self, product_ids: Option<Vec<String>>) -> BoxResult<()> {
        let mut subscription = json!({
            "type": "unsubscribe",
            "channel": "user",
            "jwt": self.jwt
        });

        if let Some(ids) = product_ids {
            subscription["product_ids"] = json!(ids);
        }

        self.base.send_message(subscription).await
    }

    pub async fn subscribe_futures_balance_summary(&mut self) -> BoxResult<()> {
        let subscription = json!({
            "type": "subscribe",
            "channel": "futures_balance_summary",
            "jwt": self.jwt
        });

        self.base.send_message(subscription).await
    }

    pub async fn unsubscribe_futures_balance_summary(&mut self) -> BoxResult<()> {
        let subscription = json!({
            "type": "unsubscribe",
            "channel": "futures_balance_summary",
            "jwt": self.jwt
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
impl WebSocketConnection<WebSocketMessage> for CoinbaseAdvancedTradeWebSocketAuthenticated {
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

    fn message_stream(
        &mut self,
    ) -> Pin<Box<dyn Stream<Item = BoxResult<WebSocketMessage>> + Send>> {
        let stream = self.base.ws_stream.take().expect("WebSocket not connected");

        Box::pin(stream.filter_map(|message| async move {
            match message {
                Ok(Message::Text(text)) => match serde_json::from_str(&text) {
                    Ok(msg) => Some(Ok(msg)),
                    Err(e) => Some(Err(Box::new(CoinbaseAdvancedTradeError::ParseError(
                        e.to_string(),
                    )) as Box<dyn Error + Send + Sync>)),
                },
                Ok(Message::Close(_)) => None,
                Ok(_) => None,
                Err(e) => Some(Err(Box::new(CoinbaseAdvancedTradeError::WebSocketError(
                    e.to_string(),
                )) as Box<dyn Error + Send + Sync>)),
            }
        }))
    }
}
