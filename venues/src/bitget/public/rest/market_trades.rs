use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::bitget::{ApiError, RestResponse, OrderSide};
use super::RestClient;

/// Request for getting market trades history
#[derive(Debug, Clone)]
pub struct GetMarketTradesRequest {
    /// Trading pair
    pub symbol: String,
    /// Start time Unix millisecond timestamp
    pub start_time: Option<u64>,
    /// End time Unix millisecond timestamp
    pub end_time: Option<u64>,
    /// Number of queries
    pub limit: Option<u32>,
}

impl GetMarketTradesRequest {
    /// Create a new request
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    /// Set start time
    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Set end time
    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Set limit
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Convert to query parameters
    pub fn to_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), self.symbol.clone());
        
        if let Some(start_time) = self.start_time {
            params.insert("startTime".to_string(), start_time.to_string());
        }
        
        if let Some(end_time) = self.end_time {
            params.insert("endTime".to_string(), end_time.to_string());
        }
        
        if let Some(limit) = self.limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        params
    }
}

/// Market trade information
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketTrade {
    /// Trading pair symbol
    pub symbol: String,
    /// Trade ID
    pub trade_id: String,
    /// Trade side
    pub side: OrderSide,
    /// Trade price
    pub price: String,
    /// Trade size
    pub size: String,
    /// Trade timestamp
    pub ts: String,
}

impl RestClient {
    /// Get market trades history
    /// 
    /// # Arguments
    /// * `request` - The request parameters
    /// 
    /// # Returns
    /// * `Result<RestResponse<Vec<MarketTrade>>, ApiError>` - The market trade information
    /// 
    /// # Example
    /// ```rust
    /// use venues::bitget::public::rest::{RestClient, GetMarketTradesRequest};
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RestClient::new("https://api.bitget.com", Default::default(), reqwest::Client::new());
    /// 
    /// let response = client.get_market_trades(
    ///     GetMarketTradesRequest::new("BTCUSDT")
    ///         .start_time(1678965010861)
    ///         .end_time(1678965910861)
    ///         .limit(20)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_market_trades(&self, request: GetMarketTradesRequest) -> Result<RestResponse<Vec<MarketTrade>>, ApiError> {
        let endpoint = "/api/v2/spot/market/fills-history";
        let params = Some(request.to_params());
        self.get(endpoint, params).await
    }
}
