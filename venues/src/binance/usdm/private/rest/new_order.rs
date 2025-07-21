use super::UsdmClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

const NEW_ORDER_ENDPOINT: &str = "/fapi/v1/order";

/// Request parameters for placing a new order.
#[derive(Debug, Clone, Serialize)]
pub struct NewOrderRequest {
    /// Symbol (e.g., "BTCUSDT")
    pub symbol: Cow<'static, str>,

    /// Side (BUY or SELL)
    pub side: OrderSide,

    /// Position side (BOTH, LONG, SHORT)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,

    /// Order type (LIMIT, MARKET, etc.)
    pub order_type: OrderType,

    /// Time in force (GTC, IOC, FOK, GTX)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,

    /// Price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// Reduce only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// New client order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<Cow<'static, str>>,

    /// Stop price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,

    /// Close position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_position: Option<bool>,

    /// Activation price (for TRAILING_STOP_MARKET)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_price: Option<f64>,

    /// Callback rate (for TRAILING_STOP_MARKET)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_rate: Option<f64>,

    /// Working type (MARK_PRICE or CONTRACT_PRICE)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_type: Option<WorkingType>,

    /// Price protect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_protect: Option<bool>,

    /// New order response type (ACK or RESULT)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OrderResponseType>,

    /// Self trade prevention mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
}

/// Response for a new order.
#[derive(Debug, Clone, Deserialize)]
pub struct NewOrderResponse {
    /// Symbol
    pub symbol: Cow<'static, str>,
    /// Order ID
    pub order_id: u64,
    /// Client order ID
    pub client_order_id: Cow<'static, str>,
    /// Transaction time (milliseconds since epoch)
    pub transact_time: u64,
    /// Price (as string for precision)
    pub price: Cow<'static, str>,
    /// Original quantity (as string for precision)
    pub orig_qty: Cow<'static, str>,
    /// Executed quantity (as string for precision)
    pub executed_qty: Cow<'static, str>,
    /// Cumulative quote asset transacted quantity (as string)
    pub cum_quote: Cow<'static, str>,
    /// Status
    pub status: OrderStatus,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Type
    pub order_type: OrderType,
    /// Side
    pub side: OrderSide,
    /// Position side
    pub position_side: PositionSide,
    /// Working type
    pub working_type: WorkingType,
}

impl UsdmClient {
    /// New Order
    ///
    /// Places a new order on Binance USDM Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/New-Order
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The new order request parameters
    ///
    /// # Returns
    /// Response containing order details
    pub async fn new_order(&self, request: NewOrderRequest) -> RestResult<NewOrderResponse> {
        self.send_signed_request(NEW_ORDER_ENDPOINT, reqwest::Method::POST, request, 1, false)
            .await
    }
}
