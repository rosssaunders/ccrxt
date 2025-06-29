use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

use super::RestClient;

/// Request for getting 24hr ticker statistics for a symbol
#[derive(Debug, Clone, Serialize)]
pub struct GetTickerRequest {
    /// Trading symbol (e.g., "BTC-USDT")
    pub symbol: String,
}

/// Request for getting all tickers
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAllTickersRequest {}

/// 24hr ticker statistics
#[derive(Debug, Clone, Deserialize)]
pub struct TickerStatistics {
    /// Symbol name
    pub symbol: String,
    /// Symbol name for display (optional as it might not always be present)
    pub name: Option<String>,
    /// Last traded price
    #[serde(rename = "last")]
    pub last_price: Option<String>,
    /// 24hr change percentage (optional as it might not always be present)
    #[serde(rename = "changePercentage")]
    pub change_percentage: Option<String>,
    /// 24hr change amount (optional as it might not always be present) 
    #[serde(rename = "changePrice")]
    pub change_price: Option<String>,
    /// 24hr high price
    pub high: Option<String>,
    /// 24hr low price
    pub low: Option<String>,
    /// 24hr volume in base currency
    pub vol: Option<String>,
    /// 24hr volume in quote currency
    #[serde(rename = "volValue")]
    pub vol_value: Option<String>,
    /// Last trade size (optional)
    #[serde(rename = "size")]
    pub last_size: Option<String>,
    /// Timestamp of the statistics
    pub time: Option<i64>,
}

/// All tickers response wrapper
#[derive(Debug, Clone, Deserialize)]
pub struct AllTickersResponse {
    /// Server timestamp
    pub time: i64,
    /// List of all ticker statistics
    pub ticker: Vec<TickerStatistics>,
}

impl RestClient {
    /// Get 24hr ticker statistics for a specific symbol
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::public::{RestClient, GetTickerRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_default();
    ///     let request = GetTickerRequest {
    ///         symbol: "BTC-USDT".to_string(),
    ///     };
    ///     let (response, _headers) = client.get_ticker(request).await?;
    ///     println!("Last price: {}", response.last_price);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_ticker(
        &self,
        request: GetTickerRequest,
    ) -> Result<(TickerStatistics, ResponseHeaders)> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol);

        let (response, headers): (RestResponse<TickerStatistics>, ResponseHeaders) =
            self.get("/api/v1/market/orderbook/level1", Some(params)).await?;

        Ok((response.data, headers))
    }

    /// Get 24hr ticker statistics for all symbols
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::public::{RestClient, GetAllTickersRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_default();
    ///     let request = GetAllTickersRequest::default();
    ///     let (response, _headers) = client.get_all_tickers(request).await?;
    ///     println!("Found {} tickers", response.ticker.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_all_tickers(
        &self,
        _request: GetAllTickersRequest,
    ) -> Result<(AllTickersResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<AllTickersResponse>, ResponseHeaders) =
            self.get("/api/v1/market/allTickers", None).await?;

        Ok((response.data, headers))
    }
}
