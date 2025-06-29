//! Orderbook endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

/// Orderbook entry (bid or ask)
#[derive(Debug, Clone, Deserialize)]
pub struct OrderbookEntry {
    /// Price level
    pub price: String,
    /// Quantity available at this price level
    pub quantity: String,
    /// Number of orders at this price level
    #[serde(rename = "orderCount")]
    pub order_count: Option<u32>,
}

/// Hybrid orderbook response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HybridOrderbook {
    /// Market symbol
    pub symbol: String,
    /// Orderbook timestamp
    pub timestamp: u64,
    /// Sequence number for ordering updates
    pub sequence: u64,
    /// Bid levels (buy orders)
    pub bids: Vec<OrderbookEntry>,
    /// Ask levels (sell orders)
    pub asks: Vec<OrderbookEntry>,
}

/// Parameters for orderbook query
#[derive(Debug, Clone, Default)]
pub struct OrderbookParams {
    /// Number of levels to return (default: 100, max: 1000)
    pub depth: Option<u32>,
    /// Whether to aggregate by price level
    pub aggregate: Option<bool>,
}

impl RestClient {
    /// Get hybrid orderbook for a market
    ///
    /// Retrieve the current orderbook state for a specific market.
    /// The hybrid orderbook combines both limit orders and AMM liquidity.
    ///
    /// # Arguments
    /// * `symbol` - Market symbol
    /// * `params` - Optional parameters for depth and aggregation
    ///
    /// # Returns
    /// Current orderbook state with bids and asks
    pub async fn get_orderbook(&self, symbol: &str, params: Option<OrderbookParams>) -> RestResult<HybridOrderbook> {
        let mut url = format!("/v1/markets/{}/orderbook/hybrid", symbol);
        
        if let Some(params) = params {
            let mut query_params = Vec::new();
            
            if let Some(depth) = params.depth {
                query_params.push(format!("depth={}", depth));
            }
            if let Some(aggregate) = params.aggregate {
                query_params.push(format!("aggregate={}", aggregate));
            }
            
            if !query_params.is_empty() {
                url.push('?');
                url.push_str(&query_params.join("&"));
            }
        }
        
        self.send_request(
            &url,
            reqwest::Method::GET,
            None::<&()>,
            EndpointType::PublicOrderbook,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orderbook_entry_deserialization() {
        let json = r#"{
            "price": "30000.0",
            "quantity": "1.5",
            "orderCount": 5
        }"#;

        let entry: OrderbookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.price, "30000.0");
        assert_eq!(entry.quantity, "1.5");
        assert_eq!(entry.order_count, Some(5));
    }

    #[test]
    fn test_hybrid_orderbook_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDC",
            "timestamp": 1640995200000,
            "sequence": 12345,
            "bids": [
                {
                    "price": "29900.0",
                    "quantity": "1.0",
                    "orderCount": 3
                }
            ],
            "asks": [
                {
                    "price": "30100.0",
                    "quantity": "2.0",
                    "orderCount": 2
                }
            ]
        }"#;

        let orderbook: HybridOrderbook = serde_json::from_str(json).unwrap();
        assert_eq!(orderbook.symbol, "BTCUSDC");
        assert_eq!(orderbook.timestamp, 1640995200000);
        assert_eq!(orderbook.sequence, 12345);
        assert_eq!(orderbook.bids.len(), 1);
        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids[0].price, "29900.0");
        assert_eq!(orderbook.asks[0].price, "30100.0");
    }

    #[test]
    fn test_orderbook_params_default() {
        let params = OrderbookParams::default();
        assert!(params.depth.is_none());
        assert!(params.aggregate.is_none());
    }
}
