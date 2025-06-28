use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::bitget::{ApiError, RestResponse};
use super::RestClient;

/// Request for getting ticker information
#[derive(Debug, Clone, Default)]
pub struct GetTickerRequest {
    /// Specific symbol to query, if empty returns all symbols
    pub symbol: Option<String>,
}

impl GetTickerRequest {
    /// Create a new request for all symbols
    pub fn new() -> Self {
        Self::default()
    }

    /// Set specific symbol to query
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Convert to query parameters
    pub fn to_params(&self) -> Option<HashMap<String, String>> {
        let mut params = HashMap::new();
        
        if let Some(symbol) = &self.symbol {
            params.insert("symbol".to_string(), symbol.clone());
        }

        if params.is_empty() {
            None
        } else {
            Some(params)
        }
    }
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
    pub bid_size: String,
    /// Selling 1 amount
    #[serde(rename = "askSz")]
    pub ask_size: String,
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
    /// # Arguments
    /// * `request` - The request parameters
    /// 
    /// # Returns
    /// * `Result<RestResponse<Vec<Ticker>>, ApiError>` - The ticker information
    /// 
    /// # Example
    /// ```rust
    /// use venues::bitget::public::rest::{RestClient, GetTickerRequest};
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RestClient::new("https://api.bitget.com", Default::default(), reqwest::Client::new());
    /// 
    /// // Get all tickers
    /// let response = client.get_ticker(GetTickerRequest::new()).await?;
    /// 
    /// // Get specific ticker
    /// let response = client.get_ticker(GetTickerRequest::new().symbol("BTCUSDT")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_ticker(&self, request: GetTickerRequest) -> Result<RestResponse<Vec<Ticker>>, ApiError> {
        let endpoint = "/api/v2/spot/market/tickers";
        let params = request.to_params();
        self.get(endpoint, params).await
    }
}
