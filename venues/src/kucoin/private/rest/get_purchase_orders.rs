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
use serde::{Deserialize, Serialize};
use crate::kucoin::{ResponseHeaders, RestResponse, Result};
use super::RestClient;

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
    pub currency: Option<String>,
    /// Purchase order ID (optional)
    #[serde(rename = "purchaseOrderNo")]
    pub purchase_order_no: Option<String>,
    /// Current page; default is 1
    #[serde(rename = "currentPage")]
    pub current_page: Option<i32>,
    /// Page size; 1<=pageSize<=50; default is 50
    #[serde(rename = "pageSize")]
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
    /// Reference: https://docs.kucoin.com/margin-credit#get-purchase-orders
    pub async fn get_purchase_orders(
        &self,
        request: GetPurchaseOrdersRequest,
    ) -> Result<(PurchaseOrdersResponse, ResponseHeaders)> {
        let mut params = std::collections::HashMap::new();
        params.insert(
            "status".to_string(),
            format!("{:?}", request.status).to_uppercase(),
        );

        if let Some(currency) = &request.currency {
            params.insert("currency".to_string(), currency.clone());
        }
        if let Some(purchase_order_no) = &request.purchase_order_no {
            params.insert("purchaseOrderNo".to_string(), purchase_order_no.clone());
        }
        if let Some(current_page) = &request.current_page {
            params.insert("currentPage".to_string(), current_page.to_string());
        }
        if let Some(page_size) = &request.page_size {
            params.insert("pageSize".to_string(), page_size.to_string());
        }

        let (response, headers): (RestResponse<PurchaseOrdersResponse>, ResponseHeaders) =
            self.get("/api/v3/purchase/orders", Some(params)).await?;

        Ok((response.data, headers))
    }
}
