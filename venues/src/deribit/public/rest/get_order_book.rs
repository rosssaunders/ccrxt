//! Implements the /public/get_order_book endpoint for Deribit.
//!
//! Retrieves the current order book for a given instrument.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

const ORDER_BOOK_ENDPOINT: &str = "public/get_order_book";

/// Request parameters for the get_order_book endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOrderBookRequest {
    /// Instrument name for which to retrieve the order book.
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Depth of the order book to return (optional, default: 20, max: 200).
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
}

/// The result object for get_order_book.
#[derive(Debug, Clone, Deserialize)]
pub struct GetOrderBookResult {
    /// List of bid entries.
    #[serde(rename = "bids")]
    pub bids: Vec<(f64, f64)>,

    /// List of ask entries.
    #[serde(rename = "asks")]
    pub asks: Vec<(f64, f64)>,

    /// The best bid price.
    #[serde(rename = "best_bid_price")]
    pub best_bid_price: f64,

    /// The best ask price.
    #[serde(rename = "best_ask_price")]
    pub best_ask_price: f64,

    /// The timestamp in milliseconds since epoch.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Response for the get_order_book endpoint.
pub type GetOrderBookResponse = JsonRpcResult<GetOrderBookResult>;

impl RestClient {
    /// Calls the /public/get_order_book endpoint.
    ///
    /// Retrieves the current order book for a given instrument.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_order_book)
    pub async fn get_order_book(
        &self,
        params: GetOrderBookRequest,
    ) -> RestResult<GetOrderBookResponse> {
        self.send_request(
            ORDER_BOOK_ENDPOINT,
            Some(&params),
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_serialize_request() {
        let req = GetOrderBookRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            depth: Some(10),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
        assert!(json.contains("depth"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 23,
            "jsonrpc": "2.0",
            "result": {
                "bids": [
                    [64999.0, 0.5]
                ],
                "asks": [
                    [65001.0, 0.4]
                ],
                "best_bid_price": 64999.0,
                "best_ask_price": 65001.0,
                "timestamp": 1680310800000
            }
        }"#;
        let resp: GetOrderBookResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 23);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.bids.len(), 1);
        assert_eq!(resp.result.asks.len(), 1);
        assert!((resp.result.best_bid_price - 64999.0).abs() < 1e-8);
        assert!((resp.result.best_ask_price - 65001.0).abs() < 1e-8);
        assert_eq!(resp.result.timestamp, 1680310800000);
    }
}
