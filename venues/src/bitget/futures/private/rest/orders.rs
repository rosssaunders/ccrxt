use crate::bitget::enums::{ProductType, OrderSide, TimeInForce, OrderType, HoldSide, MarginCoin, TriggerType, PlanType, PlanStatus};
use rest::Client;
use serde::{Deserialize, Serialize};

/// Batch Order
/// 
/// Frequency limit: 10 times/1s (uid)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderRequest {
    /// Trading pair
    pub symbol: String,
    /// Product type
    pub product_type: ProductType,
    /// Margin coin
    pub margin_coin: MarginCoin,
    /// Order list (max 20 orders)
    pub order_list: Vec<BatchOrderItem>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderItem {
    /// Order quantity
    pub size: String,
    /// Order side
    pub side: OrderSide,
    /// Order type
    pub order_type: OrderType,
    /// Order price (required for limit orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Time in force
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// Client order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
    /// Reduce only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderResponse {
    /// Success list
    pub success_list: Vec<BatchOrderResult>,
    /// Failure list
    pub failure_list: Vec<BatchOrderFailure>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderResult {
    /// Order ID
    pub order_id: String,
    /// Client order ID
    pub client_oid: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderFailure {
    /// Client order ID
    pub client_oid: String,
    /// Error code
    pub error_code: String,
    /// Error message
    pub error_msg: String,
}

/// Get Order Detail
/// 
/// Rate limit: 10 times/1s (uid)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetailRequest {
    /// Trading pair
    pub symbol: String,
    /// Product type
    pub product_type: ProductType,
    /// Order ID (use either orderId or clientOid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Client order ID (use either orderId or clientOid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetailData {
    /// User ID
    pub user_id: String,
    /// Trading pair
    pub symbol: String,
    /// Order ID
    pub order_id: String,
    /// Client order ID
    pub client_oid: String,
    /// Order price
    pub price: String,
    /// Order quantity
    pub size: String,
    /// Order type
    pub order_type: OrderType,
    /// Order side
    pub side: OrderSide,
    /// Order status
    pub status: String,
    /// Base coin
    pub base_coin: String,
    /// Quote coin
    pub quote_coin: String,
    /// Reduce only
    pub reduce_only: bool,
    /// Margin coin
    pub margin_coin: MarginCoin,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Filled quantity
    pub base_volume: String,
    /// Filled value
    pub quote_volume: String,
    /// Enter point source
    pub enter_point_source: String,
    /// Order source
    pub order_source: String,
    /// Position side
    pub pos_side: HoldSide,
    /// Margin mode
    pub margin_mode: String,
    /// Leverage
    pub leverage: String,
    /// Creation time
    pub c_time: String,
    /// Update time
    pub u_time: String,
}

/// Get Pending Orders
/// 
/// Rate limit: 10 times/1s (uid)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingOrdersRequest {
    /// Trading pair
    pub symbol: String,
    /// Product type
    pub product_type: ProductType,
    /// Page size (default 20, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// Pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_less_than: Option<String>,
}

pub async fn batch_order(
    client: &Client,
    params: &BatchOrderRequest,
) -> Result<BatchOrderResponse, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/order/batch-orders";
    client.post_signed(endpoint, Some(params)).await.map_err(Into::into)
}

pub async fn order_detail(
    client: &Client,
    params: &OrderDetailRequest,
) -> Result<OrderDetailData, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/order/detail";
    client.get_signed(endpoint, Some(params)).await.map_err(Into::into)
}

pub async fn pending_orders(
    client: &Client,
    params: &PendingOrdersRequest,
) -> Result<Vec<OrderDetailData>, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/order/orders-pending";
    client.get_signed(endpoint, Some(params)).await.map_err(Into::into)
}
