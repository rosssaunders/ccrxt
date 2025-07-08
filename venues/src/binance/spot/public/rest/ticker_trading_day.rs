use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

/// Request parameters for trading day ticker statistics
#[derive(Debug, Clone, Serialize)]
pub struct TickerTradingDayRequest {
    /// Single symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Multiple symbols (JSON array format)
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,

    /// Time zone (default: 0 (UTC))
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    /// Type of ticker response (FULL or MINI)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ticker_type: Option<String>,
}

/// Trading day ticker statistics
#[derive(Debug, Clone, Deserialize)]
pub struct TickerTradingDay {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Price change
    #[serde(rename = "priceChange")]
    pub price_change: Decimal,

    /// Price change percent
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: Decimal,

    /// Weighted average price
    #[serde(rename = "weightedAvgPrice")]
    pub weighted_avg_price: Decimal,

    /// Open price
    #[serde(rename = "openPrice")]
    pub open_price: Decimal,

    /// High price
    #[serde(rename = "highPrice")]
    pub high_price: Decimal,

    /// Low price
    #[serde(rename = "lowPrice")]
    pub low_price: Decimal,

    /// Last price
    #[serde(rename = "lastPrice")]
    pub last_price: Decimal,

    /// Total traded base asset volume
    #[serde(rename = "volume")]
    pub volume: Decimal,

    /// Total traded quote asset volume
    #[serde(rename = "quoteVolume")]
    pub quote_volume: Decimal,

    /// Statistics open time
    #[serde(rename = "openTime")]
    pub open_time: u64,

    /// Statistics close time
    #[serde(rename = "closeTime")]
    pub close_time: u64,

    /// First trade ID
    #[serde(rename = "firstId")]
    pub first_id: u64,

    /// Last trade ID
    #[serde(rename = "lastId")]
    pub last_id: u64,

    /// Total number of trades
    #[serde(rename = "count")]
    pub count: u64,
}

impl RestClient {
    /// Get trading day ticker
    ///
    /// Price change statistics for a trading day.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#trading-day-ticker)
    /// Method: GET /api/v3/ticker/tradingDay
    /// Weight: 4 per symbol (max 200)
    /// Security: None
    pub async fn get_trading_day_ticker(
        &self,
        params: TickerTradingDayRequest,
    ) -> RestResult<serde_json::Value> {
        let query_string = serde_urlencoded::to_string(&params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {e}")))?;

        // Weight is 4 per symbol, max 200 for multiple symbols
        let weight = if params.symbols.is_some() { 200 } else { 4 };

        self.send_request(
            "/api/v3/ticker/tradingDay",
            reqwest::Method::GET,
            Some(&query_string),
            weight,
        )
        .await
    }
}
