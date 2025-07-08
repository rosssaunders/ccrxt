// New Order (TRADE) endpoint implementation for POST /dapi/v1/order
// See: <https://binance-docs.github.io/apidocs/delivery/en/>

use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{
        OrderResponseType, OrderSide, OrderType, PositionSide, PriceMatch, RestResult,
        SelfTradePreventionMode, TimeInForce, WorkingType, private::rest::client::RestClient,
    },
    shared,
};

/// Request parameters for placing a new order (POST /dapi/v1/order).
#[derive(Debug, Clone, Serialize)]
pub struct NewOrderRequest {
    /// Trading symbol (e.g., "BTCUSD_200925").
    pub symbol: String,

    /// Order side (BUY or SELL).
    pub side: OrderSide,

    /// Position side (BOTH, LONG, SHORT). Optional.
    #[serde(rename = "positionSide", skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,

    /// Order type (LIMIT, MARKET, etc.).
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Time in force (GTC, IOC, FOK, GTX). Optional.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Order quantity. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,

    /// Reduce only. Optional.
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<String>,

    /// Order price. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// New client order ID. Optional.
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Stop price. Optional.
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,

    /// Close position. Optional.
    #[serde(rename = "closePosition", skip_serializing_if = "Option::is_none")]
    pub close_position: Option<String>,

    /// Activation price (for trailing stop). Optional.
    #[serde(rename = "activationPrice", skip_serializing_if = "Option::is_none")]
    pub activation_price: Option<String>,

    /// Callback rate (for trailing stop). Optional.
    #[serde(rename = "callbackRate", skip_serializing_if = "Option::is_none")]
    pub callback_rate: Option<String>,

    /// Working type (MARK_PRICE, CONTRACT_PRICE). Optional.
    #[serde(rename = "workingType", skip_serializing_if = "Option::is_none")]
    pub working_type: Option<WorkingType>,

    /// Price protect. Optional.
    #[serde(rename = "priceProtect", skip_serializing_if = "Option::is_none")]
    pub price_protect: Option<String>,

    /// New order response type (ACK, RESULT). Optional.
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OrderResponseType>,

    /// Price match mode. Optional.
    #[serde(rename = "priceMatch", skip_serializing_if = "Option::is_none")]
    pub price_match: Option<PriceMatch>,

    /// Self-trade prevention mode. Optional.
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    /// Receive window. Optional.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp (milliseconds since epoch).
    pub timestamp: u64,
}

/// Response for a new order (POST /dapi/v1/order).
#[derive(Debug, Clone, Deserialize)]
pub struct NewOrderResponse {
    pub client_order_id: Option<String>,

    #[serde(rename = "cumQty")]
    pub cum_qty: String,

    #[serde(rename = "cumBase")]
    pub cum_base: String,

    #[serde(rename = "executedQty")]
    pub executed_qty: String,

    #[serde(rename = "orderId")]
    pub order_id: u64,

    #[serde(rename = "avgPrice")]
    pub avg_price: String,

    #[serde(rename = "origQty")]
    pub orig_qty: String,

    pub price: String,

    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    pub side: OrderSide,

    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,

    pub status: String,

    #[serde(rename = "stopPrice")]
    pub stop_price: Option<String>,

    #[serde(rename = "closePosition")]
    pub close_position: Option<bool>,

    pub symbol: String,

    pub pair: String,

    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<TimeInForce>,

    #[serde(rename = "type")]
    pub order_type: OrderType,

    #[serde(rename = "origType")]
    pub orig_type: OrderType,

    #[serde(rename = "activatePrice")]
    pub activate_price: Option<String>,

    #[serde(rename = "priceRate")]
    pub price_rate: Option<String>,

    #[serde(rename = "updateTime")]
    pub update_time: u64,

    #[serde(rename = "workingType")]
    pub working_type: WorkingType,

    #[serde(rename = "priceProtect")]
    pub price_protect: bool,

    #[serde(rename = "priceMatch")]
    pub price_match: Option<PriceMatch>,

    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
}

impl RestClient {
    /// Places a new order (TRADE) on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    /// POST /dapi/v1/order
    /// Weight: 1 (order rate limit)
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`NewOrderRequest`])
    ///
    /// # Returns
    /// A [`NewOrderResponse`] object with order details.
    pub async fn post_order(&self, params: NewOrderRequest) -> RestResult<NewOrderResponse> {
        let weight = 1;
        shared::send_signed_request(
            self,
            "/dapi/v1/order",
            reqwest::Method::POST,
            params,
            weight,
            true,
        )
        .await
    }
}
