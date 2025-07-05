use serde::Serialize;
use std::collections::HashMap;

use super::RestClient;
use crate::bitget::{ApiError, CandlestickGranularity, RestResponse};

/// Request for getting candlestick data
#[derive(Debug, Clone, Serialize)]
pub struct GetCandlestickRequest {
    /// Trading pair
    pub symbol: String,
    /// Time interval
    pub granularity: CandlestickGranularity,
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

impl GetCandlestickRequest {
    /// Convert to query parameters
    pub fn to_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), self.symbol.clone());
        params.insert("granularity".to_string(), self.granularity.to_string());

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

/// Candlestick data - array format: [timestamp, open, high, low, close, base_volume, usdt_volume, quote_volume]
pub type Candlestick = [String; 8];

impl RestClient {
    /// Get candlestick data
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// The candlestick data
    pub async fn get_candlestick(
        &self,
        request: &GetCandlestickRequest,
    ) -> Result<RestResponse<Vec<Candlestick>>, ApiError> {
        let endpoint = "/api/v2/spot/market/candles";
        let params = Some(request.to_params());
        self.get(endpoint, params).await
    }
}
