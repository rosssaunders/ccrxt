use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

const FUNDING_HISTORY_ENDPOINT: &str = "/v5/market/funding/history";

/// Request parameters for getting funding rate history
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingHistoryRequest {
    /// Product type (Linear or Inverse)
    pub category: Category,

    /// Symbol name (e.g., "BTCUSDT")
    pub symbol: String,

    /// Start timestamp in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End timestamp in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Limit for data size per page. [1, 200]. Default: 200
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Funding rate information for a single timestamp
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingInfo {
    /// Symbol name
    pub symbol: String,

    /// Funding rate
    pub funding_rate: String,

    /// Funding rate timestamp in milliseconds
    pub funding_rate_timestamp: String,
}

/// Funding history data container
#[derive(Debug, Clone, Deserialize)]
pub struct GetFundingHistoryData {
    /// Product type
    pub category: Category,

    /// Array of funding rate history
    pub list: Vec<FundingInfo>,
}

/// Response from the funding history endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetFundingHistoryResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Business data result
    pub result: GetFundingHistoryData,

    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Current timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Get funding rate history
    ///
    /// Query for historical funding rates. This endpoint is only available for USDT/USDC perpetual
    /// and Inverse perpetual contracts.
    ///
    /// [docs](https://bybit-exchange.github.io/docs/v5/market/funding-rate)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The funding history request parameters including:
    ///   - `category`: Product type (Linear or Inverse)
    ///   - `symbol`: Symbol name
    ///   - `start_time`: Optional start timestamp
    ///   - `end_time`: Optional end timestamp
    ///   - `limit`: Optional result limit
    ///
    /// # Returns
    /// A result containing the funding history response with funding rate data or an error
    pub async fn get_funding_history(
        &self,
        request: GetFundingHistoryRequest,
    ) -> RestResult<GetFundingHistoryResponse> {
        self.send_public_request(
            FUNDING_HISTORY_ENDPOINT,
            Some(&request),
            EndpointType::Market,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_funding_history_request_construction() {
        let request = GetFundingHistoryRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            start_time: Some(1670601600000),
            end_time: None,
            limit: Some(50),
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.start_time, Some(1670601600000));
        assert_eq!(request.limit, Some(50));
        assert!(request.end_time.is_none());
    }
}
