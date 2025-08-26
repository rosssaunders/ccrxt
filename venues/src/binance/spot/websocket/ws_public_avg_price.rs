use serde::{Deserialize, Serialize};
use websockets::WebSocketError;

use super::client::BinanceSpotWebSocketClient;

/// Average price data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvgPriceData {
    /// Event type (always "avgPrice")
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event time
    #[serde(rename = "E")]
    pub event_time: u64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Average price interval
    #[serde(rename = "i")]
    pub interval: String,

    /// Average price
    #[serde(rename = "w")]
    pub avg_price: String,

    /// Last trade time
    #[serde(rename = "T")]
    pub last_trade_time: u64,
}

impl BinanceSpotWebSocketClient {
    /// Subscribe to average price stream
    /// 
    /// Average price streams push changes in the average price over a fixed time interval.
    /// 
    /// # Stream Name
    /// `<symbol>@avgPrice`
    /// 
    /// # Update Speed
    /// 1000ms
    /// 
    /// # API Documentation
    /// https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#average-price-streams
    pub async fn subscribe_avg_price(&mut self, symbol: &str) -> Result<(), WebSocketError> {
        let stream = format!("{}@avgPrice", symbol.to_lowercase());
        self.subscribe(&[stream]).await
    }
    
    /// Unsubscribe from average price stream
    pub async fn unsubscribe_avg_price(&mut self, symbol: &str) -> Result<(), WebSocketError> {
        let stream = format!("{}@avgPrice", symbol.to_lowercase());
        self.unsubscribe(&[stream]).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avg_price_data_deserialization() {
        let json = r#"{
            "e": "avgPrice",
            "E": 1693907033000,
            "s": "BTCUSDT",
            "i": "5m",
            "w": "25776.86000000",
            "T": 1693907032213
        }"#;
        
        let data: AvgPriceData = serde_json::from_str(json).unwrap();
        assert_eq!(data.event_type, "avgPrice");
        assert_eq!(data.symbol, "BTCUSDT");
        assert_eq!(data.interval, "5m");
        assert_eq!(data.avg_price, "25776.86000000");
    }
}