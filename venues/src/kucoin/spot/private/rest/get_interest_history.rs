use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const INTEREST_HISTORY_ENDPOINT: &str = "/api/v3/margin/interest";

/// Request for getting interest history
#[derive(Debug, Clone, Serialize)]
pub struct GetInterestHistoryRequest {
    pub currency: Option<String>,
    pub is_isolated: Option<bool>,
    pub symbol: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub current_page: Option<i32>,
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
        let mut params = HashMap::new();
        if let Some(currency) = request.currency {
            params.insert("currency".to_string(), currency);
        }
        if let Some(is_isolated) = request.is_isolated {
            params.insert("isIsolated".to_string(), is_isolated.to_string());
        }
        if let Some(symbol) = request.symbol {
            params.insert("symbol".to_string(), symbol);
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
        let (response, headers): (RestResponse<InterestHistoryResponse>, ResponseHeaders) =
            self.get(INTEREST_HISTORY_ENDPOINT, Some(params)).await?;
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
