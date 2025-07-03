//! BitMart WebSocket depth/order book stream
//!
//! This module implements the depth WebSocket stream for BitMart order book data.

use serde::{Deserialize, Serialize};

use super::client::{PublicChannel, WsClient, WsMessage};

/// Order book entry from WebSocket stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthEntry {
    /// Price level
    pub price: String,
    /// Quantity at this price level
    pub size: String,
}

/// Depth/order book data from WebSocket stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthData {
    /// Trading pair symbol
    pub symbol: String,
    /// Buy orders (bids)
    pub buys: Vec<DepthEntry>,
    /// Sell orders (asks)
    pub sells: Vec<DepthEntry>,
    /// Unix timestamp in milliseconds
    #[serde(rename = "ms_t")]
    pub timestamp: u64,
}

/// Depth levels available for subscription
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthLevel {
    /// Top 5 levels
    Level5,
    /// Top 20 levels  
    Level20,
    /// Top 50 levels
    Level50,
}

impl DepthLevel {
    /// Convert depth level to string format for topic construction
    pub fn as_str(&self) -> &'static str {
        match self {
            DepthLevel::Level5 => "5",
            DepthLevel::Level20 => "20",
            DepthLevel::Level50 => "50",
        }
    }
}

impl WsClient {
    /// Subscribe to depth data for a specific symbol
    ///
    /// # Arguments
    /// * `symbol` - Trading pair symbol (e.g., "BTC_USDT")
    /// * `level` - Depth level (5, 20, or 50)
    ///
    /// # Returns
    /// WebSocket message to send for subscription
    pub fn subscribe_depth(&self, symbol: &str, level: DepthLevel) -> WsMessage {
        let topic = PublicChannel::Depth.topic(symbol, Some(level.as_str()));
        WsMessage::subscribe(vec![topic])
    }

    /// Subscribe to depth data for multiple symbols
    ///
    /// # Arguments
    /// * `symbols` - Vector of (symbol, level) tuples
    ///
    /// # Returns
    /// WebSocket message to send for subscription
    pub fn subscribe_depths(&self, symbols: Vec<(&str, DepthLevel)>) -> WsMessage {
        let topics: Vec<String> = symbols
            .iter()
            .map(|(symbol, level)| PublicChannel::Depth.topic(symbol, Some(level.as_str())))
            .collect();
        WsMessage::subscribe(topics)
    }

    /// Unsubscribe from depth data for a specific symbol
    ///
    /// # Arguments
    /// * `symbol` - Trading pair symbol (e.g., "BTC_USDT")
    /// * `level` - Depth level (5, 20, or 50)
    ///
    /// # Returns
    /// WebSocket message to send for unsubscription
    pub fn unsubscribe_depth(&self, symbol: &str, level: DepthLevel) -> WsMessage {
        let topic = PublicChannel::Depth.topic(symbol, Some(level.as_str()));
        WsMessage::unsubscribe(vec![topic])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_subscription() {
        let client = WsClient::new();
        let msg = client.subscribe_depth("BTC_USDT", DepthLevel::Level5);
        
        assert_eq!(msg.args.len(), 1);
        assert_eq!(msg.args[0], "spot/depth5:BTC_USDT");
    }

    #[test]
    fn test_multiple_depth_subscription() {
        let client = WsClient::new();
        let symbols = vec![("BTC_USDT", DepthLevel::Level5), ("ETH_USDT", DepthLevel::Level20)];
        let msg = client.subscribe_depths(symbols);
        
        assert_eq!(msg.args.len(), 2);
        assert!(msg.args.contains(&"spot/depth5:BTC_USDT".to_string()));
        assert!(msg.args.contains(&"spot/depth20:ETH_USDT".to_string()));
    }

    #[test]
    fn test_depth_levels() {
        assert_eq!(DepthLevel::Level5.as_str(), "5");
        assert_eq!(DepthLevel::Level20.as_str(), "20");
        assert_eq!(DepthLevel::Level50.as_str(), "50");
    }

    #[test]
    fn test_depth_data_deserialization() {
        let json = r#"
        {
            "symbol": "BTC_USDT",
            "buys": [
                {"price": "50000.00", "size": "0.1"},
                {"price": "49999.00", "size": "0.2"}
            ],
            "sells": [
                {"price": "50001.00", "size": "0.15"},
                {"price": "50002.00", "size": "0.25"}
            ],
            "ms_t": 1640995200000
        }
        "#;

        let depth: DepthData = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(depth.symbol, "BTC_USDT");
        assert_eq!(depth.buys.len(), 2);
        assert_eq!(depth.sells.len(), 2);
        assert_eq!(depth.buys[0].price, "50000.00");
        assert_eq!(depth.sells[0].price, "50001.00");
        assert_eq!(depth.timestamp, 1640995200000);
    }
}
