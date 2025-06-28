use rust_decimal::Decimal;
use serde::Serialize;

use crate::binance::spot::RestResult;

use super::client::RestClient;

/// Request parameters for klines
#[derive(Debug, Clone, Serialize)]
pub struct KlinesRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Kline interval
    #[serde(rename = "interval")]
    pub interval: String,

    /// Start time timestamp in ms
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time timestamp in ms
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Time zone (default: 0 (UTC))
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    /// Number of klines to return. Default 500, Max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Kline data
/// Array format: [Open time, Open, High, Low, Close, Volume, Close time, Quote asset volume, Number of trades, Taker buy base asset volume, Taker buy quote asset volume, Unused field]
pub type KlineData = (
    u64,     // Open time
    Decimal, // Open price
    Decimal, // High price
    Decimal, // Low price
    Decimal, // Close price
    Decimal, // Volume
    u64,     // Close time
    Decimal, // Quote asset volume
    u64,     // Number of trades
    Decimal, // Taker buy base asset volume
    Decimal, // Taker buy quote asset volume
    String,  // Unused field, ignore
);

impl RestClient {
    /// Get kline/candlestick data
    ///
    /// Returns kline/candlestick bars for a symbol.
    /// Klines are uniquely identified by their open time.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#klinecandlestick-data)
    /// Method: GET /api/v3/klines
    /// Weight: 2
    /// Security: None
    pub async fn get_klines(&self, params: KlinesRequest) -> RestResult<Vec<KlineData>> {
        let query_string = serde_urlencoded::to_string(&params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {e}")))?;

        self.send_request(
            "/api/v3/klines",
            reqwest::Method::GET,
            Some(&query_string),
            2,
        )
        .await
    }
}
