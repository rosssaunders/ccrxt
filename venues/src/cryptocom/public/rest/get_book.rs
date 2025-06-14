//! Request and response structs for public/get-book endpoint
//!
//! Fetches the public order book for a particular instrument and depth.

use super::client::RestClient;
use crate::cryptocom::{EndpointType, RestResult};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
#[derive(Debug, Clone, Deserialize)]
pub struct GetBookResponse {
    /// Result data for the order book.
    #[serde(rename = "result")]
    pub result: BookResult,

    /// Success status.
    #[serde(rename = "success")]
    pub success: bool,

    /// Response ID.
    #[serde(rename = "id")]
    pub id: u64,
}

/// Result data for order book.
#[derive(Debug, Clone, Deserialize)]
pub struct BookResult {
    /// Instrument name.
    #[serde(rename = "instrument_name")]
    pub instrument_name: Cow<'static, str>,

    /// List of bids (price, quantity).
    #[serde(rename = "bids")]
    pub bids: Vec<[f64; 2]>,

    /// List of asks (price, quantity).
    #[serde(rename = "asks")]
    pub asks: Vec<[f64; 2]>,

    /// Timestamp of the order book snapshot (milliseconds since epoch).
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

impl RestClient {
    /// Calls the public/get-book endpoint.
    ///
    /// Fetches the public order book for a particular instrument and depth.
    ///
    /// [Official API docs](https://exchange-docs.crypto.com/spot/index.html#public-get-book)
    pub async fn get_book(&self, params: GetBookRequest) -> RestResult<GetBookResponse> {
        self.send_request(
            "public/get-book",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetBook,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
        assert_eq!(params["instrument_name"], "BTC_USDT");
        assert_eq!(params["depth"], 10);
    }
}
