use serde::{Deserialize, Serialize};

use crate::binance::coinm::RestResult;
use crate::binance::coinm::enums::KlineInterval;
use crate::binance::coinm::public::rest::RestClient;

/// Request parameters for the premium index kline data endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct PremiumIndexKlineRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Kline interval.
    #[serde(rename = "interval")]
    pub interval: KlineInterval,

    /// Start time in milliseconds.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Default 500; max 1500.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Represents a single premium index kline.
///
/// Klines are arrays with the following structure:
/// [Open time, Open, High, Low, Close, Ignore, Close time, Ignore, Ignore, Ignore, Ignore, Ignore]
#[derive(Debug, Clone, Deserialize)]
pub struct PremiumIndexKline(
    pub u64,    // Open time
    pub String, // Open
    pub String, // High
    pub String, // Low
    pub String, // Close
    pub String, // Ignore
    pub u64,    // Close time
    pub String, // Ignore
    pub u64,    // Ignore
    pub String, // Ignore
    pub String, // Ignore
    pub String, // Ignore
);

/// Response from the premium index kline data endpoint.
pub type PremiumIndexKlineResponse = Vec<PremiumIndexKline>;

impl RestClient {
    /// Premium index kline bars of a symbol. Klines are uniquely identified by their open time.
    ///
    /// [Official API docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Premium-Index-Kline-Data)
    ///
    /// Weight based on parameter LIMIT:
    /// - [1,100): 1
    /// - [100, 500): 2
    /// - [500, 1000]: 5
    /// - > 1000: 10
    pub async fn get_premium_index_klines(
        &self,
        params: PremiumIndexKlineRequest,
    ) -> RestResult<PremiumIndexKlineResponse> {
        let weight = match params.limit.unwrap_or(500) {
            1..=99 => 1,
            100..=499 => 2,
            500..=1000 => 5,
            _ => 10,
        };

        self.send_request(
            "/dapi/v1/premiumIndexKlines",
            reqwest::Method::GET,
            Some(params),
            weight,
        )
        .await
    }
}
