use rust_decimal::Decimal;
use serde::Serialize;

use super::client::RestClient;
use crate::binance::spot::RestResult;

/// Request parameters for UI klines
#[derive(Debug, Clone, Serialize)]
pub struct UiKlinesRequest {
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

/// UI Kline data - same structure as regular klines
/// Array format: [Open time, Open, High, Low, Close, Volume, Close time, Quote asset volume, Number of trades, Taker buy base asset volume, Taker buy quote asset volume, Unused field]
pub type UiKlineData = (
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
    /// Get UI kline/candlestick data
    ///
    /// The request is similar to klines having the same parameters and response.
    /// uiKlines return modified kline data, optimized for presentation of candlestick charts.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#uiklines)
    /// Method: GET /api/v3/uiKlines
    /// Weight: 2
    /// Security: None
    pub async fn get_ui_klines(&self, params: UiKlinesRequest) -> RestResult<Vec<UiKlineData>> {
        let query_string = serde_urlencoded::to_string(&params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {e}")))?;

        self.send_request(
            "/api/v3/uiKlines",
            reqwest::Method::GET,
            Some(&query_string),
            2,
        )
        .await
    }
}
