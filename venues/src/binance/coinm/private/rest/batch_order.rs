use serde::Serialize as SerdeSerialize;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::enums::{
    OrderResponseType, OrderSide, OrderStatus, OrderType, PositionSide, PriceMatch,
    SelfTradePreventionMode, TimeInForce, WorkingType,
};
use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::coinm::{ErrorResponse, RestResult};
use crate::binance::shared;

/// Serializes a value as a JSON string for use in URL-encoded form bodies (Binance batch orders)
fn as_json_string<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: SerdeSerialize,
{
    let json = serde_json::to_string(value).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&json)
}

/// Represents a single order in a batch order request.
///
/// All fields map directly to the Binance Coin-M Futures API parameters.
#[derive(Debug, Clone, Serialize)]
pub struct BatchOrderRequest {
    /// Trading symbol, e.g. "BTCUSD_PERP".
    /// Format: "{ASSET}USD_PERP" for perpetual contracts, "{ASSET}USD_{EXPIRY}" for delivery contracts.
    pub symbol: String,

    /// Order side (BUY or SELL).
    /// Indicates whether to buy or sell the asset.
    pub side: OrderSide,

    /// Position side (BOTH, LONG, SHORT). Required for Hedge Mode.
    /// Only required when hedge mode is enabled.
    #[serde(rename = "positionSide")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,

    /// Order type (LIMIT, MARKET, etc).
    /// Determines how the order will be executed.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Time in force (GTC, IOC, FOK, POST_ONLY).
    /// Determines how long the order remains active.
    #[serde(rename = "timeInForce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Order quantity.
    /// Format: Decimal string with precision up to 8 decimal places.
    pub quantity: String,

    /// Reduce only flag ("true" or "false").
    /// If true, the order can only reduce the position size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<String>,

    /// Order price.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Required for LIMIT orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Unique client order ID.
    /// Must be unique across all orders for this account.
    /// Format: String, max length 36 characters.
    #[serde(rename = "newClientOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Stop price (for STOP/TAKE_PROFIT orders).
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Required for STOP and TAKE_PROFIT orders.
    #[serde(rename = "stopPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,

    /// Activation price (for TRAILING_STOP_MARKET orders).
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Required for TRAILING_STOP_MARKET orders.
    #[serde(rename = "activationPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_price: Option<String>,

    /// Callback rate (for TRAILING_STOP_MARKET orders).
    /// Format: Decimal string with precision up to 4 decimal places.
    /// Required for TRAILING_STOP_MARKET orders.
    #[serde(rename = "callbackRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_rate: Option<String>,

    /// Working type (MARK_PRICE or CONTRACT_PRICE).
    /// Determines which price is used for stop orders.
    #[serde(rename = "workingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_type: Option<WorkingType>,

    /// Price protect flag ("TRUE" or "FALSE").
    /// If true, the order will be protected against adverse price movements.
    #[serde(rename = "priceProtect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_protect: Option<String>,

    /// Response type (ACK or RESULT).
    /// Determines the level of detail in the response.
    #[serde(rename = "newOrderRespType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OrderResponseType>,

    /// Price match mode.
    /// Determines how the order price is matched.
    #[serde(rename = "priceMatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_match: Option<PriceMatch>,

    /// Self-trade prevention mode.
    /// Determines how self-trades are handled.
    #[serde(rename = "selfTradePreventionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
}

/// Request type for placing multiple orders (batch).
#[derive(Debug, Clone, Serialize)]
pub struct PlaceBatchOrdersRequest {
    /// List of orders to place (max 5).
    /// Each order must be a valid BatchOrderRequest.
    #[serde(rename = "batchOrders", serialize_with = "as_json_string")]
    pub batch_orders: Vec<BatchOrderRequest>,

    /// Optional recvWindow parameter.
    /// Range: 0 to 60000 milliseconds.
    #[serde(rename = "recvWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    /// Must be within 1000ms of server time.
    pub timestamp: u64,
}

/// Represents a successful response for a single order in a batch.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchOrderResponse {
    /// Client order ID.
    /// The unique identifier provided in the request.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Cumulative filled quantity.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "cumQty")]
    pub cum_qty: String,

    /// Cumulative filled base quantity.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "cumBase")]
    pub cum_base: String,

    /// Executed quantity.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "executedQty")]
    pub executed_qty: String,

    /// Order ID.
    /// Unique identifier assigned by the exchange.
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Average price.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "avgPrice")]
    pub avg_price: String,

    /// Original quantity.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "origQty")]
    pub orig_qty: String,

    /// Price.
    /// Format: Decimal string with precision up to 8 decimal places.
    pub price: String,

    /// Reduce only flag.
    /// True if the order can only reduce the position size.
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    /// Order side.
    /// Indicates whether the order was a buy or sell.
    pub side: OrderSide,

    /// Position side.
    /// Indicates the position direction for this order.
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,

    /// Order status.
    /// Current state of the order.
    pub status: OrderStatus,

    /// Stop price.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Only present for stop orders.
    #[serde(rename = "stopPrice")]
    pub stop_price: Option<String>,

    /// Symbol.
    /// The trading pair for this order.
    pub symbol: String,

    /// Pair.
    /// The base trading pair without contract specifications.
    pub pair: String,

    /// Time in force.
    /// How long the order remains active.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    /// Order type.
    /// The type of order that was placed.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Original order type.
    /// The type of order as specified in the request.
    #[serde(rename = "origType")]
    pub orig_type: OrderType,

    /// Activation price (for TRAILING_STOP_MARKET).
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Only present for trailing stop orders.
    #[serde(rename = "activatePrice")]
    pub activate_price: Option<String>,

    /// Callback rate (for TRAILING_STOP_MARKET).
    /// Format: Decimal string with precision up to 4 decimal places.
    /// Only present for trailing stop orders.
    #[serde(rename = "priceRate")]
    pub price_rate: Option<String>,

    /// Update time.
    /// Format: Unix timestamp in milliseconds.
    #[serde(rename = "updateTime")]
    pub update_time: u64,

    /// Working type.
    /// Which price is used for stop orders.
    #[serde(rename = "workingType")]
    pub working_type: WorkingType,

    /// Price protect flag.
    /// Whether the order is protected against adverse price movements.
    #[serde(rename = "priceProtect")]
    pub price_protect: bool,

    /// Price match mode.
    /// How the order price is matched.
    #[serde(rename = "priceMatch")]
    pub price_match: PriceMatch,

    /// Self-trade prevention mode.
    /// How self-trades are handled.
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}

/// Represents a single response entry for a batch order (either success or error).
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum BatchOrderResult {
    /// Successful order response.
    Ok(BatchOrderResponse),
    /// Error response for a failed order.
    Err(ErrorResponse),
}

impl RestClient {
    /// Places multiple orders in a single batch.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Place-Multiple-Orders>
    ///
    /// POST /dapi/v1/batchOrders
    /// Weight: 5
    ///
    /// # Arguments
    /// * `request` - PlaceBatchOrdersRequest containing the orders and required parameters
    ///
    /// # Returns
    /// A vector of BatchOrderResult, each representing either a successful order or an error for that order.
    pub async fn place_batch_orders(
        &self,
        request: PlaceBatchOrdersRequest,
    ) -> RestResult<Vec<BatchOrderResult>> {
        shared::send_signed_request(
            self,
            "/dapi/v1/batchOrders",
            reqwest::Method::POST,
            request,
            5,    // weight
            true, // is_order
        )
        .await
    }
}
