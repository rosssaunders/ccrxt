//! Open Interest Statistics (GET /futures/data/openInterestHist)
//!
//! See: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest-Statistics
use super::RestClient;
use crate::binance::usdm::enums::Period;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for the Open Interest Statistics endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct OpenInterestHistRequest<'a> {
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
pub struct OpenInterestHistResponse<'a> {
    pub symbol: Cow<'a, str>,
    pub sum_open_interest: Cow<'a, str>,
    pub sum_open_interest_value: Cow<'a, str>,
    pub timestamp: Cow<'a, str>,
}

impl RestClient {
    /// Open Interest Statistics (GET /futures/data/openInterestHist)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest-Statistics)
    pub async fn open_interest_hist<'a>(&self, params: OpenInterestHistRequest<'a>) -> crate::binance::usdm::RestResult<Vec<OpenInterestHistResponse<'a>>> {
        let endpoint = "/futures/data/openInterestHist";
        let query = serde_urlencoded::to_string(&params).map_err(|e| crate::binance::usdm::Errors::Error(format!("Failed to serialize params: {e}")))?;
        let resp = self
            .send_request::<Vec<OpenInterestHistResponse>>(endpoint, reqwest::Method::GET, Some(&query), None, 0)
            .await?;
        Ok(resp)
    }
}
