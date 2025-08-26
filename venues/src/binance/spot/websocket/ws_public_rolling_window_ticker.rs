use serde::{Deserialize, Serialize};
use websockets::WebSocketError;

use super::client::BinanceSpotWebSocketClient;

/// Rolling window ticker data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollingWindowTickerData {
    /// Event type (always "1hTicker", "4hTicker", or "1dTicker")
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

    /// Open price
    #[serde(rename = "o")]
    pub open_price: String,

    /// High price
    #[serde(rename = "h")]
    pub high_price: String,

    /// Low price
    #[serde(rename = "l")]
    pub low_price: String,

    /// Last price
    #[serde(rename = "c")]
    pub last_price: String,

    /// Weighted average price
    #[serde(rename = "w")]
    pub weighted_avg_price: String,

    /// Total traded base asset volume
    #[serde(rename = "v")]
    pub volume: String,

    /// Total traded quote asset volume
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
    pub first_trade_id: i64,

    /// Last trade ID
    #[serde(rename = "L")]
    pub last_trade_id: i64,

    /// Total number of trades
    #[serde(rename = "n")]
    pub count: u64,
}

/// Rolling window size for statistics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollingWindowSize {
    /// 1 hour window
    OneHour,
    /// 4 hour window
    FourHour,
    /// 1 day window
    OneDay,
}

impl RollingWindowSize {
    /// Convert to stream suffix
    pub fn to_stream_suffix(&self) -> &'static str {
        match self {
            Self::OneHour => "1h",
            Self::FourHour => "4h",
            Self::OneDay => "1d",
        }
    }
}

impl BinanceSpotWebSocketClient {
    /// Subscribe to rolling window ticker stream
    /// 
    /// Rolling window ticker statistics for a single symbol, computed over multiple windows.
    /// 
    /// # Stream Name
    /// `<symbol>@ticker_<window_size>` where window_size is 1h, 4h, or 1d
    /// 
    /// # Update Speed
    /// 1000ms
    /// 
    /// # API Documentation
    /// https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#rolling-window-statistics-streams
    pub async fn subscribe_rolling_window_ticker(
        &mut self, 
        symbol: &str, 
        window: RollingWindowSize
    ) -> Result<(), WebSocketError> {
        let stream = format!(
            "{}@ticker_{}", 
            symbol.to_lowercase(), 
            window.to_stream_suffix()
        );
        self.subscribe(&[stream]).await
    }
    
    /// Unsubscribe from rolling window ticker stream
    pub async fn unsubscribe_rolling_window_ticker(
        &mut self, 
        symbol: &str, 
        window: RollingWindowSize
    ) -> Result<(), WebSocketError> {
        let stream = format!(
            "{}@ticker_{}", 
            symbol.to_lowercase(), 
            window.to_stream_suffix()
        );
        self.unsubscribe(&[stream]).await
    }
    
    /// Subscribe to all market rolling window ticker stream
    /// 
    /// # Stream Name
    /// `!ticker_<window_size>@arr`
    /// 
    /// # Update Speed
    /// 1000ms
    pub async fn subscribe_all_rolling_window_tickers(
        &mut self, 
        window: RollingWindowSize
    ) -> Result<(), WebSocketError> {
        let stream = format!("!ticker_{}@arr", window.to_stream_suffix());
        self.subscribe(&[stream]).await
    }
    
    /// Unsubscribe from all market rolling window ticker stream
    pub async fn unsubscribe_all_rolling_window_tickers(
        &mut self, 
        window: RollingWindowSize
    ) -> Result<(), WebSocketError> {
        let stream = format!("!ticker_{}@arr", window.to_stream_suffix());
        self.unsubscribe(&[stream]).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_window_ticker_deserialization() {
        let json = r#"{
            "e": "1hTicker",
            "E": 1672515782136,
            "s": "BTCUSDT",
            "p": "500.00000000",
            "P": "2.00",
            "o": "25000.00000000",
            "h": "25600.00000000",
            "l": "24900.00000000",
            "c": "25500.00000000",
            "w": "25250.00000000",
            "v": "89500.00000000",
            "q": "2259625000.00000000",
            "O": 1672512182136,
            "C": 1672515782136,
            "F": 0,
            "L": 18150,
            "n": 18151
        }"#;
        
        let data: RollingWindowTickerData = serde_json::from_str(json).unwrap();
        assert_eq!(data.event_type, "1hTicker");
        assert_eq!(data.symbol, "BTCUSDT");
        assert_eq!(data.price_change, "500.00000000");
        assert_eq!(data.price_change_percent, "2.00");
    }
    
    #[test]
    fn test_rolling_window_size() {
        assert_eq!(RollingWindowSize::OneHour.to_stream_suffix(), "1h");
        assert_eq!(RollingWindowSize::FourHour.to_stream_suffix(), "4h");
        assert_eq!(RollingWindowSize::OneDay.to_stream_suffix(), "1d");
    }
}