use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const BORROW_HISTORY_ENDPOINT: &str = "/api/v3/margin/borrow";

/// Request for getting borrow history
#[derive(Debug, Clone, Serialize)]
pub struct GetBorrowHistoryRequest {
    pub currency: String,
    pub is_isolated: Option<bool>,
    pub symbol: Option<String>,
    pub order_no: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub current_page: Option<i32>,
    pub page_size: Option<i32>,
}

/// Borrow history item
#[derive(Debug, Clone, Deserialize)]
pub struct BorrowHistoryItem {
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

/// Response for borrow history
#[derive(Debug, Clone, Deserialize)]
pub struct BorrowHistoryResponse {
    pub timestamp: i64,
    #[serde(rename = "currentPage")]
    pub current_page: i32,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    #[serde(rename = "totalNum")]
    pub total_num: i32,
    #[serde(rename = "totalPage")]
    pub total_page: i32,
    pub items: Vec<BorrowHistoryItem>,
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
    /// Get borrow history
    ///
    /// This API endpoint is used to get the borrowing orders for cross and isolated margin accounts.
    pub async fn get_borrow_history(
        &self,
        request: GetBorrowHistoryRequest,
    ) -> Result<(BorrowHistoryResponse, ResponseHeaders)> {
        let mut params = HashMap::new();
        params.insert("currency".to_string(), request.currency);
        if let Some(is_isolated) = request.is_isolated {
            params.insert("isIsolated".to_string(), is_isolated.to_string());
        }
        if let Some(symbol) = request.symbol {
            params.insert("symbol".to_string(), symbol);
        }
        if let Some(order_no) = request.order_no {
            params.insert("orderNo".to_string(), order_no);
        }
        if let Some(start_time) = request.start_time {
            params.insert("startTime".to_string(), start_time.to_string());
        }
        if let Some(end_time) = request.end_time {
            params.insert("endTime".to_string(), end_time.to_string());
        }
        if let Some(current_page) = request.current_page {
            params.insert("currentPage".to_string(), current_page.to_string());
        }
        if let Some(page_size) = request.page_size {
            params.insert("pageSize".to_string(), page_size.to_string());
        }
        let (response, headers): (RestResponse<BorrowHistoryResponse>, ResponseHeaders) =
            self.get(BORROW_HISTORY_ENDPOINT, Some(params)).await?;
        Ok((response.data, headers))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_borrow_history_request_creation() {
        let request = GetBorrowHistoryRequest {
            currency: "BTC".to_string(),
            is_isolated: Some(false),
            symbol: None,
            order_no: None,
            start_time: Some(1680278400000),
            end_time: Some(1680364800000),
            current_page: Some(1),
            page_size: Some(50),
        };

        assert_eq!(request.currency, "BTC");
        assert_eq!(request.is_isolated, Some(false));
        assert_eq!(request.symbol, None);
        assert_eq!(request.start_time, Some(1680278400000));
        assert_eq!(request.end_time, Some(1680364800000));
        assert_eq!(request.current_page, Some(1));
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_order_status_serialization() {
        assert_eq!(
            serde_json::to_string(&OrderStatus::Pending).unwrap(),
            "\"PENDING\""
        );
        assert_eq!(
            serde_json::to_string(&OrderStatus::Success).unwrap(),
            "\"SUCCESS\""
        );
        assert_eq!(
            serde_json::to_string(&OrderStatus::Failed).unwrap(),
            "\"FAILED\""
        );
    }
}
