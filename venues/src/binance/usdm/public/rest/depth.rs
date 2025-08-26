use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::binance::usdm::PublicRestClient as RestClient;
use crate::binance::usdm::RestResult;

/// Endpoint constant for the order book (depth) REST API call.
const ORDER_BOOK_ENDPOINT: &str = "/fapi/v1/depth";

/// Request parameters for the order book (depth) endpoint.
///
/// Represents the query parameters for fetching the order book of a given symbol.
#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookRequest {
    /// The trading pair symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,

    /// Optional limit for the number of order book levels (default 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Response for the order book (depth) endpoint.
///
/// Contains the current state of the order book, including bids and asks at various price levels.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    /// Last update ID for the order book.
    pub last_update_id: u64,

    /// Event time (optional).
    #[serde(rename = "E", default)]
    pub event_time: Option<u64>,

    /// Transaction time (optional).
    #[serde(rename = "T", default)]
    pub transaction_time: Option<u64>,

    /// Bids as a vector of (price, quantity) tuples.
    pub bids: Vec<(Cow<'static, str>, Cow<'static, str>)>,

    /// Asks as a vector of (price, quantity) tuples.
    pub asks: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl RestClient {
    /// Order Book
    ///
    /// Query symbol orderbook.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Order-Book)
    ///
    /// Rate limit: Adjusted based on the `limit` parameter:
    /// * 5, 10, 20, 50 => 2
    /// * 100 => 5
    /// * 500 => 10
    /// * 1000 => 20
    ///
    /// # Arguments
    /// * `params` - The order book request parameters.
    ///
    /// # Returns
    /// * `OrderBookResponse` - The order book response.
    pub async fn get_order_book(&self, params: OrderBookRequest) -> RestResult<OrderBookResponse> {
        // Compute request weight based on 'limit' parameter per API rate limits
        let limit = params.limit.unwrap_or(500);
        let weight = match limit {
            5 | 10 | 20 | 50 => 2,
            100 => 5,
            500 => 10,
            1000 => 20,
            _ => 2,
        };
        self.send_get_request(ORDER_BOOK_ENDPOINT, Some(params), weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_book_request_serialization() {
        let request = OrderBookRequest {
            symbol: "BTCUSDT".into(),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_order_book_request_minimal() {
        let request = OrderBookRequest {
            symbol: "ETHUSDT".into(),
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSDT");
    }

    #[test]
    fn test_order_book_response_deserialization() {
        let json = r#"{
            "lastUpdateId": 1234567890,
            "E": 1625184000000,
            "T": 1625184000001,
            "bids": [
                ["45380.00", "2.500"],
                ["45379.90", "5.000"],
                ["45379.80", "10.000"]
            ],
            "asks": [
                ["45380.10", "1.200"],
                ["45380.20", "3.500"],
                ["45380.30", "8.000"]
            ]
        }"#;

        let order_book: OrderBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.last_update_id, 1234567890);
        assert_eq!(order_book.event_time, Some(1625184000000));
        assert_eq!(order_book.transaction_time, Some(1625184000001));

        assert_eq!(order_book.bids.len(), 3);
        assert_eq!(order_book.bids[0].0, "45380.00");
        assert_eq!(order_book.bids[0].1, "2.500");
        assert_eq!(order_book.bids[1].0, "45379.90");
        assert_eq!(order_book.bids[1].1, "5.000");

        assert_eq!(order_book.asks.len(), 3);
        assert_eq!(order_book.asks[0].0, "45380.10");
        assert_eq!(order_book.asks[0].1, "1.200");
        assert_eq!(order_book.asks[1].0, "45380.20");
        assert_eq!(order_book.asks[1].1, "3.500");
    }

    #[test]
    fn test_order_book_response_without_optional_fields() {
        let json = r#"{
            "lastUpdateId": 9876543210,
            "bids": [
                ["3070.00", "10.000"]
            ],
            "asks": [
                ["3070.10", "8.500"]
            ]
        }"#;

        let order_book: OrderBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.last_update_id, 9876543210);
        assert_eq!(order_book.event_time, None);
        assert_eq!(order_book.transaction_time, None);
        assert_eq!(order_book.bids.len(), 1);
        assert_eq!(order_book.asks.len(), 1);
    }

    #[test]
    fn test_order_book_empty_book() {
        let json = r#"{
            "lastUpdateId": 1111111111,
            "bids": [],
            "asks": []
        }"#;

        let order_book: OrderBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.last_update_id, 1111111111);
        assert_eq!(order_book.bids.len(), 0);
        assert_eq!(order_book.asks.len(), 0);
    }

    #[test]
    fn test_order_book_deep_book() {
        let json = r#"{
            "lastUpdateId": 2222222222,
            "bids": [
                ["45380.00", "0.100"],
                ["45379.99", "0.200"],
                ["45379.98", "0.300"],
                ["45379.97", "0.400"],
                ["45379.96", "0.500"]
            ],
            "asks": [
                ["45380.01", "0.150"],
                ["45380.02", "0.250"],
                ["45380.03", "0.350"],
                ["45380.04", "0.450"],
                ["45380.05", "0.550"]
            ]
        }"#;

        let order_book: OrderBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.bids.len(), 5);
        assert_eq!(order_book.asks.len(), 5);

        // Check price ordering
        assert_eq!(order_book.bids[0].0, "45380.00"); // Highest bid
        assert_eq!(order_book.bids[4].0, "45379.96"); // Lowest bid
        assert_eq!(order_book.asks[0].0, "45380.01"); // Lowest ask
        assert_eq!(order_book.asks[4].0, "45380.05"); // Highest ask
    }
}
