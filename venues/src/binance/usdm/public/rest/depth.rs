//! Order Book (Depth) endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/depth
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Order-Book)

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Request parameters for the order book (depth) endpoint.
#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq)]
pub struct OrderBookRequest {
    /// The trading pair symbol (e.g., "BTCUSDT").
    #[serde(rename = "symbol")]
    pub symbol: Cow<'static, str>,

    /// Optional limit for the number of order book levels (default 500).
    #[serde(rename = "limit")]
    pub limit: Option<u32>,
}

/// Response for the order book (depth) endpoint.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct OrderBookResponse {
    /// Last update ID for the order book.
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,

    /// Event time (optional).
    #[serde(rename = "E", default)]
    pub event_time: Option<u64>,

    /// Transaction time (optional).
    #[serde(rename = "T", default)]
    pub transaction_time: Option<u64>,

    /// Bids as a vector of (price, quantity) tuples.
    #[serde(rename = "bids")]
    pub bids: Vec<(Cow<'static, str>, Cow<'static, str>)>,

    /// Asks as a vector of (price, quantity) tuples.
    #[serde(rename = "asks")]
    pub asks: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl RestClient {
    /// Get the order book for a symbol (GET /fapi/v1/depth)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Order-Book)
    pub async fn get_order_book(&self, params: OrderBookRequest) -> RestResult<OrderBookResponse> {
        let mut query = format!("symbol={}", params.symbol);
        if let Some(limit) = params.limit {
            query.push_str(&format!("&limit={}", limit));
        }
        self.send_request(
            "/fapi/v1/depth",
            reqwest::Method::GET,
            Some(&query),
            None,
            2, // weight for default limit
        )
        .await
    }
}
