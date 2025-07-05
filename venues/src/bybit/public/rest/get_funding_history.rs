use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingHistoryRequest {
    pub category: Category,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingInfo {
    pub symbol: String,
    pub funding_rate: String,
    pub funding_rate_timestamp: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetFundingHistoryData {
    pub category: Category,
    pub list: Vec<FundingInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetFundingHistoryResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetFundingHistoryData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Get funding rate history
    ///
    /// Query for historical funding rates. For USDT/USDC contract and Inverse contract only.
    ///
    /// # Arguments
    /// * `request` - The funding history request parameters
    ///
    /// # Returns
    /// A result containing the funding history response or an error
    pub async fn get_funding_history(
        &self,
        request: GetFundingHistoryRequest,
    ) -> RestResult<GetFundingHistoryResponse> {
        self.send_public_request(
            "/v5/market/funding/history",
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