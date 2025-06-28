use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::options::RestResult;

use super::client::RestClient;

/// Request parameters for klines
#[derive(Debug, Clone, Serialize)]
pub struct KlinesRequest {
    /// Option trading pair, e.g BTC-200730-9000-C
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Time interval
    #[serde(rename = "interval")]
    pub interval: String,

    /// Start time
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of records (Default: 500, Max: 1500)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Kline/candlestick data
#[derive(Debug, Clone, Deserialize)]
pub struct KlineResponse {
    /// Opening price
    #[serde(rename = "open")]
    pub open: Decimal,

    /// Highest price
    #[serde(rename = "high")]
    pub high: Decimal,

    /// Lowest price
    #[serde(rename = "low")]
    pub low: Decimal,

    /// Closing price (latest price if the current candle has not closed)
    #[serde(rename = "close")]
    pub close: Decimal,

    /// Trading volume (contracts)
    #[serde(rename = "volume")]
    pub volume: Decimal,

    /// Trading amount (in quote asset)
    #[serde(rename = "amount")]
    pub amount: Decimal,

    /// Candle type
    #[serde(rename = "interval")]
    pub interval: String,

    /// Number of completed trades
    #[serde(rename = "tradeCount")]
    pub trade_count: u64,

    /// Taker trading volume (contracts)
    #[serde(rename = "takerVolume")]
    pub taker_volume: Decimal,

    /// Taker trade amount (in quote asset)
    #[serde(rename = "takerAmount")]
    pub taker_amount: Decimal,

    /// Opening time
    #[serde(rename = "openTime")]
    pub open_time: u64,

    /// Closing time
    #[serde(rename = "closeTime")]
    pub close_time: u64,
}

impl RestClient {
    /// Get kline/candlestick data
    ///
    /// Returns kline/candlestick bars for an option symbol. Klines are uniquely identified by their open time.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-data/Kline-Candlestick-Data)
    /// Method: GET /eapi/v1/klines
    /// Weight: 1
    /// Security: None
    pub async fn get_klines(&self, params: KlinesRequest) -> RestResult<Vec<KlineResponse>> {
        let query_string = serde_urlencoded::to_string(&params).map_err(|e| {
            crate::binance::options::Errors::Error(format!("URL encoding error: {e}"))
        })?;

        self.send_request(
            "/eapi/v1/klines",
            reqwest::Method::GET,
            Some(&query_string),
            None,
            1,
        )
        .await
    }
}
