// Removed unused Serialize and Deserialize imports
use std::collections::HashMap;

use super::RestClient;
use crate::bitget::{ApiError, CandlestickGranularity, RestResponse};

/// Request for getting historical candlestick data
#[derive(Debug, Clone)]
pub struct GetHistoryCandlestickRequest {
    /// Trading pair
    pub symbol: String,
    /// Time interval
    pub granularity: CandlestickGranularity,
    /// End time Unix millisecond timestamp
    pub end_time: u64,
    /// Number of queries
    pub limit: Option<u32>,
}

impl GetHistoryCandlestickRequest {
    /// Create a new request
    pub fn new(
        symbol: impl Into<String>,
        granularity: CandlestickGranularity,
        end_time: u64,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            granularity,
            end_time,
            limit: None,
        }
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
    /// * `Result<RestResponse<Vec<HistoryCandlestick>>, ApiError>` - The historical candlestick data
    ///
    /// # Example
    /// ```rust
    /// use venues::bitget::public::rest::{RestClient, GetHistoryCandlestickRequest};
    /// use venues::bitget::CandlestickGranularity;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RestClient::new("https://api.bitget.com", Default::default(), reqwest::Client::new());
    ///
    /// let end_time = 1659080270000; // Unix millisecond timestamp
    /// let response = client.get_history_candlestick(
    ///     GetHistoryCandlestickRequest::new("BTCUSDT", CandlestickGranularity::OneMinute, end_time)
    ///         .limit(100)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_history_candlestick(
        &self,
        request: GetHistoryCandlestickRequest,
    ) -> Result<RestResponse<Vec<HistoryCandlestick>>, ApiError> {
        let endpoint = "/api/v2/spot/market/history-candles";
        let params = Some(request.to_params());
        self.get(endpoint, params).await
    }
}
