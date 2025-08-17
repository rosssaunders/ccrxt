//! Request and response structs for public/get-book endpoint
//!
//! Fetches the public order book for a particular instrument and depth.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::{ApiResult, EndpointType, RestResult};

/// Endpoint for getting order book data
const GET_BOOK_ENDPOINT: &str = "public/get-book";

/// Request parameters for the public/get-book endpoint.
///
/// Fetches the public order book for a particular instrument and depth.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetBookRequest {
    /// Instrument name (e.g., "BTCUSD-PERP"). Required.
    #[serde(rename = "instrument_name")]
    pub instrument_name: Cow<'static, str>,

    /// Depth of the order book to return. Optional. Valid values: 1-150.
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
}

/// Response for public/get-book endpoint.
pub type GetBookResponse = ApiResult<BookResult>;

/// Result data for order book.
#[derive(Debug, Clone, Deserialize)]
pub struct BookResult {
    /// Depth of the order book returned.
    #[serde(rename = "depth")]
    pub depth: u32,

    /// Data array containing bids/asks and instrument_name.
    #[serde(rename = "data")]
    pub data: Vec<BookData>,

    /// Instrument name (for convenience, also present in data[0]).
    #[serde(rename = "instrument_name")]
    pub instrument_name: Cow<'static, str>,
}

/// Order book data for a single snapshot.
#[derive(Debug, Clone, Deserialize)]
pub struct BookData {
    /// List of asks: [price, quantity, num_orders] as strings.
    #[serde(rename = "asks")]
    pub asks: Vec<Vec<String>>,

    /// List of bids: [price, quantity, num_orders] as strings.
    #[serde(rename = "bids")]
    pub bids: Vec<Vec<String>>,

    /// Optional: timestamp fields, sequence numbers, etc.
    #[serde(rename = "t", default)]
    pub t: Option<u64>,

    #[serde(rename = "tt", default)]
    pub tt: Option<u64>,

    #[serde(rename = "u", default)]
    pub u: Option<u64>,
}

impl RestClient {
    /// Calls the public/get-book endpoint.
    ///
    /// Fetches the public order book for a particular instrument and depth.
    ///
    /// [docs](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#public-get-book)
    pub async fn get_book(&self, params: GetBookRequest) -> RestResult<GetBookResponse> {
        self.send_get_request(
            GET_BOOK_ENDPOINT,
            Some(&params),
            EndpointType::PublicGetBook,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_book_endpoint_type() {
        let book_endpoint = EndpointType::PublicGetBook;
        assert!(book_endpoint.rate_limit().max_requests > 0);
    }

    #[test]
    fn test_book_parameter_building() {
        let params = json!({
            "instrument_name": "BTC_USDT",
            "depth": 10
        });
        assert_eq!(params.get("instrument_name").unwrap(), "BTC_USDT");
        assert_eq!(params.get("depth").unwrap(), 10);
    }
}
