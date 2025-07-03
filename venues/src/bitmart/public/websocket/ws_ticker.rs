//! BitMart WebSocket ticker stream
//!
//! This module implements the ticker WebSocket stream for BitMart.

use serde::{Deserialize, Serialize};

use super::client::{PublicChannel, WsClient, WsMessage};

/// Ticker data from WebSocket stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerData {
    /// Trading pair symbol
    pub symbol: String,
    /// Last trade price
    #[serde(rename = "lastPrice")]
    pub last_price: String,
    /// Volume in base currency over 24h
    #[serde(rename = "baseVolume")]
    pub base_volume: String,
    /// Volume in quote currency over 24h
    #[serde(rename = "quoteVolume")]
    pub quote_volume: String,
    /// Highest price in 24h
    #[serde(rename = "high24h")]
    pub high_24h: String,
    /// Lowest price in 24h
    #[serde(rename = "low24h")]
    pub low_24h: String,
    /// Price change percentage in 24h
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: String,
    /// Unix timestamp in milliseconds
    #[serde(rename = "s_t")]
    pub timestamp: u64,
}

impl WsClient {
    /// Subscribe to ticker data for a specific symbol
    ///
    /// # Arguments
    /// * `symbol` - Trading pair symbol (e.g., "BTC_USDT")
    ///
    /// # Returns
    /// WebSocket message to send for subscription
    pub fn subscribe_ticker(&self, symbol: &str) -> WsMessage {
        let topic = PublicChannel::Ticker.topic(symbol, None);
        WsMessage::subscribe(vec![topic])
    }

    /// Subscribe to ticker data for multiple symbols
    ///
    /// # Arguments
    /// * `symbols` - Vector of trading pair symbols
    ///
    /// # Returns
    /// WebSocket message to send for subscription
    pub fn subscribe_tickers(&self, symbols: Vec<&str>) -> WsMessage {
        let topics: Vec<String> = symbols
            .iter()
            .map(|symbol| PublicChannel::Ticker.topic(symbol, None))
            .collect();
        WsMessage::subscribe(topics)
    }

    /// Unsubscribe from ticker data for a specific symbol
    ///
    /// # Arguments
    /// * `symbol` - Trading pair symbol (e.g., "BTC_USDT")
    ///
    /// # Returns
    /// WebSocket message to send for unsubscription
    pub fn unsubscribe_ticker(&self, symbol: &str) -> WsMessage {
        let topic = PublicChannel::Ticker.topic(symbol, None);
        WsMessage::unsubscribe(vec![topic])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_subscription() {
        let client = WsClient::new();
        let msg = client.subscribe_ticker("BTC_USDT");
        
        assert_eq!(msg.args.len(), 1);
        assert_eq!(msg.args[0], "spot/ticker:BTC_USDT");
    }

    #[test]
    fn test_multiple_ticker_subscription() {
        let client = WsClient::new();
        let symbols = vec!["BTC_USDT", "ETH_USDT"];
        let msg = client.subscribe_tickers(symbols);
        
        assert_eq!(msg.args.len(), 2);
        assert!(msg.args.contains(&"spot/ticker:BTC_USDT".to_string()));
        assert!(msg.args.contains(&"spot/ticker:ETH_USDT".to_string()));
    }

    #[test]
    fn test_ticker_data_deserialization() {
        let json = r#"
        {
            "symbol": "BTC_USDT",
            "lastPrice": "50000.00",
            "baseVolume": "100.5",
            "quoteVolume": "5025000.00",
            "high24h": "51000.00",
            "low24h": "49000.00",
            "priceChangePercent": "2.04",
            "s_t": 1640995200000
        }
        "#;

        let ticker: TickerData = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(ticker.symbol, "BTC_USDT");
        assert_eq!(ticker.last_price, "50000.00");
        assert_eq!(ticker.timestamp, 1640995200000);
    }
}
