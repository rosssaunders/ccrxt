use serde::{Deserialize, Serialize};

use crate::binance::coinm::RestResult;
use crate::binance::coinm::public::rest::RestClient;

/// Request parameters for the funding rate history endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct FundingRateRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Timestamp in ms to get funding rate from INCLUSIVE.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// Timestamp in ms to get funding rate until INCLUSIVE.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Default 100; max 1000.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Represents a single funding rate entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    /// Trading symbol.
    pub symbol: String,

    /// Funding time.
    pub funding_time: u64,

    /// Funding rate.
    pub funding_rate: String,
}

/// Response from the funding rate history endpoint.
pub type FundingRateResponse = Vec<FundingRate>;

impl RestClient {
    /// Get Funding Rate History of Perpetual Futures.
    ///
    /// Empty array will be returned for delivery symbols.
    ///
    /// [Official API docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Get-Funding-Rate-History-of-Perpetual-Futures)
    ///
    /// Weight: 1
    pub async fn get_funding_rate_history(&self, params: FundingRateRequest) -> RestResult<FundingRateResponse> {
        self.send_request(
            "/dapi/v1/fundingRate",
            reqwest::Method::GET,
            Some(params),
            1,
        )
        .await
    }
}
