use serde::{Deserialize, Serialize};
use super::{
    api_errors::BinanceCoinMResult,
    private_rest::BinanceCoinMPrivateRest,
    types::BinanceResponse,
    common::request::send_request,
    enums::{OrderSide, OrderType, TimeInForce, PositionSide, WorkingType, OrderStatus},
};

#[derive(Debug, Serialize, Deserialize)]
/// Request struct for creating a new order
pub struct OrderRequest {
    /// Trading pair symbol (e.g. "BTCUSD_PERP")
    pub symbol: String,
    /// Order side (BUY or SELL)
    pub side: OrderSide,
    /// Order type (LIMIT, MARKET, STOP, etc.)
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order quantity in contracts
    pub quantity: f64,
    /// Order price (required for LIMIT orders)
    pub price: Option<f64>,
    /// Time in force (GTC, IOC, FOK, GTX)
    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<TimeInForce>,
    /// Whether this order can only reduce the position size
    #[serde(rename = "reduceOnly")]
    pub reduce_only: Option<bool>,
    /// Whether this order should close the entire position
    #[serde(rename = "closePosition")]
    pub close_position: Option<bool>,
    /// Position side (BOTH, LONG, SHORT)
    #[serde(rename = "positionSide")]
    pub position_side: Option<PositionSide>,
    /// Working type for stop orders (MARK_PRICE or CONTRACT_PRICE)
    #[serde(rename = "workingType")]
    pub working_type: Option<WorkingType>,
    /// Whether to enable price protection for stop orders
    #[serde(rename = "priceProtect")]
    pub price_protect: Option<bool>,
    /// Custom order ID (max 36 chars)
    #[serde(rename = "newClientOrderId")]
    pub new_client_order_id: Option<String>,
    /// Stop price for stop orders
    #[serde(rename = "stopPrice")]
    pub stop_price: Option<f64>,
    /// Activation price for trailing stop orders
    #[serde(rename = "activationPrice")]
    pub activation_price: Option<f64>,
    /// Callback rate for trailing stop orders (0.1 to 10)
    #[serde(rename = "callbackRate")]
    pub callback_rate: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Response struct for order creation
pub struct OrderResponse {
    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    /// Cumulative filled quantity
    #[serde(rename = "cumQty")]
    pub cum_qty: String,
    /// Cumulative filled quote quantity
    #[serde(rename = "cumQuote")]
    pub cum_quote: String,
    /// Executed quantity
    #[serde(rename = "executedQty")]
    pub executed_qty: String,
    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Average filled price
    #[serde(rename = "avgPrice")]
    pub avg_price: String,
    /// Original order quantity
    #[serde(rename = "origQty")]
    pub orig_qty: String,
    /// Order price
    pub price: String,
    /// Whether this order can only reduce the position size
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    /// Order side (BUY or SELL)
    pub side: OrderSide,
    /// Position side (BOTH, LONG, SHORT)
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,
    /// Order status
    pub status: OrderStatus,
    /// Stop price for stop orders
    #[serde(rename = "stopPrice")]
    pub stop_price: String,
    /// Whether this order should close the entire position
    #[serde(rename = "closePosition")]
    pub close_position: bool,
    /// Trading pair symbol
    pub symbol: String,
    /// Time in force
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Original order type
    #[serde(rename = "origType")]
    pub orig_type: OrderType,
    /// Activation price for trailing stop orders
    #[serde(rename = "activatePrice")]
    pub activate_price: String,
    /// Price rate for trailing stop orders
    #[serde(rename = "priceRate")]
    pub price_rate: String,
    /// Last update time in milliseconds
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    /// Working type for stop orders
    #[serde(rename = "workingType")]
    pub working_type: WorkingType,
    /// Whether price protection is enabled
    #[serde(rename = "priceProtect")]
    pub price_protect: bool,
}

impl BinanceCoinMPrivateRest {
    /// Creates a new order for the specified symbol.
    /// 
    /// # Notes
    /// 
    /// * Order with type `STOP`, parameter `timeInForce` can be sent (default `GTC`)
    /// * Order with type `TAKE_PROFIT`, parameter `timeInForce` can be sent (default `GTC`)
    /// * For `TRAILING_STOP_MARKET`, if you get error code -2021 "Order would immediately trigger":
    ///   - BUY: `activationPrice` should be smaller than latest price
    ///   - SELL: `activationPrice` should be larger than latest price
    /// 
    /// # Documentation
    /// 
    /// [Binance COIN-M Futures API Documentation](https://binance-docs.github.io/apidocs/delivery/en/#new-order-trade)
    pub async fn create_order(&self, request: OrderRequest) -> BinanceCoinMResult<BinanceResponse<OrderResponse>> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let mut query_string = format!(
            "symbol={}&side={}&type={}&quantity={}&timestamp={}",
            request.symbol, request.side, request.order_type, request.quantity, timestamp
        );
        
        if let Some(p) = request.price {
            query_string.push_str(&format!("&price={}", p));
        }
        
        if let Some(tif) = request.time_in_force {
            query_string.push_str(&format!("&timeInForce={}", tif));
        }
        
        if let Some(reduce_only) = request.reduce_only {
            query_string.push_str(&format!("&reduceOnly={}", reduce_only));
        }
        
        if let Some(close_position) = request.close_position {
            query_string.push_str(&format!("&closePosition={}", close_position));
        }
        
        if let Some(position_side) = request.position_side {
            query_string.push_str(&format!("&positionSide={}", position_side));
        }
        
        if let Some(working_type) = request.working_type {
            query_string.push_str(&format!("&workingType={}", working_type));
        }
        
        if let Some(price_protect) = request.price_protect {
            query_string.push_str(&format!("&priceProtect={}", price_protect));
        }
        
        if let Some(client_order_id) = request.new_client_order_id {
            query_string.push_str(&format!("&newClientOrderId={}", client_order_id));
        }
        
        if let Some(stop_price) = request.stop_price {
            query_string.push_str(&format!("&stopPrice={}", stop_price));
        }
        
        if let Some(activation_price) = request.activation_price {
            query_string.push_str(&format!("&activationPrice={}", activation_price));
        }
        
        if let Some(callback_rate) = request.callback_rate {
            query_string.push_str(&format!("&callbackRate={}", callback_rate));
        }
        
        let signature = self.sign_request(&query_string);
        let endpoint = format!("/dapi/v1/order?{}&signature={}", query_string, signature);
        
        send_request(
            &self.client,
            &self.base_url,
            &endpoint,
            reqwest::Method::POST,
            None,
            Some(&self.api_key),
            || self.rate_limiter.check_weight_limit("order", 1)
        ).await
    }
} 