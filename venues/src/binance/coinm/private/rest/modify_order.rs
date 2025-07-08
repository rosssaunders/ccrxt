// Modify Order (TRADE) endpoint implementation for PUT /dapi/v1/order
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Order>

use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{
        OrderSide, OrderStatus, OrderType, PositionSide, PriceMatch, RestResult,
        SelfTradePreventionMode, TimeInForce, WorkingType, private::rest::client::RestClient,
    },
    shared,
};

/// Request parameters for modifying an existing order (PUT /dapi/v1/order).
#[derive(Debug, Clone, Serialize)]
pub struct ModifyOrderRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Order side (BUY or SELL).
    pub side: OrderSide,

    /// Order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// New order quantity. Either quantity or price must be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,

    /// New order price. Either quantity or price must be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Price match mode. Only available for LIMIT/STOP/TAKE_PROFIT orders.
    /// Cannot be passed together with price.
    #[serde(rename = "priceMatch", skip_serializing_if = "Option::is_none")]
    pub price_match: Option<PriceMatch>,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for modifying an order (PUT /dapi/v1/order).
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyOrderResponse {
    /// Order ID.
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Trading symbol.
    pub symbol: String,

    /// Trading pair.
    pub pair: String,

    /// Order status.
    pub status: OrderStatus,

    /// Client order ID.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Order price.
    pub price: String,

    /// Average price.
    #[serde(rename = "avgPrice")]
    pub avg_price: String,

    /// Original quantity.
    #[serde(rename = "origQty")]
    pub orig_qty: String,

    /// Executed quantity.
    #[serde(rename = "executedQty")]
    pub executed_qty: String,

    /// Cumulative quantity.
    #[serde(rename = "cumQty")]
    pub cum_qty: String,

    /// Cumulative base quantity.
    #[serde(rename = "cumBase")]
    pub cum_base: String,

    /// Time in force.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    /// Order type.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Reduce only flag.
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    /// Close position flag.
    #[serde(rename = "closePosition")]
    pub close_position: bool,

    /// Order side.
    pub side: OrderSide,

    /// Position side.
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,

    /// Stop price.
    #[serde(rename = "stopPrice")]
    pub stop_price: String,

    /// Working type.
    #[serde(rename = "workingType")]
    pub working_type: WorkingType,

    /// Price protect flag.
    #[serde(rename = "priceProtect")]
    pub price_protect: bool,

    /// Original order type.
    #[serde(rename = "origType")]
    pub orig_type: OrderType,

    /// Price match mode.
    #[serde(rename = "priceMatch")]
    pub price_match: PriceMatch,

    /// Self-trade prevention mode.
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,

    /// Update time.
    #[serde(rename = "updateTime")]
    pub update_time: u64,
}

impl RestClient {
    /// Modifies an existing order (TRADE) on Binance Coin-M Futures.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Order>
    /// PUT /dapi/v1/order
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// Currently only LIMIT order modification is supported.
    /// Modified orders will be reordered in the match queue.
    /// Either orderId or origClientOrderId must be sent.
    /// Either quantity or price must be sent.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`ModifyOrderRequest`])
    ///
    /// # Returns
    /// A [`ModifyOrderResponse`] object with updated order details.
    pub async fn modify_order(
        &self,
        params: ModifyOrderRequest,
    ) -> RestResult<ModifyOrderResponse> {
        let weight = 1;
        shared::send_signed_request(
            self,
            "/dapi/v1/order",
            reqwest::Method::PUT,
            params,
            weight,
            true,
        )
        .await
    }
}
