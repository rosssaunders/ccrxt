//! 24hr Ticker Price Change Statistics endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/ticker/24hr
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/24hr-Ticker-Price-Change-Statistics)

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Request parameters for 24hr ticker price change statistics.
#[derive(Debug, Clone, Serialize, Default)]
pub struct Ticker24hrRequest {
    /// Trading pair symbol (e.g., "BTCUSDT"). Optional.
    pub symbol: Option<Cow<'static, str>>,
}

/// Represents a 24hr ticker price change statistics response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker24hr {
    /// Trading pair symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,

    /// Price change in the last 24 hours.
    #[serde(rename = "priceChange")]
    pub price_change: String,

    /// Price change percent in the last 24 hours.
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: String,

    /// Weighted average price in the last 24 hours.
    #[serde(rename = "weightedAvgPrice")]
    pub weighted_avg_price: String,

    /// Last price.
    #[serde(rename = "lastPrice")]
    pub last_price: String,

    /// Last quantity.
    #[serde(rename = "lastQty")]
    pub last_qty: String,

    /// Open price 24 hours ago.
    #[serde(rename = "openPrice")]
    pub open_price: String,

    /// Highest price in the last 24 hours.
    #[serde(rename = "highPrice")]
    pub high_price: String,

    /// Lowest price in the last 24 hours.
    #[serde(rename = "lowPrice")]
    pub low_price: String,

    /// Total traded base asset volume in the last 24 hours.
    pub volume: String,

    /// Total traded quote asset volume in the last 24 hours.
    #[serde(rename = "quoteVolume")]
    pub quote_volume: String,

    /// Open time in milliseconds since epoch.
    #[serde(rename = "openTime")]
    pub open_time: u64,

    /// Close time in milliseconds since epoch.
    #[serde(rename = "closeTime")]
    pub close_time: u64,

    /// First trade ID.
    #[serde(rename = "firstId")]
    pub first_id: u64,

    /// Last trade ID.
    #[serde(rename = "lastId")]
    pub last_id: u64,

    /// Total number of trades.
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Ticker24hrResult {
    Multiple(Vec<Ticker24hr>),
    Single(Ticker24hr),
}

impl RestClient {
    /// Get 24hr ticker price change statistics (GET /fapi/v1/ticker/24hr)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/24hr-Ticker-Price-Change-Statistics)
    pub async fn get_ticker_24hr(&self, params: Ticker24hrRequest) -> RestResult<Ticker24hrResult> {
        let query = params.symbol.map(|s| format!("symbol={}", s));
        self.send_request(
            "/fapi/v1/ticker/24hr",
            reqwest::Method::GET,
            query.as_deref(),
            None,
            1,
        )
        .await
    }
}
