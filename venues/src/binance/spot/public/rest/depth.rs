use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

/// Request parameters for order book depth
#[derive(Debug, Clone, Serialize)]
pub struct DepthRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Number of entries to return. Default 100, Max 5000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Order book depth response
#[derive(Debug, Clone, Deserialize)]
pub struct DepthResponse {
    /// Last update ID
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,

    /// Bid orders (price, quantity)
    #[serde(rename = "bids")]
    pub bids: Vec<(Decimal, Decimal)>,

    /// Ask orders (price, quantity)
    #[serde(rename = "asks")]
    pub asks: Vec<(Decimal, Decimal)>,
}

impl RestClient {
    /// Get order book depth
    ///
    /// Returns the order book for a given symbol.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#order-book)
    /// Method: GET /api/v3/depth
    /// Weight: Variable based on limit (5-250)
    /// Security: None
    pub async fn get_depth(&self, params: DepthRequest) -> RestResult<DepthResponse> {
        // Calculate weight based on limit
        let weight = match params.limit.unwrap_or(100) {
            1..=100 => 5,
            101..=500 => 25,
            501..=1000 => 50,
            1001..=5000 => 250,
            _ => 250, // Use max weight for any value above 5000
        };

        let query_string = serde_urlencoded::to_string(&params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {e}")))?;

        self.send_request(
            "/api/v3/depth",
            reqwest::Method::GET,
            Some(&query_string),
            weight,
        )
        .await
    }
}
