//! Symbol Price Ticker endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/ticker/price
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker)

use super::RestClient;
use crate::binance::usdm::RestResult;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for symbol price ticker.
#[derive(Debug, Clone, Serialize, Default)]
pub struct TickerPriceRequest {
    /// Trading pair symbol (e.g., "BTCUSDT"). Optional.
    pub symbol: Option<Cow<'static, str>>,
}

/// Represents a symbol price ticker response.
#[derive(Debug, Clone, Deserialize)]
pub struct TickerPrice {
    pub symbol: Cow<'static, str>,
    pub price: String,
    pub time: Option<u64>,
}

impl RestClient {
    /// Get latest price for a symbol or symbols (GET /fapi/v1/ticker/price)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker)
    pub async fn get_ticker_price(&self, params: TickerPriceRequest) -> RestResult<Vec<TickerPrice>> {
        let query = params.symbol.map(|s| format!("symbol={}", s));
        self.send_request(
            "/fapi/v1/ticker/price",
            reqwest::Method::GET,
            query.as_deref(),
            None,
            1,
        )
        .await
    }
}
