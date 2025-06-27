use crate::binance::coinm::RestResult;
use crate::binance::coinm::enums::{ContractType, KlineInterval};
use crate::binance::coinm::public::rest::RestClient;

use serde::{Deserialize, Serialize};

/// Request parameters for the continuous contract kline/candlestick data endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct ContinuousKlineRequest {
    /// Trading pair (e.g., "BTCUSD").
    #[serde(rename = "pair")]
    pub pair: String,

    /// Contract type.
    #[serde(rename = "contractType")]
    pub contract_type: ContractType,

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

/// Represents a single continuous contract kline/candlestick.
///
/// Klines are arrays with the following structure:
/// [Open time, Open, High, Low, Close, Volume, Close time, Base asset volume, Number of trades, Taker buy volume, Taker buy base asset volume, Ignore]
#[derive(Debug, Clone, Deserialize)]
pub struct ContinuousKline(
    pub u64,    // Open time
    pub String, // Open
    pub String, // High
    pub String, // Low
    pub String, // Close (or latest price)
    pub String, // Volume
    pub u64,    // Close time
    pub String, // Base asset volume
    pub u64,    // Number of trades
    pub String, // Taker buy volume
    pub String, // Taker buy base asset volume
    pub String, // Ignore
);

/// Response from the continuous contract kline/candlestick data endpoint.
pub type ContinuousKlineResponse = Vec<ContinuousKline>;

impl RestClient {
    /// Kline/candlestick bars for a specific contract type. Klines are uniquely identified by their open time.
    ///
    /// [Official API docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Continuous-Contract-Kline-Candlestick-Data)
    ///
    /// Weight based on parameter LIMIT:
    /// - [1,100): 1
    /// - [100, 500): 2
    /// - [500, 1000]: 5
    /// - > 1000: 10
    pub async fn get_continuous_klines(&self, params: ContinuousKlineRequest) -> RestResult<ContinuousKlineResponse> {
        let weight = match params.limit.unwrap_or(500) {
            1..=99 => 1,
            100..=499 => 2,
            500..=1000 => 5,
            _ => 10,
        };

        self.send_request(
            "/dapi/v1/continuousKlines",
            reqwest::Method::GET,
            Some(params),
            weight,
        )
        .await
    }
}
