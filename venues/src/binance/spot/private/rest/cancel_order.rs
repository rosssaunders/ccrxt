use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    CancelRestrictions, OrderSide, OrderStatus, OrderType, RestResult, SelfTradePreventionMode,
    TimeInForce,
};

/// Request parameters for cancelling an order
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// New client order ID for the cancel
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Cancel restrictions
    #[serde(rename = "cancelRestrictions", skip_serializing_if = "Option::is_none")]
    pub cancel_restrictions: Option<CancelRestrictions>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Cancel order response
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderResponse {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Original client order ID
    #[serde(rename = "origClientOrderId")]
    pub orig_client_order_id: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Order list ID
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Transaction time
    #[serde(rename = "transactTime")]
    pub transact_time: u64,

    /// Order price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Original quantity
    #[serde(rename = "origQty")]
    pub orig_qty: Decimal,

    /// Executed quantity
    #[serde(rename = "executedQty")]
    pub executed_qty: Decimal,

    /// Cumulative quote quantity
    #[serde(rename = "cummulativeQuoteQty")]
    pub cummulative_quote_qty: Decimal,

    /// Order status
    #[serde(rename = "status")]
    pub status: OrderStatus,

    /// Time in force
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order side
    #[serde(rename = "side")]
    pub side: OrderSide,

    /// Stop price
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<Decimal>,

    /// Iceberg quantity
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    pub iceberg_qty: Option<Decimal>,

    /// Self-trade prevention mode
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}

impl RestClient {
    /// Cancel an active order
    ///
    /// Cancel an active order.
    /// Either orderId or origClientOrderId must be sent.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#cancel-order--trade)
    /// Method: DELETE /api/v3/order
    /// Weight: 1
    /// Security: TRADE
    pub async fn cancel_order(
        &self,
        params: CancelOrderRequest,
    ) -> RestResult<CancelOrderResponse> {
        let body_params: Vec<(&str, String)> = vec![("symbol", params.symbol)]
            .into_iter()
            .chain(params.order_id.map(|v| ("orderId", v.to_string())))
            .chain(
                params
                    .orig_client_order_id
                    .map(|v| ("origClientOrderId", v)),
            )
            .chain(params.new_client_order_id.map(|v| ("newClientOrderId", v)))
            .chain(
                params
                    .cancel_restrictions
                    .map(|v| ("cancelRestrictions", v.to_string())),
            )
            .chain(params.recv_window.map(|v| ("recvWindow", v.to_string())))
            .collect();

        let body: Vec<(&str, &str)> = body_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.send_request(
            "/api/v3/order",
            reqwest::Method::DELETE,
            None,
            Some(&body),
            1,
            true,
        )
        .await
    }
}
