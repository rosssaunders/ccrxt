use serde::{Deserialize, Serialize};
use websockets::WebSocketError;

use super::client::BinanceSpotWebSocketClient;
use super::enums::StreamType;

/// Mini ticker data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniTicker24hrData {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event time
    #[serde(rename = "E")]
    pub event_time: u64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Current close price
    #[serde(rename = "c")]
    pub close_price: String,

    /// Open price
    #[serde(rename = "o")]
    pub open_price: String,

    /// High price
    #[serde(rename = "h")]
    pub high_price: String,

    /// Low price
    #[serde(rename = "l")]
    pub low_price: String,

    /// Base volume
    #[serde(rename = "v")]
    pub volume: String,

    /// Quote volume
    #[serde(rename = "q")]
    pub quote_volume: String,
}

impl BinanceSpotWebSocketClient {
    /// Subscribe to individual symbol mini ticker stream
    /// 
    /// 24hr rolling window mini-ticker statistics for a single symbol.
    /// These are NOT the statistics of the UTC day, but a 24hr rolling window
    /// for the previous 24hrs.
    /// 
    /// # Stream Name
    /// `<symbol>@miniTicker`
    /// 
    /// # Update Speed
    /// 1000ms
    /// 
    /// # API Documentation
    /// https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#individual-symbol-mini-ticker-stream
    pub async fn subscribe_mini_ticker(&mut self, symbol: &str) -> Result<(), WebSocketError> {
        let stream = Self::build_stream_name(symbol, StreamType::MiniTicker, None);
        self.subscribe(&[stream]).await
    }
    
    /// Unsubscribe from individual symbol mini ticker stream
    pub async fn unsubscribe_mini_ticker(&mut self, symbol: &str) -> Result<(), WebSocketError> {
        let stream = Self::build_stream_name(symbol, StreamType::MiniTicker, None);
        self.unsubscribe(&[stream]).await
    }
}