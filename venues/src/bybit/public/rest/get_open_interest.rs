use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

const OPEN_INTEREST_ENDPOINT: &str = "/v5/market/open-interest";

/// Request parameters for getting open interest
#[derive(Debug, Clone, Serialize)]
pub struct GetOpenInterestRequest {
    /// Product type
    pub category: Category,
    
    /// Symbol name (e.g., "BTCUSDT")
    pub symbol: String,
    
    /// Interval time: 5min, 15min, 30min, 1h, 4h, 1d
    #[serde(rename = "intervalTime")]
    pub interval_time: String,
    
    /// Start timestamp in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    
    /// End timestamp in milliseconds. Default: current time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    
    /// Limit for data size per page. [1, 200]. Default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Open interest information
#[derive(Debug, Clone, Deserialize)]
pub struct OpenInterestInfo {
    /// Open interest amount
    #[serde(rename = "openInterest")]
    pub open_interest: String,
    
    /// Timestamp in milliseconds
    pub timestamp: String,
}

/// Open interest data container
#[derive(Debug, Clone, Deserialize)]
pub struct GetOpenInterestData {
    /// Product type
    pub category: Category,
    
    /// Symbol name
    pub symbol: String,
    
    /// Array of open interest data
    pub list: Vec<OpenInterestInfo>,
    
    /// Cursor for pagination
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}

/// Response from the open interest endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetOpenInterestResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    
    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    
    /// Business data result
    pub result: GetOpenInterestData,
    
    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    
    /// Current timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Get open interest
    ///
    /// Query for historical open interest data of a symbol.
    ///
    /// [API Documentation](https://bybit-exchange.github.io/docs/v5/market/open-interest)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The open interest request parameters including:
    ///   - `category`: Product type
    ///   - `symbol`: Symbol name
    ///   - `interval_time`: Time interval
    ///   - `start_time`: Optional start timestamp
    ///   - `end_time`: Optional end timestamp
    ///   - `limit`: Optional result limit
    ///   - `cursor`: Optional pagination cursor
    ///
    /// # Returns
    /// A result containing the open interest response with historical open interest data or an error
    pub async fn get_open_interest(
        &self,
        request: GetOpenInterestRequest,
    ) -> RestResult<GetOpenInterestResponse> {
        self.send_public_request(OPEN_INTEREST_ENDPOINT, Some(&request), EndpointType::Market)
            .await
    }
}
