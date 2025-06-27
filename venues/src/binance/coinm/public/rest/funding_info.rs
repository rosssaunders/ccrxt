use serde::{Deserialize, Serialize};

use crate::binance::coinm::RestResult;
use crate::binance::coinm::public::rest::RestClient;

/// Request parameters for the funding rate info endpoint.
/// No parameters required for this endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct FundingInfoRequest {}

/// Represents funding rate info for a symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingInfo {
    /// Trading symbol.
    pub symbol: String,

    /// Adjusted funding rate cap.
    pub adjusted_funding_rate_cap: String,

    /// Adjusted funding rate floor.
    pub adjusted_funding_rate_floor: String,

    /// Funding interval hours.
    pub funding_interval_hours: u32,

    /// Disclaimer flag.
    pub disclaimer: bool,
}

/// Response from the funding rate info endpoint.
pub type FundingInfoResponse = Vec<FundingInfo>;

impl RestClient {
    /// Query funding rate info for symbols that had FundingRateCap/FundingRateFloor/fundingIntervalHours adjustment.
    ///
    /// [Official API docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Get-Funding-Info)
    ///
    /// Weight: 1
    pub async fn get_funding_info(&self) -> RestResult<FundingInfoResponse> {
        self.send_request("/dapi/v1/fundingInfo", reqwest::Method::GET, None::<()>, 1)
            .await
    }
}
