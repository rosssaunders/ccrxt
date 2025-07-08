//! Get Funding Rate Info endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/fundingInfo
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-Info)
use std::borrow::Cow;

use serde::Deserialize;

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Represents a funding rate info record returned by Binance USDM.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FundingRateInfo {
    /// Trading pair symbol (e.g., "BTCUSDT").
    #[serde(rename = "symbol")]
    pub symbol: Cow<'static, str>,

    /// The adjusted funding rate cap as a string (decimal).
    #[serde(rename = "adjustedFundingRateCap")]
    pub adjusted_funding_rate_cap: Cow<'static, str>,

    /// The adjusted funding rate floor as a string (decimal).
    #[serde(rename = "adjustedFundingRateFloor")]
    pub adjusted_funding_rate_floor: Cow<'static, str>,

    /// Funding interval in hours.
    #[serde(rename = "fundingIntervalHours")]
    pub funding_interval_hours: u32,
    // disclaimer: bool (ignored)
}

impl RestClient {
    /// Get funding rate info (GET /fapi/v1/fundingInfo)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-Info)
    pub async fn get_funding_rate_info(&self) -> RestResult<Vec<FundingRateInfo>> {
        self.send_request("/fapi/v1/fundingInfo", reqwest::Method::GET, None, None, 1)
            .await
    }
}
