//! Taker Buy/Sell Volume (GET /futures/data/takerlongshortRatio)
//!
//! See: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Taker-BuySell-Volume
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::{RestResult, enums::Period};

/// Request parameters for the Taker Buy/Sell Volume endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct TakerLongShortRatioRequest<'a> {
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
pub struct TakerLongShortRatioResponse<'a> {
    pub buy_sell_ratio: Cow<'a, str>,
    pub buy_vol: Cow<'a, str>,
    pub sell_vol: Cow<'a, str>,
    pub timestamp: Cow<'a, str>,
}

impl RestClient {
    /// Taker Buy/Sell Volume (GET /futures/data/takerlongshortRatio)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Taker-BuySell-Volume)
    pub async fn taker_long_short_ratio<'a>(
        &self,
        params: TakerLongShortRatioRequest<'a>,
    ) -> RestResult<Vec<TakerLongShortRatioResponse<'a>>> {
        let endpoint = "/futures/data/takerlongshortRatio";
        let query = serde_urlencoded::to_string(&params).map_err(|e| {
            crate::binance::usdm::Errors::Error(format!("Failed to serialize params: {e}"))
        })?;
        let resp = self
            .send_request::<Vec<TakerLongShortRatioResponse>>(
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
