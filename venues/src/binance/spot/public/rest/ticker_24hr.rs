use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

/// Request parameters for 24hr ticker statistics
#[derive(Debug, Clone, Serialize, Default)]
pub struct Ticker24hrRequest {
    /// Single symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Multiple symbols (JSON array format)
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,

    /// Type of ticker response (FULL or MINI)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ticker_type: Option<String>,
}

/// 24hr ticker statistics (FULL type)
#[derive(Debug, Clone, Deserialize)]
pub struct Ticker24hr {
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

    /// Previous close price
    #[serde(rename = "prevClosePrice")]
    pub prev_close_price: Decimal,

    /// Last price
    #[serde(rename = "lastPrice")]
    pub last_price: Decimal,

    /// Last quantity
    #[serde(rename = "lastQty")]
    pub last_qty: Decimal,

    /// Best bid price
    #[serde(rename = "bidPrice")]
    pub bid_price: Decimal,

    /// Best bid quantity
    #[serde(rename = "bidQty")]
    pub bid_qty: Decimal,

    /// Best ask price
    #[serde(rename = "askPrice")]
    pub ask_price: Decimal,

    /// Best ask quantity
    #[serde(rename = "askQty")]
    pub ask_qty: Decimal,

    /// Open price
    #[serde(rename = "openPrice")]
    pub open_price: Decimal,

    /// High price
    #[serde(rename = "highPrice")]
    pub high_price: Decimal,

    /// Low price
    #[serde(rename = "lowPrice")]
    pub low_price: Decimal,

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

/// 24hr ticker statistics (MINI type)
#[derive(Debug, Clone, Deserialize)]
pub struct Ticker24hrMini {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Close price
    #[serde(rename = "closePrice")]
    pub close_price: Decimal,

    /// Open price
    #[serde(rename = "openPrice")]
    pub open_price: Decimal,

    /// High price
    #[serde(rename = "highPrice")]
    pub high_price: Decimal,

    /// Low price
    #[serde(rename = "lowPrice")]
    pub low_price: Decimal,

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
    /// Get 24hr ticker price change statistics
    ///
    /// Returns 24 hour rolling window price change statistics.
    /// Careful when accessing this with no symbol.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#24hr-ticker-price-change-statistics)
    /// Method: GET /api/v3/ticker/24hr
    /// Weight: Variable (2-80 based on symbols count)
    /// Security: None
    pub async fn get_24hr_ticker(
        &self,
        params: Option<Ticker24hrRequest>,
    ) -> RestResult<serde_json::Value> {
        let (query_string, weight) = if let Some(p) = params {
            let weight = if p.symbol.is_some() {
                2 // Single symbol
            } else if p.symbols.is_some() {
                40 // Multiple symbols
            } else {
                80 // All symbols
            };

            let qs = serde_urlencoded::to_string(&p).map_err(|e| {
                crate::binance::spot::Errors::Error(format!("URL encoding error: {e}"))
            })?;
            (Some(qs), weight)
        } else {
            (None, 80) // All symbols
        };

        self.send_request(
            "/api/v3/ticker/24hr",
            reqwest::Method::GET,
            query_string.as_deref(),
            weight,
        )
        .await
    }
}
