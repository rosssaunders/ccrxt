use serde::{Deserialize, Serialize};
use super::{
    errors::BinanceCoinMResult,
    private_rest::BinanceCoinMPrivateRest,
    types::BinanceResponse,
    common::request::send_request,
    enums::{OrderSide, PositionSide, OrderStatus, TimeInForce, OrderType, WorkingType},
};
use super::errors::BinanceCoinMError;

/// Request struct for creating multiple orders in a single request
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOrderRequest {
    /// List of orders to create
    pub orders: Vec<OrderRequest>,
}

/// Individual order request within a batch
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRequest {
    /// Trading pair symbol
    pub symbol: String,
    /// Order side (BUY or SELL)
    pub side: OrderSide,
    /// Order type (LIMIT, MARKET, etc.)
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order quantity
    pub quantity: String,
    /// Order price (required for LIMIT orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Time in force (required for LIMIT orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// Whether to reduce only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Whether to close position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_position: Option<bool>,
    /// Position side (BOTH, LONG, SHORT)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,
    /// Working type (MARK_PRICE, CONTRACT_PRICE)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_type: Option<WorkingType>,
    /// Whether to enable price protection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_protect: Option<bool>,
    /// Client order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,
    /// Stop price for stop orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    /// Activation price for trailing stop orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_price: Option<String>,
    /// Callback rate for trailing stop orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_rate: Option<String>,
}

/// Response struct for batch order creation
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOrderResponse {
    /// List of order responses
    pub orders: Vec<OrderResponse>,
    /// List of order reports
    pub order_reports: Vec<OrderReport>,
}

/// Individual order response within a batch
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    /// Trading pair symbol
    pub symbol: String,
    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    /// Transaction time
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
    /// Order price
    pub price: String,
    /// Original quantity
    #[serde(rename = "origQty")]
    pub orig_qty: String,
    /// Executed quantity
    #[serde(rename = "executedQty")]
    pub executed_qty: String,
    /// Cumulative quantity
    #[serde(rename = "cumQty")]
    pub cum_qty: String,
    /// Cumulative quote quantity
    #[serde(rename = "cumQuote")]
    pub cum_quote: String,
    /// Order status
    pub status: String,
    /// Time in force
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: String,
    /// Order side
    pub side: String,
    /// Whether order is reduce only
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    /// Whether to close position
    #[serde(rename = "closePosition")]
    pub close_position: bool,
    /// Position side
    #[serde(rename = "positionSide")]
    pub position_side: String,
    /// Stop price
    #[serde(rename = "stopPrice")]
    pub stop_price: String,
    /// Working type
    #[serde(rename = "workingType")]
    pub working_type: String,
    /// Whether price protection is enabled
    #[serde(rename = "priceProtect")]
    pub price_protect: bool,
    /// Original order type
    #[serde(rename = "origType")]
    pub orig_type: String,
    /// Price match
    pub price_match: String,
    /// Self trade prevention mode
    pub self_trade_prevention_mode: String,
    /// Good till date
    pub good_till_date: i64,
}

/// Order report
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderReport {
    /// Trading pair symbol
    pub symbol: String,
    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    /// Transaction time
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
    /// Order price
    pub price: String,
    /// Original quantity
    #[serde(rename = "origQty")]
    pub orig_qty: String,
    /// Executed quantity
    #[serde(rename = "executedQty")]
    pub executed_qty: String,
    /// Cumulative quantity
    #[serde(rename = "cumQty")]
    pub cum_qty: String,
    /// Cumulative quote quantity
    #[serde(rename = "cumQuote")]
    pub cum_quote: String,
    /// Order status
    pub status: String,
    /// Time in force
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: String,
    /// Order side
    pub side: String,
    /// Whether order is reduce only
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    /// Whether to close position
    #[serde(rename = "closePosition")]
    pub close_position: bool,
    /// Position side
    #[serde(rename = "positionSide")]
    pub position_side: String,
    /// Stop price
    #[serde(rename = "stopPrice")]
    pub stop_price: String,
    /// Working type
    #[serde(rename = "workingType")]
    pub working_type: String,
    /// Whether price protection is enabled
    #[serde(rename = "priceProtect")]
    pub price_protect: bool,
    /// Original order type
    #[serde(rename = "origType")]
    pub orig_type: String,
    /// Price match
    pub price_match: String,
    /// Self trade prevention mode
    pub self_trade_prevention_mode: String,
    /// Good till date
    pub good_till_date: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOrder {
    pub symbol: String,
    pub side: String,
    pub type_: String,
    pub quantity: Option<String>,
    pub price: Option<String>,
    pub time_in_force: Option<String>,
    pub reduce_only: Option<bool>,
    pub close_position: Option<bool>,
    pub position_side: Option<String>,
    pub stop_price: Option<String>,
    pub working_type: Option<String>,
    pub price_protect: Option<bool>,
    pub new_client_order_id: Option<String>,
    pub activation_price: Option<String>,
    pub callback_rate: Option<String>,
}

impl BinanceCoinMPrivateRest {
    /// Place multiple orders
    /// 
    /// # Arguments
    /// 
    /// * `orders` - List of orders to place
    /// 
    /// # Returns
    /// 
    /// Response containing the results of the batch order placement
    pub async fn place_batch_orders(&self, orders: Vec<BatchOrder>) -> BinanceCoinMResult<BatchOrderResponse> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let mut query_str = format!("timestamp={}", timestamp);
        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));
        let response = send_request::<BatchOrderResponse, _, _>(
            &self.client,
            &self.base_url,
            "/dapi/v1/batchOrders",
            reqwest::Method::POST, // or PUT if required by API
            Some(&query_str),
            Some(self.api_key.expose_secret()),
            || async { Ok(()) }, // TODO: Replace with actual rate limit check
        ).await?;
        Ok(response.data)
    }
} 