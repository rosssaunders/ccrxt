use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

const HISTORICAL_VOLATILITY_ENDPOINT: &str = "/v5/market/historical-volatility";

/// Request parameters for getting historical volatility
#[derive(Debug, Clone, Serialize)]
pub struct GetHistoricalVolatilityRequest {
    /// Product type (Option only)
    pub category: Category,
    
    /// Base coin (e.g., "BTC", "ETH")
    #[serde(rename = "baseCoin")]
    pub base_coin: String,
    
    /// Period: 7, 14, 21, 30, 60, 90, 180, 270
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    
    /// Start timestamp in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    
    /// End timestamp in milliseconds. Default: current time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
}

/// Historical volatility information for a specific period
#[derive(Debug, Clone, Deserialize)]
pub struct VolatilityInfo {
    /// Volatility period
    pub period: i32,
    
    /// Volatility value
    pub value: String,
    
    /// Timestamp in milliseconds
    pub time: String,
}

/// Historical volatility data container
#[derive(Debug, Clone, Deserialize)]
pub struct GetHistoricalVolatilityData {
    /// Product type
    pub category: Category,
    
    /// Array of volatility data
    pub list: Vec<VolatilityInfo>,
}

/// Response from the historical volatility endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetHistoricalVolatilityResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    
    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    
    /// Business data result
    pub result: GetHistoricalVolatilityData,
    
    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    
    /// Current timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Get historical volatility
    ///
    /// Query for historical volatility data. This endpoint is only available for Option contracts.
    ///
    /// [API Documentation](https://bybit-exchange.github.io/docs/v5/market/iv)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The historical volatility request parameters including:
    ///   - `category`: Product type (must be Option)
    ///   - `base_coin`: Base coin name
    ///   - `period`: Optional volatility period (7, 14, 21, 30, 60, 90, 180, 270)
    ///   - `start_time`: Optional start timestamp
    ///   - `end_time`: Optional end timestamp
    ///
    /// # Returns
    /// A result containing the historical volatility response with volatility data or an error
    pub async fn get_historical_volatility(
        &self,
        request: GetHistoricalVolatilityRequest,
    ) -> RestResult<GetHistoricalVolatilityResponse> {
        self.send_public_request(
            HISTORICAL_VOLATILITY_ENDPOINT,
            Some(&request),
            EndpointType::Market,
        )
        .await
    }
}
