//! Top Trader Long/Short Ratio (Accounts) (GET /futures/data/topLongShortAccountRatio)
//!
//! See: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Top-Long-Short-Account-Ratio
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::{Errors, RestResult, enums::Period};

/// Request parameters for the Top Trader Long/Short Ratio (Accounts) endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct TopLongShortAccountRatioRequest<'a> {
    /// The symbol to query (e.g., "BTCUSDT").
    pub symbol: Cow<'a, str>,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopLongShortAccountRatioResponse<'a> {
    pub symbol: Cow<'a, str>,
    #[serde(rename = "longShortRatio")]
    pub long_short_ratio: Cow<'a, str>,
    #[serde(rename = "longAccount")]
    pub long_account: Cow<'a, str>,
    #[serde(rename = "shortAccount")]
    pub short_account: Cow<'a, str>,
    pub timestamp: u64,
}

impl RestClient {
    /// Top Trader Long/Short Ratio (Accounts) (GET /futures/data/topLongShortAccountRatio)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Top-Long-Short-Account-Ratio)
    pub async fn top_long_short_account_ratio<'a>(
        &self,
        params: TopLongShortAccountRatioRequest<'a>,
    ) -> RestResult<Vec<TopLongShortAccountRatioResponse<'a>>> {
        let endpoint = "/futures/data/topLongShortAccountRatio";
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| Errors::Error(format!("Failed to serialize params: {e}")))?;
        let resp = self
            .send_request::<Vec<TopLongShortAccountRatioResponse>>(
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
