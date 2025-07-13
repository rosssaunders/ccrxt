use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const INNER_TRANSFERS_ENDPOINT: &str = "/api/v1/accounts/transferable";

/// Request for getting inner transfer history
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetInnerTransfersRequest {
    /// Currency filter (optional)
    pub currency: Option<String>,

    /// Transfer from account type filter (optional)
    pub from: Option<String>,

    /// Transfer to account type filter (optional)
    pub to: Option<String>,

    /// Order ID filter (optional)
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,

    /// Start time filter (optional, milliseconds)
    #[serde(rename = "startAt")]
    pub start_time: Option<i64>,

    /// End time filter (optional, milliseconds)
    #[serde(rename = "endAt")]
    pub end_time: Option<i64>,
}

/// Inner transfer record
#[derive(Debug, Clone, Deserialize)]
pub struct InnerTransfer {
    /// Transfer order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Currency
    pub currency: String,

    /// Transfer amount
    pub amount: String,

    /// Transfer from account type
    pub from: String,

    /// Transfer to account type
    pub to: String,

    /// Transfer from account ID
    #[serde(rename = "fromAccountId")]
    pub from_account_id: Option<String>,

    /// Transfer to account ID
    #[serde(rename = "toAccountId")]
    pub to_account_id: Option<String>,

    /// Transfer status
    pub status: String,

    /// Creation time
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

/// Response wrapper for inner transfers
#[derive(Debug, Clone, Deserialize)]
pub struct InnerTransfersResponse {
    /// Current page
    #[serde(rename = "currentPage")]
    pub current_page: i32,

    /// Page size
    #[serde(rename = "pageSize")]
    pub page_size: i32,

    /// Total number of records
    #[serde(rename = "totalNum")]
    pub total_num: i32,

    /// Total pages
    #[serde(rename = "totalPage")]
    pub total_page: i32,

    /// Transfer items
    pub items: Vec<InnerTransfer>,
}

impl RestClient {
    /// Get inner transfer history
    ///
    /// Reference: https://docs.kucoin.com/#get-inner-transfer-records
    pub async fn get_inner_transfers(
        &self,
        request: GetInnerTransfersRequest,
    ) -> Result<(InnerTransfersResponse, ResponseHeaders)> {
        let mut params = HashMap::new();

        if let Some(currency) = request.currency {
            params.insert("currency".to_string(), currency);
        }
        if let Some(from) = request.from {
            params.insert("from".to_string(), from);
        }
        if let Some(to) = request.to {
            params.insert("to".to_string(), to);
        }
        if let Some(order_id) = request.order_id {
            params.insert("orderId".to_string(), order_id);
        }
        if let Some(start_time) = request.start_time {
            params.insert("startAt".to_string(), start_time.to_string());
        }
        if let Some(end_time) = request.end_time {
            params.insert("endAt".to_string(), end_time.to_string());
        }

        let (response, headers): (RestResponse<InnerTransfersResponse>, ResponseHeaders) =
            self.get(INNER_TRANSFERS_ENDPOINT, Some(params)).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner_transfers_request_default() {
        let request = GetInnerTransfersRequest::default();
        assert!(request.currency.is_none());
        assert!(request.from.is_none());
        assert!(request.to.is_none());
    }

    #[test]
    fn test_inner_transfers_request_creation() {
        let request = GetInnerTransfersRequest {
            currency: Some("USDT".to_string()),
            from: Some("main".to_string()),
            to: Some("trade".to_string()),
            ..Default::default()
        };
        assert_eq!(request.currency, Some("USDT".to_string()));
        assert_eq!(request.from, Some("main".to_string()));
        assert_eq!(request.to, Some("trade".to_string()));
    }
}
