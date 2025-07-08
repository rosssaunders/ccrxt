//! Kline/Candlestick Data endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/klines
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Kline-Candlestick-Data)

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::{RestResult, enums::KlineInterval};

/// Request parameters for kline/candlestick data.
#[derive(Debug, Clone, Serialize)]
pub struct KlinesRequest {
    /// Trading pair symbol (e.g., "BTCUSDT").
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

/// Represents a single kline/candlestick bar.
#[derive(Debug, Clone, Deserialize)]
pub struct Kline {
    pub open_time: u64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub close_time: u64,
    pub quote_asset_volume: String,
    pub number_of_trades: u64,
    pub taker_buy_base_asset_volume: String,
    pub taker_buy_quote_asset_volume: String,
    pub ignore: String,
}

impl RestClient {
    /// Get kline/candlestick bars (GET /fapi/v1/klines)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Kline-Candlestick-Data)
    pub async fn get_klines(&self, params: KlinesRequest) -> RestResult<Vec<Kline>> {
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
            "/fapi/v1/klines",
            reqwest::Method::GET,
            Some(&query),
            None,
            1,
        )
        .await
    }
}
