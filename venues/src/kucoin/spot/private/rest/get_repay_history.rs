use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result, private_client::RestClient};

const REPAY_HISTORY_ENDPOINT: &str = "/api/v3/margin/repay";

/// Request for getting repay history
#[derive(Debug, Clone, Serialize)]
pub struct GetRepayHistoryRequest {
    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "isIsolated")]
    pub is_isolated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "orderNo")]
    pub order_no: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "endTime")]
    pub end_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "currentPage")]
    pub current_page: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "pageSize")]
    pub page_size: Option<i32>,
}

/// Repay history item
#[derive(Debug, Clone, Deserialize)]
pub struct RepayHistoryItem {
    pub order_no: String,

    pub symbol: String,

    pub currency: String,

    pub size: String,

    pub principal: String,

    pub interest: String,

    pub status: OrderStatus,

    #[serde(rename = "createdTime")]
    pub created_time: i64,
}

/// Response for repay history
#[derive(Debug, Clone, Deserialize)]
pub struct RepayHistoryResponse {
    pub timestamp: i64,

    #[serde(rename = "currentPage")]
    pub current_page: i32,

    #[serde(rename = "pageSize")]
    pub page_size: i32,

    #[serde(rename = "totalNum")]
    pub total_num: i32,

    #[serde(rename = "totalPage")]
    pub total_page: i32,

    pub items: Vec<RepayHistoryItem>,
}

/// Order status for borrow and repay operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILED")]
    Failed,
}

impl RestClient {
    /// Get repay history
    ///
    /// This API endpoint is used to get the repayment orders for cross and isolated margin accounts.
    pub async fn get_repay_history(
        &self,
        request: GetRepayHistoryRequest,
    ) -> Result<(RepayHistoryResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<RepayHistoryResponse>, ResponseHeaders) = self
            .get_with_request(REPAY_HISTORY_ENDPOINT, &request)
            .await?;
        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repay_history_request_creation() {
        let request = GetRepayHistoryRequest {
            currency: "USDT".to_string(),
            is_isolated: Some(false),
            symbol: None,
            order_no: None,
            start_time: Some(1680278400000),
            end_time: Some(1680364800000),
            current_page: Some(1),
            page_size: Some(50),
        };

        assert_eq!(request.currency, "USDT");
        assert_eq!(request.is_isolated, Some(false));
        assert_eq!(request.symbol, None);
        assert_eq!(request.start_time, Some(1680278400000));
        assert_eq!(request.end_time, Some(1680364800000));
        assert_eq!(request.current_page, Some(1));
        assert_eq!(request.page_size, Some(50));
    }
}
