use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::{ApiError, RestResponse};

const TICKER_ENDPOINT: &str = "/api/v2/spot/market/tickers";

/// Request for getting ticker information
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetTickerRequest {
    /// Specific symbol to query, if empty returns all symbols
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Ticker information
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    /// Trading pair symbol
    pub symbol: String,
    /// 24h highest price
    pub high24h: String,
    /// 24h open price
    pub open: String,
    /// Latest price
    #[serde(rename = "lastPr")]
    pub last_price: String,
    /// 24h lowest price
    pub low24h: String,
    /// Trading volume in quote currency
    pub quote_volume: String,
    /// Trading volume in base currency
    pub base_volume: String,
    /// Trading volume in USDT
    pub usdt_volume: String,
    /// Bid 1 price
    #[serde(rename = "bidPr")]
    pub bid_price: String,
    /// Ask 1 price
    #[serde(rename = "askPr")]
    pub ask_price: String,
    /// Buying 1 amount
    #[serde(rename = "bidSz")]
    pub bid_size: Option<String>,
    /// Selling 1 amount
    #[serde(rename = "askSz")]
    pub ask_size: Option<String>,
    /// UTC±00:00 Entry price
    #[serde(rename = "openUtc")]
    pub open_utc: String,
    /// Current time Unix millisecond timestamp
    pub ts: String,
    /// Change at UTC+0, 0.01 means 1%
    #[serde(rename = "changeUtc24h")]
    pub change_utc_24h: String,
    /// 24-hour change, 0.01 means 1%
    pub change24h: String,
}

impl RestClient {
    /// Get ticker information
    ///
    /// Returns the latest ticker information for a symbol or all symbols.
    ///
    /// [docs]: https://www.bitget.com/api-doc/spot/market/Get-Ticker-Information
    ///
    /// Rate limit: see official docs
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// The ticker information
    pub async fn get_ticker(
        &self,
        request: &GetTickerRequest,
    ) -> Result<RestResponse<Vec<Ticker>>, ApiError> {
        let endpoint = TICKER_ENDPOINT;

        let mut params = HashMap::new();
        if let Some(symbol) = &request.symbol {
            params.insert("symbol".to_string(), symbol.clone());
        }

        let params = if params.is_empty() {
            None
        } else {
            Some(params)
        };

        self.get(endpoint, params).await
    }
}
