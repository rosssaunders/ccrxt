use std::collections::HashMap;

use serde::Serialize;

use super::RestClient;
use crate::bitget::{ApiError, CandlestickGranularity, RestResponse};

/// Endpoint for getting candlestick data
const CANDLESTICK_ENDPOINT: &str = "/api/v2/spot/market/candles";

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

/// Candlestick data - array format: [timestamp, open, high, low, close, base_volume, usdt_volume, quote_volume]
pub type Candlestick = [String; 8];

impl RestClient {
    /// Get candlestick data
    ///
    /// Returns candlestick (kline) data for a symbol and interval.
    ///
    /// [Bitget API Docs - Get Candle Data](https://www.bitget.com/api-doc/spot/market/Get-Candle-Data)
    ///
    /// Rate limit: see official docs
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
        let endpoint = CANDLESTICK_ENDPOINT;

        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol.clone());
        params.insert("granularity".to_string(), request.granularity.to_string());

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
