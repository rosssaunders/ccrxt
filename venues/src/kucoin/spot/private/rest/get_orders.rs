use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{OrderSide, OrderStatus, ResponseHeaders, RestResponse, Result};

const ORDERS_ENDPOINT: &str = "/api/v1/orders";

/// Request for getting order list
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetOrdersRequest {
    /// Order status filter (optional): active, done
    pub status: Option<OrderStatus>,

    /// Trading symbol filter (optional)
    pub symbol: Option<String>,

    /// Order side filter (optional)
    pub side: Option<OrderSide>,

    /// Order type filter (optional)
    #[serde(rename = "type")]
    pub order_type: Option<String>,

    /// Start time filter (optional, milliseconds)
    #[serde(rename = "startAt")]
    pub start_time: Option<i64>,

    /// End time filter (optional, milliseconds)
    #[serde(rename = "endAt")]
    pub end_time: Option<i64>,
}

/// Order information
#[derive(Debug, Clone, Deserialize)]
pub struct Order {
    /// Order ID
    pub id: String,

    /// Trading symbol
    pub symbol: String,

    /// Operation type (DEAL)
    #[serde(rename = "opType")]
    pub operation_type: String,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: String,

    /// Order side
    pub side: OrderSide,

    /// Order amount
    pub amount: String,

    /// Order funds
    pub funds: String,

    /// Dealt amount
    #[serde(rename = "dealAmount")]
    pub deal_amount: String,

    /// Dealt funds
    #[serde(rename = "dealFunds")]
    pub deal_funds: String,

    /// Fee
    pub fee: String,

    /// Fee currency
    #[serde(rename = "feeCurrency")]
    pub fee_currency: String,

    /// Self-trade prevention
    pub stp: String,

    /// Stop type
    pub stop: String,

    /// Stop triggered flag
    #[serde(rename = "stopTriggered")]
    pub stop_triggered: bool,

    /// Stop price
    #[serde(rename = "stopPrice")]
    pub stop_price: String,

    /// Time in force
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,

    /// Post only flag
    #[serde(rename = "postOnly")]
    pub post_only: bool,

    /// Hidden order flag
    pub hidden: bool,

    /// Iceberg order flag
    pub iceberg: bool,

    /// Visible size for iceberg orders
    #[serde(rename = "visibleSize")]
    pub visible_size: String,

    /// Cancel after time
    #[serde(rename = "cancelAfter")]
    pub cancel_after: i64,

    /// Channel
    pub channel: String,

    /// Client order ID
    #[serde(rename = "clientOid")]
    pub client_order_id: String,

    /// Remark
    pub remark: String,

    /// Tags
    pub tags: String,

    /// Is active flag
    #[serde(rename = "isActive")]
    pub is_active: bool,

    /// Cancel exist flag
    #[serde(rename = "cancelExist")]
    pub cancel_exist: bool,

    /// Creation time
    #[serde(rename = "createdAt")]
    pub created_at: i64,

    /// Trade type
    #[serde(rename = "tradeType")]
    pub trade_type: String,

    /// Price
    pub price: String,

    /// Size
    pub size: String,
}

/// Response wrapper for order list
#[derive(Debug, Clone, Deserialize)]
pub struct OrdersResponse {
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

    /// Order items
    pub items: Vec<Order>,
}

impl RestClient {
    /// Get list of orders
    ///
    /// Reference: https://docs.kucoin.com/#list-orders
    pub async fn get_orders(
        &self,
        request: GetOrdersRequest,
    ) -> Result<(OrdersResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<OrdersResponse>, ResponseHeaders) =
            self.get_with_request(ORDERS_ENDPOINT, &request).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kucoin::spot::OrderStatus;

    #[test]
    fn test_orders_request_creation() {
        let request = GetOrdersRequest {
            status: Some(OrderStatus::Active),
            symbol: Some("BTC-USDT".to_string()),
            ..Default::default()
        };
        assert_eq!(request.status, Some(OrderStatus::Active));
        assert_eq!(request.symbol, Some("BTC-USDT".to_string()));
    }

    #[test]
    fn test_orders_request_default() {
        let request = GetOrdersRequest::default();
        assert!(request.status.is_none());
        assert!(request.symbol.is_none());
    }
}
