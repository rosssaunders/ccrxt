use crate::bitget::enums::{ProductType, OrderSide, TriggerType, PlanType, PlanStatus, HoldSide, OrderType, TimeInForce, MarginCoin};
use crate::bitget::private::rest::RestClient as Client;
use serde::{Deserialize, Serialize};

/// Place Trigger/Plan Order
/// 
/// Frequency limit: 10 times/1s (uid)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTriggerOrderRequest {
    /// Trading pair
    pub symbol: String,
    /// Product type
    pub product_type: ProductType,
    /// Margin coin
    pub margin_coin: MarginCoin,
    /// Order quantity
    pub size: String,
    /// Order side
    pub side: OrderSide,
    /// Order type (limit/market)
    pub order_type: OrderType,
    /// Trigger price
    pub trigger_price: String,
    /// Trigger type
    pub trigger_type: TriggerType,
    /// Plan type
    pub plan_type: PlanType,
    /// Hold side
    pub hold_side: HoldSide,
    /// Order price (required for limit orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_price: Option<String>,
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
pub struct PlaceTriggerOrderResponse {
    /// Order ID
    pub order_id: String,
    /// Client order ID
    pub client_oid: String,
}

/// Cancel Trigger/Plan Order
/// 
/// Frequency limit: 10 times/1s (uid)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelTriggerOrderRequest {
    /// Trading pair
    pub symbol: String,
    /// Product type
    pub product_type: ProductType,
    /// Plan type
    pub plan_type: PlanType,
    /// Order ID (use either orderId or clientOid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Client order ID (use either orderId or clientOid)  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
}

/// Get Pending Trigger Orders
/// 
/// Rate limit: 10 times/1s (uid)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingTriggerOrdersRequest {
    /// Product type
    pub product_type: ProductType,
    /// Plan type
    pub plan_type: PlanType,
    /// Trading pair (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Page size (default 20, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// Pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_less_than: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerOrderData {
    /// Order ID
    pub order_id: String,
    /// Client order ID
    pub client_oid: String,
    /// Trading pair
    pub symbol: String,
    /// Margin coin
    pub margin_coin: MarginCoin,
    /// Order quantity
    pub size: String,
    /// Order side
    pub side: OrderSide,
    /// Order type
    pub order_type: OrderType,
    /// Execute price
    pub execute_price: String,
    /// Trigger price
    pub trigger_price: String,
    /// Order status
    pub status: PlanStatus,
    /// Plan type
    pub plan_type: PlanType,
    /// Trigger type
    pub trigger_type: TriggerType,
    /// Hold side
    pub hold_side: HoldSide,
    /// Reduce only
    pub reduce_only: bool,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Creation time
    pub c_time: String,
    /// Update time
    pub u_time: String,
}

pub async fn place_trigger_order(
    client: &Client,
    params: &PlaceTriggerOrderRequest,
) -> Result<PlaceTriggerOrderResponse, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/order/place-plan-order";
    client.post_signed(endpoint, Some(params)).await.map_err(Into::into)
}

pub async fn cancel_trigger_order(
    client: &Client,
    params: &CancelTriggerOrderRequest,
) -> Result<String, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/order/cancel-plan-order";
    client.post_signed(endpoint, Some(params)).await.map_err(Into::into)
}

pub async fn pending_trigger_orders(
    client: &Client,
    params: &PendingTriggerOrdersRequest,
) -> Result<Vec<TriggerOrderData>, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/order/orders-plan-pending";
    client.get_signed(endpoint, Some(params)).await.map_err(Into::into)
}
