use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

/// Request parameters for 24hr ticker
#[derive(Debug, Clone, Serialize, Default)]
pub struct TickerRequest {
    /// Option trading pair (if omitted, returns all symbols)
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// 24hr ticker price change statistics
#[derive(Debug, Clone, Deserialize)]
pub struct TickerResponse {
    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// 24-hour price change
    #[serde(rename = "priceChange")]
    pub price_change: Decimal,

    /// 24-hour percent price change
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: Decimal,

    /// Last trade price
    #[serde(rename = "lastPrice")]
    pub last_price: Decimal,

    /// Last trade amount
    #[serde(rename = "lastQty")]
    pub last_qty: Decimal,

    /// 24-hour open price
    #[serde(rename = "open")]
    pub open: Decimal,

    /// 24-hour high
    #[serde(rename = "high")]
    pub high: Decimal,

    /// 24-hour low
    #[serde(rename = "low")]
    pub low: Decimal,

    /// Trading volume (contracts)
    #[serde(rename = "volume")]
    pub volume: Decimal,

    /// Trade amount (in quote asset)
    #[serde(rename = "amount")]
    pub amount: Decimal,

    /// The best buy price
    #[serde(rename = "bidPrice")]
    pub bid_price: Decimal,

    /// The best sell price
    #[serde(rename = "askPrice")]
    pub ask_price: Decimal,

    /// Time the first trade occurred within the last 24 hours
    #[serde(rename = "openTime")]
    pub open_time: u64,

    /// Time the last trade occurred within the last 24 hours
    #[serde(rename = "closeTime")]
    pub close_time: u64,

    /// First trade ID
    #[serde(rename = "firstTradeId")]
    pub first_trade_id: u64,

    /// Number of trades
    #[serde(rename = "tradeCount")]
    pub trade_count: u64,

    /// Strike price
    #[serde(rename = "strikePrice")]
    pub strike_price: Decimal,

    /// Estimated settlement price one hour before exercise, index price at other times
    #[serde(rename = "exercisePrice")]
    pub exercise_price: Decimal,
}

impl RestClient {
    /// Get 24hr ticker price change statistics
    ///
    /// Returns 24 hour rolling window price change statistics.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-data/24hr-Ticker-Price-Change-Statistics)
    /// Method: GET /eapi/v1/ticker
    /// Weight: 5
    /// Security: None
    pub async fn get_ticker(&self, params: TickerRequest) -> RestResult<Vec<TickerResponse>> {
        let query_string = if params.symbol.is_some() {
            Some(serde_urlencoded::to_string(&params).map_err(|e| {
                crate::binance::options::Errors::Error(format!("URL encoding error: {e}"))
            })?)
        } else {
            None
        };

        self.send_request(
            "/eapi/v1/ticker",
            reqwest::Method::GET,
            query_string.as_deref(),
            None,
            5,
        )
        .await
    }
}
