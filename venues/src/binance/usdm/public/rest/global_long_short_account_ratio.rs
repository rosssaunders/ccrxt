//! Long/Short Ratio (GET /futures/data/globalLongShortAccountRatio)
//!
//! See: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Long-Short-Ratio

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::enums::Period;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalLongShortAccountRatioResponse<'a> {
    pub symbol: Cow<'a, str>,
    pub long_short_ratio: Cow<'a, str>,
    pub long_account: Cow<'a, str>,
    pub short_account: Cow<'a, str>,
    pub timestamp: Cow<'a, str>,
}

/// Request parameters for the global long/short account ratio endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct GlobalLongShortAccountRatioRequest<'a> {
    /// The symbol to query (e.g., "BTCUSDT").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'a, str>>,

    /// The period interval (e.g., "5m", "1h").
    pub period: Period,

    /// Number of data points to return (default 30, max 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Start time in milliseconds since epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds since epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
}

impl RestClient {
    /// Query symbol Long/Short Ratio (GET /futures/data/globalLongShortAccountRatio)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Long-Short-Ratio)
    pub async fn global_long_short_account_ratio<'a>(
        &self,
        params: GlobalLongShortAccountRatioRequest<'a>,
    ) -> crate::binance::usdm::RestResult<Vec<GlobalLongShortAccountRatioResponse<'a>>> {
        let endpoint = "/futures/data/globalLongShortAccountRatio";
        let query = serde_urlencoded::to_string(&params).map_err(|e| {
            crate::binance::usdm::Errors::Error(format!("Failed to serialize params: {e}"))
        })?;
        let resp = self
            .send_request::<Vec<GlobalLongShortAccountRatioResponse>>(
                endpoint,
                reqwest::Method::GET,
                Some(&query),
                None,
                0,
            )
            .await?;
        Ok(resp)
    }
}
