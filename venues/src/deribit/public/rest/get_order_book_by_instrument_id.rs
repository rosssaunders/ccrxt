//! Implements the /public/get_order_book_by_instrument_id endpoint for Deribit.
//!
//! Retrieves the current order book for a given instrument by its ID.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, RestResult};

const ORDER_BOOK_BY_INSTRUMENT_ID_ENDPOINT: &str = "public/get_order_book_by_instrument_id";

/// Request parameters for the get_order_book_by_instrument_id endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOrderBookByInstrumentIdRequest {
    /// Instrument ID for which to retrieve the order book.
    #[serde(rename = "instrument_id")]
    pub instrument_id: String,

    /// Depth of the order book to return (optional, default: 20, max: 200).
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
}

/// Represents a single order book entry (bid or ask).
#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookEntry {
    /// Price level.
    #[serde(rename = "price")]
    pub price: f64,

    /// Amount available at this price level.
    #[serde(rename = "amount")]
    pub amount: f64,
}

/// The result object for get_order_book_by_instrument_id.
#[derive(Debug, Clone, Deserialize)]
pub struct GetOrderBookByInstrumentIdResult {
    /// List of bid entries.
    #[serde(rename = "bids")]
    pub bids: Vec<OrderBookEntry>,

    /// List of ask entries.
    #[serde(rename = "asks")]
    pub asks: Vec<OrderBookEntry>,

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

/// Response for the get_order_book_by_instrument_id endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetOrderBookByInstrumentIdResponse {
    /// The id that was sent in the request.
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0).
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result object containing the order book.
    #[serde(rename = "result")]
    pub result: GetOrderBookByInstrumentIdResult,
}

impl RestClient {
    /// Calls the /public/get_order_book_by_instrument_id endpoint.
    ///
    /// Retrieves the current order book for a given instrument by its ID.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_order_book_by_instrument_id)
    pub async fn get_order_book_by_instrument_id(
        &self,
        params: GetOrderBookByInstrumentIdRequest,
    ) -> RestResult<GetOrderBookByInstrumentIdResponse> {
        self.send_request(
            ORDER_BOOK_BY_INSTRUMENT_ID_ENDPOINT,
            Method::POST,
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
        let req = GetOrderBookByInstrumentIdRequest {
            instrument_id: "BTC-PERPETUAL-ID".to_string(),
            depth: Some(10),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL-ID"));
        assert!(json.contains("depth"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 24,
            "jsonrpc": "2.0",
            "result": {
                "bids": [
                    {"price": 64998.0, "amount": 0.6}
                ],
                "asks": [
                    {"price": 65002.0, "amount": 0.3}
                ],
                "best_bid_price": 64998.0,
                "best_ask_price": 65002.0,
                "timestamp": 1680310800000
            }
        }"#;
        let resp: GetOrderBookByInstrumentIdResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 24);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.bids.len(), 1);
        assert_eq!(resp.result.asks.len(), 1);
        assert!((resp.result.best_bid_price - 64998.0).abs() < 1e-8);
        assert!((resp.result.best_ask_price - 65002.0).abs() < 1e-8);
        assert_eq!(resp.result.timestamp, 1680310800000);
    }
}
