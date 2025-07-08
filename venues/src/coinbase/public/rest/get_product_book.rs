//! Get product book endpoint for Coinbase Exchange REST API
//!
//! Get a list of open orders for a product.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::coinbase::RestResult;

/// Endpoint URL path for getting product book
const ENDPOINT_PATH: &str = "products/{}/book";

/// Request to get product order book
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetProductBookRequest {
    /// Level of detail for the order book
    /// 1: Only the best bid and ask
    /// 2: Full aggregated order book
    /// 3: Full non-aggregated order book
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u32>,
}

/// Order book level (bid or ask)
///
/// For level 1 & 2: [price, size, num-orders]
/// For level 3: [price, size, order-id]
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum OrderBookLevel {
    /// Level 1 & 2: Aggregated level with price, size, and number of orders
    Aggregated(String, String, u32),
    /// Level 3: Non-aggregated level with price, size, and order ID
    NonAggregated(String, String, String),
}

/// Auction information
#[derive(Debug, Clone, Deserialize)]
pub struct AuctionInfo {
    /// Opening price during auction
    #[serde(default)]
    pub open_price: String,

    /// Opening size during auction
    #[serde(default)]
    pub open_size: String,

    /// Best bid price during auction
    #[serde(default)]
    pub best_bid_price: String,

    /// Best bid size during auction
    #[serde(default)]
    pub best_bid_size: String,

    /// Best ask price during auction
    #[serde(default)]
    pub best_ask_price: String,

    /// Best ask size during auction
    #[serde(default)]
    pub best_ask_size: String,

    /// Current auction state
    #[serde(default)]
    pub auction_state: String,

    /// Whether the auction can open
    #[serde(default)]
    pub can_open: String,

    /// Auction timestamp
    pub time: DateTime<Utc>,
}

/// Product order book
#[derive(Debug, Clone, Deserialize)]
pub struct GetProductBookResponse {
    /// Bid orders (buy orders)
    pub bids: Vec<OrderBookLevel>,

    /// Ask orders (sell orders)
    pub asks: Vec<OrderBookLevel>,

    /// Sequence number for this update
    pub sequence: f64,

    /// Whether the book is in auction mode
    #[serde(default)]
    pub auction_mode: bool,

    /// Auction information (when in auction mode)
    pub auction: Option<AuctionInfo>,

    /// Timestamp of the order book
    pub time: DateTime<Utc>,
}

impl RestClient {
    /// Get product book
    ///
    /// Get a list of open orders for a product. The amount of detail shown can be
    /// customized with the level parameter.
    ///
    /// # Arguments
    /// * `product_id` - The product ID (e.g., "BTC-USD")
    /// * `request` - The product book request parameters
    ///
    /// # Returns
    /// A result containing the product order book or an error
    pub async fn get_product_book(
        &self,
        product_id: &str,
        request: &GetProductBookRequest,
    ) -> RestResult<GetProductBookResponse> {
        let endpoint = ENDPOINT_PATH.replace("{}", product_id);
        self.send_request(&endpoint, reqwest::Method::GET, Some(request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_product_book_request_serialization() {
        let request = GetProductBookRequest { level: Some(2) };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("level=2"));
    }

    #[test]
    fn test_get_product_book_request_default() {
        let request = GetProductBookRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.is_empty());
    }

    #[test]
    fn test_order_book_level_aggregated_deserialization() {
        let json = r#"["50000.00", "1.5", 3]"#;
        let level: OrderBookLevel = serde_json::from_str(json).unwrap();

        match level {
            OrderBookLevel::Aggregated(price, size, num_orders) => {
                assert_eq!(price, "50000.00");
                assert_eq!(size, "1.5");
                assert_eq!(num_orders, 3);
            }
            _ => panic!("Expected aggregated level"),
        }
    }

    #[test]
    fn test_order_book_level_non_aggregated_deserialization() {
        let json = r#"["50000.00", "1.5", "order-123"]"#;
        let level: OrderBookLevel = serde_json::from_str(json).unwrap();

        match level {
            OrderBookLevel::NonAggregated(price, size, order_id) => {
                assert_eq!(price, "50000.00");
                assert_eq!(size, "1.5");
                assert_eq!(order_id, "order-123");
            }
            _ => panic!("Expected non-aggregated level"),
        }
    }

    #[test]
    fn test_get_product_book_response_deserialization() {
        let json = r#"{
            "bids": [["50000.00", "1.5", 3]],
            "asks": [["50001.00", "2.0", 2]],
            "sequence": 12345.0,
            "auction_mode": false,
            "auction": null,
            "time": "2021-01-01T00:00:00.000Z"
        }"#;

        let order_book: GetProductBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.bids.len(), 1);
        assert_eq!(order_book.asks.len(), 1);
        assert_eq!(order_book.sequence, 12345.0);
        assert!(!order_book.auction_mode);
    }
}
