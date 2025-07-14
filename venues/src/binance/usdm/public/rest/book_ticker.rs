//! Symbol Order Book Ticker endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/ticker/bookTicker
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Order-Book-Ticker)

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Request parameters for symbol order book ticker.
#[derive(Debug, Clone, Serialize, Default)]
pub struct BookTickerRequest {
    /// Trading pair symbol (e.g., "BTCUSDT"). Optional.
    pub symbol: Option<Cow<'static, str>>,
}

/// Represents a symbol order book ticker response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookTicker {
    pub symbol: Cow<'static, str>,
    #[serde(rename = "bidPrice")]
    pub bid_price: String,
    #[serde(rename = "bidQty")]
    pub bid_qty: String,
    #[serde(rename = "askPrice")]
    pub ask_price: String,
    #[serde(rename = "askQty")]
    pub ask_qty: String,
    pub time: Option<u64>,
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BookTickerResult {
    Multiple(Vec<BookTicker>),
    Single(BookTicker),
}

impl RestClient {
    /// Get best price/qty on the order book for a symbol or symbols (GET /fapi/v1/ticker/bookTicker)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Order-Book-Ticker)
    pub async fn get_book_ticker(&self, params: BookTickerRequest) -> RestResult<BookTickerResult> {
        let query = params.symbol.map(|s| format!("symbol={}", s));
        self.send_request(
            "/fapi/v1/ticker/bookTicker",
            reqwest::Method::GET,
            query.as_deref(),
            None,
            2,
        )
        .await
    }
}
