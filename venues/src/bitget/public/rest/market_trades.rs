use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::{ApiError, OrderSide, RestResponse};

/// Endpoint for getting market trade history
const MARKET_TRADES_ENDPOINT: &str = "/api/v2/spot/market/fills-history";

/// Request for getting market trades history
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMarketTradesRequest {
    /// Trading pair
    pub symbol: String,
    /// Start time Unix millisecond timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time Unix millisecond timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Number of queries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
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
    /// The market trade information
    pub async fn get_market_trades(
        &self,
        request: &GetMarketTradesRequest,
    ) -> Result<RestResponse<Vec<MarketTrade>>, ApiError> {
        let endpoint = MARKET_TRADES_ENDPOINT;

        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol.clone());

        if let Some(start_time) = request.start_time {
            params.insert("startTime".to_string(), start_time.to_string());
        }

        if let Some(end_time) = request.end_time {
            params.insert("endTime".to_string(), end_time.to_string());
        }

        if let Some(limit) = request.limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        self.get(endpoint, Some(params)).await
    }
}
