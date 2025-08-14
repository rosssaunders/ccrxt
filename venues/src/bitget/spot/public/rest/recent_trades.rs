use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::{ApiError, OrderSide, RestResponse};

/// Endpoint for getting recent trades
const RECENT_TRADES_ENDPOINT: &str = "/api/v2/spot/market/fills";

/// Request for getting recent trades
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRecentTradesRequest {
    /// Trading pair
    pub symbol: String,
    /// Number of queries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
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
    /// Returns recent trades for a symbol.
    ///
    /// [docs](https://www.bitget.com/api-doc/spot/market/Get-Recent-Trades)
    ///
    /// Rate limit: see official docs
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
        let endpoint = RECENT_TRADES_ENDPOINT;

        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol.clone());

        if let Some(limit) = request.limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        self.get(endpoint, Some(params)).await
    }
}
