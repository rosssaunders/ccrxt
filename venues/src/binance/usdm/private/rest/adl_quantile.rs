//! ADL quantile endpoints for Binance USDM REST API.

use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const ADL_QUANTILE_ENDPOINT: &str = "/fapi/v1/adlQuantile";

/// Request for getting ADL quantile estimation.
#[derive(Debug, Clone, Serialize)]
pub struct GetAdlQuantileRequest {
    pub symbol: Option<Cow<'static, str>>,
}

/// ADL quantile values for different position sides.
#[derive(Debug, Clone, Deserialize)]
pub struct AdlQuantileValues {
    /// ADL quantile for LONG position in hedge mode or position in one-way mode.
    #[serde(rename = "LONG")]
    pub long: Option<u8>,
    /// ADL quantile for SHORT position in hedge mode.
    #[serde(rename = "SHORT")]
    pub short: Option<u8>,
    /// ADL quantile for position in one-way mode.
    #[serde(rename = "BOTH")]
    pub both: Option<u8>,
    /// Sign for hedge mode (only a sign, ignore the value).
    #[serde(rename = "HEDGE")]
    pub hedge: Option<u8>,
}

/// Response for ADL quantile estimation.
#[derive(Debug, Clone, Deserialize)]
pub struct AdlQuantileResponse {
    pub symbol: String,

    #[serde(rename = "adlQuantile")]
    pub adl_quantile: AdlQuantileValues,
}

impl UsdmClient {
    /// ADL quantile estimation (GET /fapi/v1/adlQuantile)
    ///
    /// Retrieves ADL quantile estimation for positions.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Position-ADL-Quantile-Estimation
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `params` - The request parameters
    ///
    /// # Returns
    /// ADL quantile response
    pub async fn get_adl_quantile(
        &self,
        params: GetAdlQuantileRequest,
    ) -> RestResult<Vec<AdlQuantileResponse>> {
        self.send_signed_request(ADL_QUANTILE_ENDPOINT, Method::GET, params, 5, false)
            .await
    }
}
