// Removed unused Serialize and Deserialize imports
use std::collections::HashMap;

use super::RestClient;
use crate::bitget::{ApiError, CandlestickGranularity, RestResponse};

/// Request for getting candlestick data
#[derive(Debug, Clone)]
pub struct GetCandlestickRequest {
    /// Trading pair
    pub symbol: String,
    /// Time interval
    pub granularity: CandlestickGranularity,
    /// Start time Unix millisecond timestamp
    pub start_time: Option<u64>,
    /// End time Unix millisecond timestamp
    pub end_time: Option<u64>,
    /// Number of queries
    pub limit: Option<u32>,
}

impl GetCandlestickRequest {
    /// Create a new request
    pub fn new(symbol: impl Into<String>, granularity: CandlestickGranularity) -> Self {
        Self {
            symbol: symbol.into(),
            granularity,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    /// Set start time
    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Set end time
    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
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
    /// * `Result<RestResponse<Vec<Candlestick>>, ApiError>` - The candlestick data
    ///
    /// # Example
    /// ```rust
    /// use venues::bitget::public::rest::{RestClient, GetCandlestickRequest};
    /// use venues::bitget::CandlestickGranularity;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RestClient::new("https://api.bitget.com", Default::default(), reqwest::Client::new());
    ///
    /// let response = client.get_candlestick(
    ///     GetCandlestickRequest::new("BTCUSDT", CandlestickGranularity::OneMinute)
    ///         .limit(100)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_candlestick(
        &self,
        request: GetCandlestickRequest,
    ) -> Result<RestResponse<Vec<Candlestick>>, ApiError> {
        let endpoint = "/api/v2/spot/market/candles";
        let params = Some(request.to_params());
        self.get(endpoint, params).await
    }
}
