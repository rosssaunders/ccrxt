use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::bitget::{ApiError, RestResponse, DepthType};
use super::RestClient;

/// Request for getting orderbook depth
#[derive(Debug, Clone)]
pub struct GetOrderbookRequest {
    /// Trading pair
    pub symbol: String,
    /// Depth type
    pub depth_type: Option<DepthType>,
    /// Number of queries
    pub limit: Option<u32>,
}

impl GetOrderbookRequest {
    /// Create a new request
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            depth_type: None,
            limit: None,
        }
    }

    /// Set depth type
    pub fn depth_type(mut self, depth_type: DepthType) -> Self {
        self.depth_type = Some(depth_type);
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
        
        if let Some(depth_type) = &self.depth_type {
            params.insert("type".to_string(), depth_type.to_string());
        }
        
        if let Some(limit) = self.limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        params
    }
}

/// Orderbook depth information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Orderbook {
    /// Ask depth
    pub asks: Vec<[String; 2]>,
    /// Bid depth
    pub bids: Vec<[String; 2]>,
    /// Matching engine timestamp(ms)
    pub ts: String,
}

impl RestClient {
    /// Get orderbook depth
    /// 
    /// # Arguments
    /// * `request` - The request parameters
    /// 
    /// # Returns
    /// * `Result<RestResponse<Orderbook>, ApiError>` - The orderbook information
    /// 
    /// # Example
    /// ```rust
    /// use venues::bitget::public::rest::{RestClient, GetOrderbookRequest};
    /// use venues::bitget::DepthType;
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RestClient::new("https://api.bitget.com", Default::default(), reqwest::Client::new());
    /// 
    /// let response = client.get_orderbook(
    ///     GetOrderbookRequest::new("BTCUSDT")
    ///         .depth_type(DepthType::Step0)
    ///         .limit(100)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_orderbook(&self, request: GetOrderbookRequest) -> Result<RestResponse<Orderbook>, ApiError> {
        let endpoint = "/api/v2/spot/market/orderbook";
        let params = Some(request.to_params());
        self.get(endpoint, params).await
    }
}
