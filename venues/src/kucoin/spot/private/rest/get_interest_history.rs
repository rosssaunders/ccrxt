use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const INTEREST_HISTORY_ENDPOINT: &str = "/api/v3/margin/interest";

/// Request for getting interest history
#[derive(Debug, Clone, Serialize)]
pub struct GetInterestHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "isIsolated")]
    pub is_isolated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "endTime")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "currentPage")]
    pub current_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "pageSize")]
    pub page_size: Option<i32>,
}

/// Interest history item
#[derive(Debug, Clone, Deserialize)]
pub struct InterestHistoryItem {
    pub currency: String,
    #[serde(rename = "dayRatio")]
    pub day_ratio: String,
    #[serde(rename = "interestAmount")]
    pub interest_amount: String,
    #[serde(rename = "createdTime")]
    pub created_time: i64,
}

/// Response for interest history
#[derive(Debug, Clone, Deserialize)]
pub struct InterestHistoryResponse {
    pub timestamp: i64,
    #[serde(rename = "currentPage")]
    pub current_page: i32,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    #[serde(rename = "totalNum")]
    pub total_num: i32,
    #[serde(rename = "totalPage")]
    pub total_page: i32,
    pub items: Vec<InterestHistoryItem>,
}

impl RestClient {
    /// Get interest history
    ///
    /// Request the interest records of the cross/isolated margin lending via this endpoint.
    pub async fn get_interest_history(
        &self,
        request: GetInterestHistoryRequest,
    ) -> Result<(InterestHistoryResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<InterestHistoryResponse>, ResponseHeaders) = self
            .get_with_request(INTEREST_HISTORY_ENDPOINT, &request)
            .await?;
        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_interest_history_request_creation() {
        let request = GetInterestHistoryRequest {
            currency: Some("BTC".to_string()),
            is_isolated: Some(true),
            symbol: Some("BTC-USDT".to_string()),
            start_time: Some(1680278400000),
            end_time: Some(1680364800000),
            current_page: Some(1),
            page_size: Some(50),
        };

        assert_eq!(request.currency, Some("BTC".to_string()));
        assert_eq!(request.is_isolated, Some(true));
        assert_eq!(request.symbol, Some("BTC-USDT".to_string()));
        assert_eq!(request.start_time, Some(1680278400000));
        assert_eq!(request.end_time, Some(1680364800000));
        assert_eq!(request.current_page, Some(1));
        assert_eq!(request.page_size, Some(50));
    }
}
