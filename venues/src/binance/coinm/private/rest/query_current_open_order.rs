// Query Current Open Order (USER_DATA) endpoint implementation for GET /dapi/v1/openOrder
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Current-Open-Order>

use serde::{Deserialize, Serialize};

use crate::binance::coinm::RestResult;
use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::coinm::{
    OrderSide, OrderStatus, OrderType, PositionSide, PriceMatch, SelfTradePreventionMode,
    TimeInForce, WorkingType,
};
use crate::binance::shared;

/// Request parameters for querying a current open order (GET /dapi/v1/openOrder).
#[derive(Debug, Clone, Serialize, Default)]
pub struct QueryCurrentOpenOrderRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for querying a current open order (GET /dapi/v1/openOrder).
#[derive(Debug, Clone, Deserialize)]
pub struct QueryCurrentOpenOrderResponse {
    /// Average price.
    #[serde(rename = "avgPrice")]
    pub avg_price: String,

    /// Client order ID.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Cumulative base quantity.
    #[serde(rename = "cumBase")]
    pub cum_base: String,

    /// Executed quantity.
    #[serde(rename = "executedQty")]
    pub executed_qty: String,

    /// Order ID.
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Original quantity.
    #[serde(rename = "origQty")]
    pub orig_qty: String,

    /// Original order type.
    #[serde(rename = "origType")]
    pub orig_type: OrderType,

    /// Order price.
    pub price: String,

    /// Reduce only flag.
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    /// Order side.
    pub side: OrderSide,

    /// Position side.
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,

    /// Order status.
    pub status: OrderStatus,

    /// Stop price.
    #[serde(rename = "stopPrice")]
    pub stop_price: String,

    /// Close position flag.
    #[serde(rename = "closePosition")]
    pub close_position: bool,

    /// Trading symbol.
    pub symbol: String,

    /// Trading pair.
    pub pair: String,

    /// Order time.
    pub time: u64,

    /// Time in force.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    /// Order type.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Activation price (for trailing stop orders).
    #[serde(rename = "activatePrice", skip_serializing_if = "Option::is_none")]
    pub activate_price: Option<String>,

    /// Price rate (for trailing stop orders).
    #[serde(rename = "priceRate", skip_serializing_if = "Option::is_none")]
    pub price_rate: Option<String>,

    /// Update time.
    #[serde(rename = "updateTime")]
    pub update_time: u64,

    /// Working type.
    #[serde(rename = "workingType")]
    pub working_type: WorkingType,

    /// Price protect flag.
    #[serde(rename = "priceProtect")]
    pub price_protect: bool,

    /// Price match mode.
    #[serde(rename = "priceMatch")]
    pub price_match: PriceMatch,

    /// Self-trade prevention mode.
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}

impl RestClient {
    /// Queries a current open order (USER_DATA) on Binance Coin-M Futures.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Current-Open-Order>
    /// GET /dapi/v1/openOrder
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// Either orderId or origClientOrderId must be sent.
    /// If the queried order has been filled or cancelled, the error message "Order does not exist" will be returned.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`QueryCurrentOpenOrderRequest`])
    ///
    /// # Returns
    /// A [`QueryCurrentOpenOrderResponse`] with the open order details.
    pub async fn query_current_open_order(
        &self,
        params: QueryCurrentOpenOrderRequest,
    ) -> RestResult<QueryCurrentOpenOrderResponse> {
        let weight = 1;
        shared::send_signed_request(
            self,
            "/dapi/v1/openOrder",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
