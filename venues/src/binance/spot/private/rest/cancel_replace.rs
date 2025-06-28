use rust_decimal::Decimal;
use serde::Serialize;

use crate::binance::spot::{
    CancelReplaceMode, CancelRestrictions, OrderRateLimitExceededMode, OrderResponseType,
    OrderSide, OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

use super::client::RestClient;

/// Request parameters for cancel replace order
#[derive(Debug, Clone, Serialize)]
pub struct CancelReplaceOrderRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order side (BUY or SELL)
    #[serde(rename = "side")]
    pub side: OrderSide,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Cancel replace mode
    #[serde(rename = "cancelReplaceMode")]
    pub cancel_replace_mode: CancelReplaceMode,

    /// Time in force
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Order quantity
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,

    /// Quote order quantity
    #[serde(rename = "quoteOrderQty", skip_serializing_if = "Option::is_none")]
    pub quote_order_qty: Option<Decimal>,

    /// Order price
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,

    /// Cancel order ID
    #[serde(rename = "cancelOrderId", skip_serializing_if = "Option::is_none")]
    pub cancel_order_id: Option<u64>,

    /// Cancel original client order ID
    #[serde(
        rename = "cancelOrigClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancel_orig_client_order_id: Option<String>,

    /// New cancel client order ID
    #[serde(
        rename = "cancelNewClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancel_new_client_order_id: Option<String>,

    /// New client order ID
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Strategy ID
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<u32>,

    /// Strategy type
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<u32>,

    /// Stop price
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<Decimal>,

    /// Trailing delta
    #[serde(rename = "trailingDelta", skip_serializing_if = "Option::is_none")]
    pub trailing_delta: Option<u32>,

    /// Iceberg quantity
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    pub iceberg_qty: Option<Decimal>,

    /// Response type
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OrderResponseType>,

    /// Self-trade prevention mode
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    /// Cancel restrictions
    #[serde(rename = "cancelRestrictions", skip_serializing_if = "Option::is_none")]
    pub cancel_restrictions: Option<CancelRestrictions>,

    /// Order rate limit exceeded mode
    #[serde(
        rename = "orderRateLimitExceededMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_rate_limit_exceeded_mode: Option<OrderRateLimitExceededMode>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

impl RestClient {
    /// Cancel and replace order
    ///
    /// Cancel an existing order and place a new order on the same symbol.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#cancel-and-replace-order--trade)
    /// Method: POST /api/v3/order/cancelReplace
    /// Weight: 1
    /// Security: TRADE
    pub async fn cancel_replace_order(
        &self,
        params: CancelReplaceOrderRequest,
    ) -> RestResult<serde_json::Value> {
        let body_params: Vec<(&str, String)> = vec![
            ("symbol", params.symbol),
            ("side", params.side.to_string()),
            ("type", params.order_type.to_string()),
            ("cancelReplaceMode", params.cancel_replace_mode.to_string()),
        ]
        .into_iter()
        .chain(params.time_in_force.map(|v| ("timeInForce", v.to_string())))
        .chain(params.quantity.map(|v| ("quantity", v.to_string())))
        .chain(
            params
                .quote_order_qty
                .map(|v| ("quoteOrderQty", v.to_string())),
        )
        .chain(params.price.map(|v| ("price", v.to_string())))
        .chain(
            params
                .cancel_order_id
                .map(|v| ("cancelOrderId", v.to_string())),
        )
        .chain(
            params
                .cancel_orig_client_order_id
                .map(|v| ("cancelOrigClientOrderId", v)),
        )
        .chain(
            params
                .cancel_new_client_order_id
                .map(|v| ("cancelNewClientOrderId", v)),
        )
        .chain(params.new_client_order_id.map(|v| ("newClientOrderId", v)))
        .chain(params.strategy_id.map(|v| ("strategyId", v.to_string())))
        .chain(
            params
                .strategy_type
                .map(|v| ("strategyType", v.to_string())),
        )
        .chain(params.stop_price.map(|v| ("stopPrice", v.to_string())))
        .chain(
            params
                .trailing_delta
                .map(|v| ("trailingDelta", v.to_string())),
        )
        .chain(params.iceberg_qty.map(|v| ("icebergQty", v.to_string())))
        .chain(
            params
                .new_order_resp_type
                .map(|v| ("newOrderRespType", v.to_string())),
        )
        .chain(
            params
                .self_trade_prevention_mode
                .map(|v| ("selfTradePreventionMode", v.to_string())),
        )
        .chain(
            params
                .cancel_restrictions
                .map(|v| ("cancelRestrictions", v.to_string())),
        )
        .chain(
            params
                .order_rate_limit_exceeded_mode
                .map(|v| ("orderRateLimitExceededMode", v.to_string())),
        )
        .chain(params.recv_window.map(|v| ("recvWindow", v.to_string())))
        .collect();

        let body: Vec<(&str, &str)> = body_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.send_request(
            "/api/v3/order/cancelReplace",
            reqwest::Method::POST,
            None,
            Some(&body),
            1,
            true,
        )
        .await
    }
}
