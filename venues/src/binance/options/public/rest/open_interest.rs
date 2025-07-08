use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

/// Request parameters for open interest
#[derive(Debug, Clone, Serialize)]
pub struct OpenInterestRequest {
    /// Underlying asset, e.g ETH/BTC
    #[serde(rename = "underlyingAsset")]
    pub underlying_asset: String,

    /// Expiration date, e.g 221225
    #[serde(rename = "expiration")]
    pub expiration: String,
}

/// Open interest information
#[derive(Debug, Clone, Deserialize)]
pub struct OpenInterestResponse {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Sum of open interest
    #[serde(rename = "sumOpenInterest")]
    pub sum_open_interest: Decimal,

    /// Sum of open interest in USD
    #[serde(rename = "sumOpenInterestUsd")]
    pub sum_open_interest_usd: Decimal,

    /// Timestamp
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

impl RestClient {
    /// Get open interest
    ///
    /// Returns open interest for specific underlying asset on specific expiration date.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-data/Open-Interest)
    /// Method: GET /eapi/v1/openInterest
    /// Weight: 0
    /// Security: None
    pub async fn get_open_interest(
        &self,
        params: OpenInterestRequest,
    ) -> RestResult<Vec<OpenInterestResponse>> {
        let query_string = serde_urlencoded::to_string(&params).map_err(|e| {
            crate::binance::options::Errors::Error(format!("URL encoding error: {e}"))
        })?;

        self.send_request(
            "/eapi/v1/openInterest",
            reqwest::Method::GET,
            Some(&query_string),
            None,
            0,
        )
        .await
    }
}
