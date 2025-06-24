//! Continuous Contract Kline/Candlestick Data endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/continuousKlines
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Continuous-Contract-Kline-Candlestick-Data)

use super::RestClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::{ContractType, KlineInterval};

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for continuous contract kline/candlestick data.
#[derive(Debug, Clone, Serialize)]
pub struct ContinuousKlinesRequest {
    /// Pair (e.g., "BTCUSDT").
    pub pair: Cow<'static, str>,
    /// Contract type (PERPETUAL, CURRENT_QUARTER, NEXT_QUARTER).
    #[serde(rename = "contractType")]
    pub contract_type: ContractType,
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

/// Represents a single continuous contract kline/candlestick bar.
#[derive(Debug, Clone, Deserialize)]
pub struct ContinuousKline {
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
    /// Get continuous contract kline/candlestick bars (GET /fapi/v1/continuousKlines)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Continuous-Contract-Kline-Candlestick-Data)
    pub async fn get_continuous_klines(&self, params: ContinuousKlinesRequest) -> RestResult<Vec<ContinuousKline>> {
        let mut query = format!(
            "pair={}&contractType={}&interval={}",
            params.pair,
            params.contract_type.as_str(),
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
            "/fapi/v1/continuousKlines",
            reqwest::Method::GET,
            Some(&query),
            None,
            1,
        )
        .await
    }
}
