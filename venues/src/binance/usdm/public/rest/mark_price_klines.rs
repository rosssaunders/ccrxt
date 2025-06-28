//! Mark Price Kline/Candlestick Data endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/markPriceKlines
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price-Kline-Candlestick-Data)

use super::RestClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::KlineInterval;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for mark price kline/candlestick data.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MarkPriceKlinesRequest {
    /// Symbol (e.g., "BTCUSDT").
    #[serde(rename = "symbol")]
    pub symbol: Cow<'static, str>,

    /// Kline interval.
    #[serde(rename = "interval")]
    pub interval: KlineInterval,

    /// Start time in ms.
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>,

    /// End time in ms.
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>,

    /// Number of klines to return. Default 500; max 1500.
    #[serde(rename = "limit")]
    pub limit: Option<u16>,
}

/// Represents a single mark price kline/candlestick bar.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MarkPriceKline {
    /// Open time in ms.
    pub open_time: u64,
    /// Open price as string.
    pub open: Cow<'static, str>,
    /// High price as string.
    pub high: Cow<'static, str>,
    /// Low price as string.
    pub low: Cow<'static, str>,
    /// Close price as string.
    pub close: Cow<'static, str>,
    /// Ignore field 1.
    pub ignore1: Cow<'static, str>,
    /// Close time in ms.
    pub close_time: u64,
    /// Ignore field 2.
    pub ignore2: Cow<'static, str>,
    /// Ignore field 3.
    pub ignore3: u64,
    /// Ignore field 4.
    pub ignore4: Cow<'static, str>,
    /// Ignore field 5.
    pub ignore5: Cow<'static, str>,
    /// Ignore field 6.
    pub ignore6: Cow<'static, str>,
}

impl RestClient {
    /// Get mark price kline/candlestick bars (GET /fapi/v1/markPriceKlines)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price-Kline-Candlestick-Data)
    pub async fn get_mark_price_klines(
        &self,
        params: MarkPriceKlinesRequest,
    ) -> RestResult<Vec<MarkPriceKline>> {
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
            "/fapi/v1/markPriceKlines",
            reqwest::Method::GET,
            Some(&query),
            None,
            1,
        )
        .await
    }
}
