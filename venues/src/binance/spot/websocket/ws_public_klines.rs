use serde::{Deserialize, Serialize};
use websockets::WebSocketError;

use super::client::BinanceSpotWebSocketClient;
use super::enums::KlineInterval;

/// Kline/candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KlineData {
    /// Event type (always "kline")
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event time
    #[serde(rename = "E")]
    pub event_time: u64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Kline data
    #[serde(rename = "k")]
    pub kline: Kline,
}

/// Kline details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kline {
    /// Kline start time
    #[serde(rename = "t")]
    pub start_time: u64,

    /// Kline close time
    #[serde(rename = "T")]
    pub close_time: u64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Interval
    #[serde(rename = "i")]
    pub interval: String,

    /// First trade ID
    #[serde(rename = "f")]
    pub first_trade_id: u64,

    /// Last trade ID
    #[serde(rename = "L")]
    pub last_trade_id: u64,

    /// Open price
    #[serde(rename = "o")]
    pub open: String,

    /// Close price
    #[serde(rename = "c")]
    pub close: String,

    /// High price
    #[serde(rename = "h")]
    pub high: String,

    /// Low price
    #[serde(rename = "l")]
    pub low: String,

    /// Base asset volume
    #[serde(rename = "v")]
    pub volume: String,

    /// Number of trades
    #[serde(rename = "n")]
    pub trade_count: u64,

    /// Is this kline closed?
    #[serde(rename = "x")]
    pub is_closed: bool,

    /// Quote asset volume
    #[serde(rename = "q")]
    pub quote_volume: String,

    /// Taker buy base asset volume
    #[serde(rename = "V")]
    pub taker_buy_volume: String,

    /// Taker buy quote asset volume
    #[serde(rename = "Q")]
    pub taker_buy_quote_volume: String,

    /// Ignore
    #[serde(rename = "B")]
    pub ignore: String,
}

impl BinanceSpotWebSocketClient {
    /// Subscribe to kline/candlestick stream
    /// 
    /// The Kline/Candlestick Stream push updates to the current klines/candlestick every second.
    /// 
    /// # Stream Name
    /// `<symbol>@kline_<interval>`
    /// 
    /// # Update Speed
    /// 1000ms
    /// 
    /// # API Documentation
    /// https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#klinecandlestick-streams-for-utc
    pub async fn subscribe_klines(
        &mut self,
        symbol: &str,
        interval: KlineInterval,
    ) -> Result<(), WebSocketError> {
        let stream = format!("{}@kline_{}", symbol.to_lowercase(), interval.as_str());
        self.subscribe(&[stream]).await
    }
    
    /// Unsubscribe from kline/candlestick stream
    pub async fn unsubscribe_klines(
        &mut self,
        symbol: &str,
        interval: KlineInterval,
    ) -> Result<(), WebSocketError> {
        let stream = format!("{}@kline_{}", symbol.to_lowercase(), interval.as_str());
        self.unsubscribe(&[stream]).await
    }
}