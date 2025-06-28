use serde::{Deserialize, Serialize};

use crate::binance::coinm::RestResult;
use crate::binance::coinm::enums::KlineInterval;
use crate::binance::coinm::public::rest::RestClient;

/// Request parameters for the kline/candlestick data endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct KlineRequest {
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

/// Represents a single kline/candlestick.
///
/// Klines are arrays with the following structure:
/// [Open time, Open, High, Low, Close, Volume, Close time, Base asset volume, Number of trades, Taker buy volume, Taker buy base asset volume, Ignore]
#[derive(Debug, Clone, Deserialize)]
pub struct Kline(
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

/// Response from the kline/candlestick data endpoint.
pub type KlineResponse = Vec<Kline>;

impl RestClient {
    /// Kline/candlestick bars for a symbol. Klines are uniquely identified by their open time.
    ///
    /// [Official API docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Kline-Candlestick-Data)
    ///
    /// Weight based on parameter LIMIT:
    /// - [1,100): 1
    /// - [100, 500): 2
    /// - [500, 1000]: 5
    /// - > 1000: 10
    pub async fn get_klines(&self, params: KlineRequest) -> RestResult<KlineResponse> {
        let weight = match params.limit.unwrap_or(500) {
            1..=99 => 1,
            100..=499 => 2,
            500..=1000 => 5,
            _ => 10,
        };

        self.send_request(
            "/dapi/v1/klines",
            reqwest::Method::GET,
            Some(params),
            weight,
        )
        .await
    }
}
