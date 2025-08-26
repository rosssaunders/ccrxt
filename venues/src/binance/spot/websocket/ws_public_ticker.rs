use serde::{Deserialize, Serialize};
use websockets::WebSocketError;

use super::client::BinanceSpotWebSocketClient;
use super::enums::StreamType;

/// 24hr ticker data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker24hrData {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event time
    #[serde(rename = "E")]
    pub event_time: u64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Price change
    #[serde(rename = "p")]
    pub price_change: String,

    /// Price change percent
    #[serde(rename = "P")]
    pub price_change_percent: String,

    /// Weighted average price
    #[serde(rename = "w")]
    pub weighted_avg_price: String,

    /// Previous close price
    #[serde(rename = "x")]
    pub prev_close_price: String,

    /// Current close price
    #[serde(rename = "c")]
    pub last_price: String,

    /// Close quantity
    #[serde(rename = "Q")]
    pub last_quantity: String,

    /// Best bid price
    #[serde(rename = "b")]
    pub bid_price: String,

    /// Best bid quantity
    #[serde(rename = "B")]
    pub bid_quantity: String,

    /// Best ask price
    #[serde(rename = "a")]
    pub ask_price: String,

    /// Best ask quantity
    #[serde(rename = "A")]
    pub ask_quantity: String,

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

    /// Statistics open time
    #[serde(rename = "O")]
    pub open_time: u64,

    /// Statistics close time
    #[serde(rename = "C")]
    pub close_time: u64,

    /// First trade ID
    #[serde(rename = "F")]
    pub first_trade_id: u64,

    /// Last trade ID
    #[serde(rename = "L")]
    pub last_trade_id: u64,

    /// Trade count
    #[serde(rename = "n")]
    pub count: u64,
}

impl BinanceSpotWebSocketClient {
    /// Subscribe to individual symbol ticker stream
    /// 
    /// 24hr rolling window ticker statistics for a single symbol.
    /// These are NOT the statistics of the UTC day, but a 24hr rolling window
    /// for the previous 24hrs.
    /// 
    /// # Stream Name
    /// `<symbol>@ticker`
    /// 
    /// # Update Speed
    /// 1000ms
    /// 
    /// # API Documentation
    /// https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#individual-symbol-ticker-streams
    pub async fn subscribe_ticker(&mut self, symbol: &str) -> Result<(), WebSocketError> {
        let stream = Self::build_stream_name(symbol, StreamType::Ticker, None);
        self.subscribe(&[stream]).await
    }
    
    /// Unsubscribe from individual symbol ticker stream
    pub async fn unsubscribe_ticker(&mut self, symbol: &str) -> Result<(), WebSocketError> {
        let stream = Self::build_stream_name(symbol, StreamType::Ticker, None);
        self.unsubscribe(&[stream]).await
    }
}