use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::bitget::{ApiError, RestResponse, OrderSide};
use super::RestClient;

/// Request for getting recent trades
#[derive(Debug, Clone)]
pub struct GetRecentTradesRequest {
    /// Trading pair
    pub symbol: String,
    /// Number of queries
    pub limit: Option<u32>,
}

impl GetRecentTradesRequest {
    /// Create a new request
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            limit: None,
        }
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
        
        if let Some(limit) = self.limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        params
    }
}

/// Recent trade information
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentTrade {
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
    /// Get recent trades
    /// 
    /// # Arguments
    /// * `request` - The request parameters
    /// 
    /// # Returns
    /// * `Result<RestResponse<Vec<RecentTrade>>, ApiError>` - The recent trade information
    /// 
    /// # Example
    /// ```rust
    /// use venues::bitget::public::rest::{RestClient, GetRecentTradesRequest};
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RestClient::new("https://api.bitget.com", Default::default(), reqwest::Client::new());
    /// 
    /// let response = client.get_recent_trades(
    ///     GetRecentTradesRequest::new("BTCUSDT")
    ///         .limit(100)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_recent_trades(&self, request: GetRecentTradesRequest) -> Result<RestResponse<Vec<RecentTrade>>, ApiError> {
        let endpoint = "/api/v2/spot/market/fills";
        let params = Some(request.to_params());
        self.get(endpoint, params).await
    }
}
