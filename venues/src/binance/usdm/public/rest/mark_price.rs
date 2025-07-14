//! Mark Price endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/premiumIndex
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price)

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Request parameters for mark price.
#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq)]
pub struct MarkPriceRequest {
    /// Trading pair symbol (e.g., "BTCUSDT"). Optional.
    #[serde(rename = "symbol")]
    pub symbol: Option<Cow<'static, str>>,
}

/// Represents a mark price response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarkPrice {
    /// Trading pair symbol (e.g., "BTCUSDT").
    #[serde(rename = "symbol")]
    pub symbol: Cow<'static, str>,

    /// Mark price as a string.
    #[serde(rename = "markPrice")]
    pub mark_price: Cow<'static, str>,

    /// Index price as a string.
    #[serde(rename = "indexPrice")]
    pub index_price: Cow<'static, str>,

    /// Estimated settle price as a string.
    #[serde(rename = "estimatedSettlePrice")]
    pub estimated_settle_price: Cow<'static, str>,

    /// Last funding rate as a string.
    #[serde(rename = "lastFundingRate")]
    pub last_funding_rate: Cow<'static, str>,

    /// Interest rate as a string.
    #[serde(rename = "interestRate")]
    pub interest_rate: Cow<'static, str>,

    /// Next funding time in ms.
    #[serde(rename = "nextFundingTime")]
    pub next_funding_time: u64,

    /// Timestamp in ms.
    #[serde(rename = "time")]
    pub time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkPriceResult {
    Multiple(Vec<MarkPrice>),
    Single(MarkPrice),
}

impl RestClient {
    /// Get mark price and funding rate (GET /fapi/v1/premiumIndex)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price)
    pub async fn get_mark_price(&self, params: MarkPriceRequest) -> RestResult<MarkPriceResult> {
        let query = params.symbol.map(|s| format!("symbol={}", s));
        self.send_request(
            "/fapi/v1/premiumIndex",
            reqwest::Method::GET,
            query.as_deref(),
            None,
            1,
        )
        .await
    }
}
