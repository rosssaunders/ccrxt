//! Index Price Kline/Candlestick Data endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/indexPriceKlines
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Price-Kline-Candlestick-Data)

use super::RestClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::KlineInterval;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for index price kline/candlestick data.
#[derive(Debug, Clone, Serialize)]
pub struct IndexPriceKlinesRequest {
    /// Pair (e.g., "BTCUSDT").
    pub pair: Cow<'static, str>,
    /// Kline interval.
    pub interval: KlineInterval,
    /// Start time in ms.
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>,
    /// End time in ms.
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>,
    /// Number of klines to return. Default 500; max 1500.
    pub limit: Option<u16>,
}

/// Represents a single index price kline/candlestick bar.
#[derive(Debug, Clone, Deserialize)]
pub struct IndexPriceKline {
    pub open_time: u64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub ignore1: String,
    pub close_time: u64,
    pub ignore2: String,
    pub ignore3: u64,
    pub ignore4: String,
    pub ignore5: String,
    pub ignore6: String,
}

impl RestClient {
    /// Get index price kline/candlestick bars (GET /fapi/v1/indexPriceKlines)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Price-Kline-Candlestick-Data)
    pub async fn get_index_price_klines(&self, params: IndexPriceKlinesRequest) -> RestResult<Vec<IndexPriceKline>> {
        let mut query = format!("pair={}&interval={}", params.pair, params.interval.as_str());
        if let Some(start_time) = params.start_time {
            query.push_str(&format!("&startTime={}", start_time));
        }
        if let Some(end_time) = params.end_time {
            query.push_str(&format!("&endTime={}", end_time));
        }
        if let Some(limit) = params.limit {
            query.push_str(&format!("&limit={}", limit));
        }
        self.send_request(
            "/fapi/v1/indexPriceKlines",
            reqwest::Method::GET,
            Some(&query),
            None,
            1,
        )
        .await
    }
}
