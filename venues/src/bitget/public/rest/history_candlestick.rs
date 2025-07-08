use std::collections::HashMap;

use serde::Serialize;

use super::RestClient;
use crate::bitget::{ApiError, CandlestickGranularity, RestResponse};

/// Endpoint for getting historical candlestick data
const HISTORY_CANDLESTICK_ENDPOINT: &str = "/api/v2/spot/market/history-candles";

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
        let endpoint = HISTORY_CANDLESTICK_ENDPOINT;

        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol.clone());
        params.insert("granularity".to_string(), request.granularity.to_string());
        params.insert("endTime".to_string(), request.end_time.to_string());

        if let Some(limit) = request.limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        self.get(endpoint, Some(params)).await
    }
}
