// Cancel Order (TRADE) endpoint implementation for DELETE /dapi/v1/order
// See: <https://binance-docs.github.io/apidocs/delivery/en/>

use crate::binance::coinm::RestResult;
use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::coinm::{OrderSide, OrderType, PositionSide, PriceMatch, SelfTradePreventionMode, TimeInForce, WorkingType};
use serde::{Deserialize, Serialize};

/// Request parameters for canceling an active order (DELETE /dapi/v1/order).
#[derive(Debug, Clone, Serialize, Default)]
pub struct CancelOrderRequest {
    /// Trading symbol (e.g., "BTCUSD_200925").
    pub symbol: String,

    /// Order ID to cancel. Either `order_id` or `orig_client_order_id` must be sent.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Either `order_id` or `orig_client_order_id` must be sent.
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// The value cannot be greater than 60000.
    /// Range: 0 to 60000 milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds since epoch. Mandatory.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Response for canceling an order (DELETE /dapi/v1/order).
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderResponse {
    #[serde(rename = "avgPrice")]
    pub avg_price: String,

    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    #[serde(rename = "cumQty")]
    pub cum_qty: String,

    #[serde(rename = "cumBase")]
    pub cum_base: String,

    #[serde(rename = "executedQty")]
    pub executed_qty: String,

    #[serde(rename = "orderId")]
    pub order_id: u64,

    #[serde(rename = "origQty")]
    pub orig_qty: String,

    #[serde(rename = "origType")]
    pub orig_type: OrderType,

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
    /// Cancels an active order on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    /// DELETE /dapi/v1/order
    /// Weight: 1 (order rate limit)
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`CancelOrderRequest`])
    ///
    /// # Returns
    /// A [`CancelOrderResponse`] object with order details.
    pub async fn delete_order(&self, params: CancelOrderRequest) -> RestResult<CancelOrderResponse> {
        let weight = 1;
        self.send_signed_request(
            "/dapi/v1/order",
            reqwest::Method::DELETE,
            params,
            weight,
            true, // is_order
        )
        .await
    }
}
