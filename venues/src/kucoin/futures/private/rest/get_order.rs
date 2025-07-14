use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{
    OrderSide, OrderStatus, OrderType, ResponseHeaders, RestResponse, Result, TimeInForce,
};

/// Endpoint URL for get order (format string)
pub const GET_ORDER_ENDPOINT: &str = "/api/v1/orders/";

#[derive(Debug, Clone, Serialize)]
pub struct GetOrderRequest {
    pub order_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetails {
    pub id: String,
    pub symbol: String,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: OrderSide,
    pub size: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    pub deal_size: String,
    pub deal_funds: String,
    pub fee: String,
    pub fee_currency: String,
    pub stop: Option<String>,
    pub time_in_force: Option<TimeInForce>,
    pub post_only: bool,
    pub hidden: bool,
    pub iceberg: bool,
    pub leverage: String,
    pub force_hold: bool,
    pub close_order: bool,
    pub visible_size: Option<String>,
    pub client_oid: Option<String>,
    pub remark: Option<String>,
    pub tags: Option<String>,
    pub is_active: bool,
    pub cancel_exist_stop: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub end_at: Option<i64>,
    pub order_time: i64,
    pub settle_currency: String,
    pub status: OrderStatus,
    pub filled_value: String,
    pub reduce_only: bool,
}

impl super::RestClient {
    /// Get order details
    pub async fn get_order(
        &self,
        request: GetOrderRequest,
    ) -> Result<(RestResponse<OrderDetails>, ResponseHeaders)> {
        let endpoint = format!("{}{}", GET_ORDER_ENDPOINT, request.order_id);
        self.get(&endpoint, None).await
    }
}
