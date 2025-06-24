//! Get Funding Rate History endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/fundingRate
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-History)

use super::RestClient;
use crate::binance::usdm::RestResult;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for funding rate history.
#[derive(Debug, Clone, Serialize, Default)]
pub struct FundingRateHistoryRequest {
    /// Trading pair symbol (e.g., "BTCUSDT"). Optional.
    pub symbol: Option<Cow<'static, str>>,
    /// Timestamp in ms to get funding rate from INCLUSIVE.
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>,
    /// Timestamp in ms to get funding rate until INCLUSIVE.
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>,
    /// Number of records to return. Default 100; max 1000.
    pub limit: Option<u16>,
}

/// Represents a funding rate history record.
#[derive(Debug, Clone, Deserialize)]
pub struct FundingRateHistory {
    pub symbol: Cow<'static, str>,
    #[serde(rename = "fundingRate")]
    pub funding_rate: String,
    #[serde(rename = "fundingTime")]
    pub funding_time: u64,
    #[serde(rename = "markPrice")]
    pub mark_price: Option<String>,
}

impl RestClient {
    /// Get funding rate history (GET /fapi/v1/fundingRate)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-History)
    pub async fn get_funding_rate_history(&self, params: FundingRateHistoryRequest) -> RestResult<Vec<FundingRateHistory>> {
        let mut query = String::new();
        if let Some(symbol) = params.symbol {
            query.push_str(&format!("symbol={}", symbol));
        }
        if let Some(start_time) = params.start_time {
            if !query.is_empty() {
                query.push('&');
            }
            query.push_str(&format!("startTime={}", start_time));
        }
        if let Some(end_time) = params.end_time {
            if !query.is_empty() {
                query.push('&');
            }
            query.push_str(&format!("endTime={}", end_time));
        }
        if let Some(limit) = params.limit {
            if !query.is_empty() {
                query.push('&');
            }
            query.push_str(&format!("limit={}", limit));
        }
        let query_opt = if query.is_empty() {
            None
        } else {
            Some(query.as_str())
        };
        self.send_request(
            "/fapi/v1/fundingRate",
            reqwest::Method::GET,
            query_opt,
            None,
            1,
        )
        .await
    }
}
