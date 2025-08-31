use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::bitget::{ApiError, DepthType, PublicRestClient as RestClient, RestResponse};

/// Endpoint for getting orderbook data
const ORDERBOOK_ENDPOINT: &str = "/api/v2/spot/market/orderbook";

/// Request for getting orderbook depth
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetOrderbookRequest {
    /// Trading pair
    pub symbol: String,
    /// Depth type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_type: Option<DepthType>,
    /// Number of queries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
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
    /// Returns the current orderbook depth for a symbol.
    ///
    /// [docs](https://www.bitget.com/api-doc/spot/market/Get-OrderBook-Depth)
    ///
    /// Rate limit: see official docs
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// The orderbook information
    pub async fn get_orderbook(
        &self,
        request: &GetOrderbookRequest,
    ) -> Result<RestResponse<Orderbook>, ApiError> {
        let endpoint = ORDERBOOK_ENDPOINT;

        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol.clone());

        if let Some(depth_type) = &request.depth_type {
            params.insert("type".to_string(), depth_type.to_string());
        }

        if let Some(limit) = request.limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        self.get(endpoint, Some(params)).await
    }
}
