//! Premium Index Kline Data endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/premiumIndexKlines
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Premium-Index-Kline-Data)

use super::RestClient;
use crate::binance::usdm::KlineInterval;
use crate::binance::usdm::RestResult;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for premium index kline data.
#[derive(Debug, Clone, Serialize)]
pub struct PremiumIndexKlinesRequest {
    /// Symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,
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

/// Represents a single premium index kline bar.
#[derive(Debug, Clone, Deserialize)]
pub struct PremiumIndexKline {
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
    /// Get premium index kline bars (GET /fapi/v1/premiumIndexKlines)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Premium-Index-Kline-Data)
    pub async fn get_premium_index_klines(
        &self,
        params: PremiumIndexKlinesRequest,
    ) -> RestResult<Vec<PremiumIndexKline>> {
        let mut query = format!(
            "symbol={}&interval={}",
            params.symbol,
            params.interval.as_str()
        );
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
            "/fapi/v1/premiumIndexKlines",
            reqwest::Method::GET,
            Some(&query),
            None,
            1,
        )
        .await
    }
}
