use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const REDEEM_ORDERS_ENDPOINT: &str = "/api/v3/redeem/orders";

/// Redeem order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RedeemOrderStatus {
    /// Completed
    Done,
    /// Settling
    Pending,
}

/// Request for getting redeem orders
#[derive(Debug, Clone, Serialize)]
pub struct GetRedeemOrdersRequest {
    /// Order status (required)
    pub status: RedeemOrderStatus,
    /// Currency (optional)
    pub currency: Option<String>,
    /// Redeem order ID (optional)
    #[serde(rename = "redeemOrderNo")]
    pub redeem_order_no: Option<String>,
    /// Current page; default is 1
    #[serde(rename = "currentPage")]
    pub current_page: Option<i32>,
    /// Page size; 1<=pageSize<=50; default is 50
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
}

/// Redeem order item
#[derive(Debug, Clone, Deserialize)]
pub struct RedeemOrder {
    /// Currency
    pub currency: String,
    /// Purchase order ID
    #[serde(rename = "purchaseOrderNo")]
    pub purchase_order_no: String,
    /// Redeem order ID
    #[serde(rename = "redeemOrderNo")]
    pub redeem_order_no: String,
    /// Redemption size
    #[serde(rename = "redeemSize")]
    pub redeem_size: String,
    /// Redeemed size
    #[serde(rename = "receiptSize")]
    pub receipt_size: String,
    /// Time of redeem
    #[serde(rename = "applyTime")]
    pub apply_time: String,
    /// Status
    pub status: String,
}

/// Paginated response for redeem orders
#[derive(Debug, Clone, Deserialize)]
pub struct RedeemOrdersResponse {
    /// Current page
    #[serde(rename = "currentPage")]
    pub current_page: i32,
    /// Page size
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    /// Total number
    #[serde(rename = "totalNum")]
    pub total_num: i32,
    /// Total pages
    #[serde(rename = "totalPage")]
    pub total_page: i32,
    /// Redeem order items
    pub items: Vec<RedeemOrder>,
}

impl RestClient {
    /// Get redeem orders with pagination
    ///
    /// Reference: https://docs.kucoin.com/margin-credit#get-redeem-orders
    pub async fn get_redeem_orders(
        &self,
        request: GetRedeemOrdersRequest,
    ) -> Result<(RedeemOrdersResponse, ResponseHeaders)> {
        let mut params = std::collections::HashMap::new();
        params.insert(
            "status".to_string(),
            format!("{:?}", request.status).to_uppercase(),
        );

        if let Some(currency) = &request.currency {
            params.insert("currency".to_string(), currency.clone());
        }
        if let Some(redeem_order_no) = &request.redeem_order_no {
            params.insert("redeemOrderNo".to_string(), redeem_order_no.clone());
        }
        if let Some(current_page) = &request.current_page {
            params.insert("currentPage".to_string(), current_page.to_string());
        }
        if let Some(page_size) = &request.page_size {
            params.insert("pageSize".to_string(), page_size.to_string());
        }

        let (response, headers): (RestResponse<RedeemOrdersResponse>, ResponseHeaders) =
            self.get(REDEEM_ORDERS_ENDPOINT, Some(params)).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redeem_order_status_serialization() {
        assert_eq!(
            serde_json::to_string(&RedeemOrderStatus::Done).unwrap(),
            "\"DONE\""
        );
        assert_eq!(
            serde_json::to_string(&RedeemOrderStatus::Pending).unwrap(),
            "\"PENDING\""
        );
    }
}
