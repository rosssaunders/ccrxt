use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

const LONG_SHORT_RATIO_ENDPOINT: &str = "/v5/market/account-ratio";

/// Request parameters for getting long/short ratio
#[derive(Debug, Clone, Serialize)]
pub struct GetLongShortRatioRequest {
    /// Product type (Linear or Inverse)
    pub category: Category,

    /// Symbol name (e.g., "BTCUSDT")
    pub symbol: String,

    /// Data recording period: 5min, 15min, 30min, 1h, 4h, 1d
    pub period: String,

    /// Limit for data size per page. [1, 500]. Default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Long/short ratio information
#[derive(Debug, Clone, Deserialize)]
pub struct LongShortRatioInfo {
    /// Symbol name
    pub symbol: String,

    /// Buy ratio
    #[serde(rename = "buyRatio")]
    pub buy_ratio: String,

    /// Sell ratio
    #[serde(rename = "sellRatio")]
    pub sell_ratio: String,

    /// Timestamp in milliseconds
    pub timestamp: String,
}

/// Long/short ratio data container
#[derive(Debug, Clone, Deserialize)]
pub struct GetLongShortRatioData {
    /// Product type
    pub category: Category,

    /// Array of long/short ratio data
    pub list: Vec<LongShortRatioInfo>,
}

/// Response from the long/short ratio endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetLongShortRatioResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Business data result
    pub result: GetLongShortRatioData,

    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Current timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Get long/short ratio
    ///
    /// Query for account long/short ratio. This endpoint is only available for USDT perpetual
    /// and Inverse contracts.
    ///
    /// [docs](https://bybit-exchange.github.io/docs/v5/market/long-short-ratio)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The long/short ratio request parameters including:
    ///   - `category`: Product type (Linear or Inverse)
    ///   - `symbol`: Symbol name
    ///   - `period`: Data recording period
    ///   - `limit`: Optional result limit
    ///
    /// # Returns
    /// A result containing the long/short ratio response with buy/sell ratio data or an error
    pub async fn get_long_short_ratio(
        &self,
        request: GetLongShortRatioRequest,
    ) -> RestResult<GetLongShortRatioResponse> {
        self.send_public_request(
            LONG_SHORT_RATIO_ENDPOINT,
            Some(&request),
            EndpointType::Market,
        )
        .await
    }
}
