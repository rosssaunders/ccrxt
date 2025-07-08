use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

/// Request parameters for order book
#[derive(Debug, Clone, Serialize)]
pub struct OrderBookRequest {
    /// Option trading pair, e.g BTC-200730-9000-C
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Default: 100, Max: 1000, Optional values: [10, 20, 50, 100, 500, 1000]
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Order book response
#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookResponse {
    /// Transaction time
    #[serde(rename = "T")]
    pub transaction_time: u64,

    /// Update ID
    #[serde(rename = "u")]
    pub update_id: u64,

    /// Buy orders
    #[serde(rename = "bids")]
    pub bids: Vec<OrderBookLevel>,

    /// Sell orders
    #[serde(rename = "asks")]
    pub asks: Vec<OrderBookLevel>,
}

/// Order book level (price and quantity)
#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookLevel(pub Decimal, pub Decimal);

impl RestClient {
    /// Get order book
    ///
    /// Returns orderbook depth for the specified symbol.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-data/Order-Book)
    /// Method: GET /eapi/v1/depth
    /// Weight: 2 (for 5,10,20,50), 5 (for 100), 10 (for 500), 20 (for 1000)
    /// Security: None
    pub async fn get_order_book(&self, params: OrderBookRequest) -> RestResult<OrderBookResponse> {
        let weight = match params.limit.unwrap_or(100) {
            5 | 10 | 20 | 50 => 2,
            100 => 5,
            500 => 10,
            1000 => 20,
            _ => 5, // Default to 100 weight
        };

        let query_string = serde_urlencoded::to_string(&params).map_err(|e| {
            crate::binance::options::Errors::Error(format!("URL encoding error: {e}"))
        })?;

        self.send_request(
            "/eapi/v1/depth",
            reqwest::Method::GET,
            Some(&query_string),
            None,
            weight,
        )
        .await
    }
}
