use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::RestClient;
use crate::bitget::{ApiError, OrderSide, RestResponse};

/// Request for getting recent trades
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRecentTradesRequest {
    /// Trading pair
    pub symbol: String,
    /// Number of queries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl GetRecentTradesRequest {
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
    /// The recent trade information
    pub async fn get_recent_trades(
        &self,
        request: &GetRecentTradesRequest,
    ) -> Result<RestResponse<Vec<RecentTrade>>, ApiError> {
        let endpoint = "/api/v2/spot/market/fills";
        let params = Some(request.to_params());
        self.get(endpoint, params).await
    }
}
