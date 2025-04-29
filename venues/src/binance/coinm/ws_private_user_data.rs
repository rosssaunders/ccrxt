use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures::stream::Stream;
use serde_json::json;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;
use std::error::Error;
use super::{
    types::WebSocketMessage,
    errors::{BinanceCoinMError, BinanceCoinMResult},
};

pub struct BinanceCoinMPrivateWebSocket {
    base_url: String,
    ws_stream: Option<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    subscribed_channels: Vec<String>,
    api_key: String,
    api_secret: String,
}

impl BinanceCoinMPrivateWebSocket {
    pub fn new(base_url: String, api_key: String, api_secret: String) -> Self {
        Self {
            base_url,
            ws_stream: None,
            subscribed_channels: Vec::new(),
            api_key,
            api_secret,
        }
    }

    fn sign_request(&self, query_string: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(query_string.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    pub async fn subscribe_user_data(&mut self) -> BinanceCoinMResult<()> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let query_string = format!("timestamp={}", timestamp);
        let signature = self.sign_request(&query_string);
        
        let listen_key = self.get_listen_key().await?;
        let channel = format!("{}", listen_key);
        self.subscribe(vec![channel]).await
    }

    async fn get_listen_key(&self) -> BinanceCoinMResult<String> {
        let url = format!("{}/dapi/v1/listenKey", self.base_url);
        
        let response = reqwest::Client::new()
            .post(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?
            .error_for_status()?;
            
        let response: serde_json::Value = response.json().await?;
        Ok(response["listenKey"].as_str().unwrap_or_default().to_string())
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
impl crate::websockets::WebSocketConnection<WebSocketMessage> for BinanceCoinMPrivateWebSocket {
    async fn connect(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (ws_stream, _) = connect_async(&self.base_url).await?;
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