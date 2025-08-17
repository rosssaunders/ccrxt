use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const PURCHASE_ORDERS_ENDPOINT: &str = "/api/v3/purchase/orders";

/// Purchase order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PurchaseOrderStatus {
    /// Completed
    Done,
    /// Settling
    Pending,
}

/// Request for getting purchase orders
#[derive(Debug, Clone, Serialize)]
pub struct GetPurchaseOrdersRequest {
    /// Order status (required)
    pub status: PurchaseOrderStatus,

    /// Currency (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Purchase order ID (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "purchaseOrderNo")]
    pub purchase_order_no: Option<String>,

    /// Current page; default is 1
    #[serde(skip_serializing_if = "Option::is_none", rename = "currentPage")]
    pub current_page: Option<i32>,

    /// Page size; 1<=pageSize<=50; default is 50
    #[serde(skip_serializing_if = "Option::is_none", rename = "pageSize")]
    pub page_size: Option<i32>,
}

/// Purchase order item
#[derive(Debug, Clone, Deserialize)]
pub struct PurchaseOrder {
    /// Currency
    pub currency: String,

    /// Purchase order ID
    #[serde(rename = "purchaseOrderNo")]
    pub purchase_order_no: String,

    /// Purchase amount
    #[serde(rename = "purchaseSize")]
    pub purchase_size: String,

    /// Matched amount
    #[serde(rename = "matchSize")]
    pub match_size: String,

    /// Redemption size
    #[serde(rename = "redeemSize")]
    pub redeem_size: String,

    /// Purchase interest rate
    #[serde(rename = "interestRate")]
    pub interest_rate: String,

    /// Incomes(Interest)
    #[serde(rename = "incomeSize")]
    pub income_size: String,

    /// Time of purchase
    #[serde(rename = "applyTime")]
    pub apply_time: String,

    /// Status
    pub status: String,
}

/// Paginated response for purchase orders
#[derive(Debug, Clone, Deserialize)]
pub struct PurchaseOrdersResponse {
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

    /// Purchase order items
    pub items: Vec<PurchaseOrder>,
}

impl RestClient {
    /// Get purchase orders with pagination
    ///
    /// [docs](https://docs.kucoin.com/margin-credit#get-purchase-orders)
    pub async fn get_purchase_orders(
        &self,
        request: GetPurchaseOrdersRequest,
    ) -> Result<(PurchaseOrdersResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<PurchaseOrdersResponse>, ResponseHeaders) = self
            .get_with_request(PURCHASE_ORDERS_ENDPOINT, &request)
            .await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purchase_order_status_serialization() {
        assert_eq!(
            serde_json::to_string(&PurchaseOrderStatus::Done).unwrap(),
            "\"DONE\""
        );
        assert_eq!(
            serde_json::to_string(&PurchaseOrderStatus::Pending).unwrap(),
            "\"PENDING\""
        );
    }
}
