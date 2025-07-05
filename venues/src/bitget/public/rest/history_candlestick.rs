use serde::Serialize;
use std::collections::HashMap;

use super::RestClient;
use crate::bitget::{ApiError, CandlestickGranularity, RestResponse};

/// Request for getting historical candlestick data
#[derive(Debug, Clone, Serialize)]
pub struct GetHistoryCandlestickRequest {
    /// Trading pair
    pub symbol: String,
    /// Time interval
    pub granularity: CandlestickGranularity,
    /// End time Unix millisecond timestamp
    pub end_time: u64,
    /// Number of queries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl GetHistoryCandlestickRequest {
    /// Convert to query parameters
    pub fn to_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), self.symbol.clone());
        params.insert("granularity".to_string(), self.granularity.to_string());
        params.insert("endTime".to_string(), self.end_time.to_string());

        if let Some(limit) = self.limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        params
    }
}

/// Historical candlestick data - array format: [timestamp, open, high, low, close, base_volume, usdt_volume, quote_volume]
pub type HistoryCandlestick = [String; 8];

impl RestClient {
    /// Get historical candlestick data
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// The historical candlestick data
    pub async fn get_history_candlestick(
        &self,
        request: &GetHistoryCandlestickRequest,
    ) -> Result<RestResponse<Vec<HistoryCandlestick>>, ApiError> {
        let endpoint = "/api/v2/spot/market/history-candles";
        let params = Some(request.to_params());
        self.get(endpoint, params).await
    }
}
