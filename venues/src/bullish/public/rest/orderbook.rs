//! Orderbook endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use crate::bullish::{EndpointType, PublicRestClient, RestResult};

/// Endpoint URL path for orderbook
const ORDERBOOK_ENDPOINT: &str = "/trading-api/v1/markets/{}/orderbook/hybrid";

/// Orderbook entry (bid or ask)
#[derive(Debug, Clone, Deserialize)]
pub struct OrderbookEntry {
    /// Price level
    pub price: String,

    /// Quantity available at this price level
    #[serde(rename = "priceLevelQuantity")]
    pub quantity: String,

    /// Entry type (bid/ask)
    #[serde(rename = "type")]
    pub entry_type: String,
}

/// Hybrid orderbook response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HybridOrderbook {
    /// Market symbol
    pub symbol: String,

    /// Bid levels (buy orders)
    pub bids: Vec<OrderbookEntry>,

    /// Ask levels (sell orders)
    pub asks: Vec<OrderbookEntry>,
}

/// Request parameters for orderbook query
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookRequest {
    /// Number of levels to return (default: 100, max: 1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,

    /// Whether to aggregate by price level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<bool>,
}

impl PublicRestClient {
    /// Get hybrid orderbook for a market
    ///
    /// Retrieve the current orderbook state for a specific market.
    /// The hybrid orderbook combines both limit orders and AMM liquidity.
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/markets/-symbol-/orderbook/hybrid)
    ///
    /// # Arguments
    /// * `symbol` - Market symbol
    /// * `request` - Optional parameters for depth and aggregation
    ///
    /// # Returns
    /// Current orderbook state with bids and asks
    pub async fn get_orderbook(
        &self,
        symbol: &str,
        request: Option<OrderbookRequest>,
    ) -> RestResult<HybridOrderbook> {
        let mut url = ORDERBOOK_ENDPOINT.replace("{}", symbol);

        if let Some(request) = request {
            let query_string = serde_urlencoded::to_string(&request).map_err(|e| {
                crate::bullish::Errors::Error(format!(
                    "Failed to serialize query parameters: {}",
                    e
                ))
            })?;

            if !query_string.is_empty() {
                url.push('?');
                url.push_str(&query_string);
            }
        }

        self.send_get_request(&url, EndpointType::PublicOrderbook)
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
            "priceLevelQuantity": "1.5",
            "type": "bid"
        }"#;

        let entry: OrderbookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.price, "30000.0");
        assert_eq!(entry.quantity, "1.5");
        assert_eq!(entry.entry_type, "bid");
    }

    #[test]
    fn test_hybrid_orderbook_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDC",
            "bids": [
                {
                    "price": "29900.0",
                    "priceLevelQuantity": "1.0",
                    "type": "bid"
                }
            ],
            "asks": [
                {
                    "price": "30100.0",
                    "priceLevelQuantity": "2.0",
                    "type": "ask"
                }
            ]
        }"#;

        let orderbook: HybridOrderbook = serde_json::from_str(json).unwrap();
        assert_eq!(orderbook.symbol, "BTCUSDC");
        assert_eq!(orderbook.bids.len(), 1);
        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids[0].price, "29900.0");
        assert_eq!(orderbook.asks[0].price, "30100.0");
    }

    #[test]
    fn test_orderbook_request_default() {
        let request = OrderbookRequest::default();
        assert!(request.depth.is_none());
        assert!(request.aggregate.is_none());
    }
}
