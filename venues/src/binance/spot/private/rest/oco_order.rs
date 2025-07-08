use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    ContingencyType, OrderListOrderStatus, OrderListStatus, OrderResponseType, OrderSide,
    RestResult, SelfTradePreventionMode, TimeInForce,
};

/// Request parameters for OCO order
#[derive(Debug, Clone, Serialize)]
pub struct OcoOrderRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// List client order ID
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,

    /// Order side (BUY or SELL)
    #[serde(rename = "side")]
    pub side: OrderSide,

    /// Order quantity
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Limit client order ID
    #[serde(rename = "limitClientOrderId", skip_serializing_if = "Option::is_none")]
    pub limit_client_order_id: Option<String>,

    /// Limit order price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Limit iceberg quantity
    #[serde(rename = "limitIcebergQty", skip_serializing_if = "Option::is_none")]
    pub limit_iceberg_qty: Option<Decimal>,

    /// Stop client order ID
    #[serde(rename = "stopClientOrderId", skip_serializing_if = "Option::is_none")]
    pub stop_client_order_id: Option<String>,

    /// Stop price
    #[serde(rename = "stopPrice")]
    pub stop_price: Decimal,

    /// Stop limit price
    #[serde(rename = "stopLimitPrice", skip_serializing_if = "Option::is_none")]
    pub stop_limit_price: Option<Decimal>,

    /// Stop limit time in force
    #[serde(
        rename = "stopLimitTimeInForce",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_limit_time_in_force: Option<TimeInForce>,

    /// Stop iceberg quantity
    #[serde(rename = "stopIcebergQty", skip_serializing_if = "Option::is_none")]
    pub stop_iceberg_qty: Option<Decimal>,

    /// Stop strategy ID
    #[serde(rename = "stopStrategyId", skip_serializing_if = "Option::is_none")]
    pub stop_strategy_id: Option<u32>,

    /// Stop strategy type
    #[serde(rename = "stopStrategyType", skip_serializing_if = "Option::is_none")]
    pub stop_strategy_type: Option<u32>,

    /// Response type
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OrderResponseType>,

    /// Self-trade prevention mode
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// OCO order response
#[derive(Debug, Clone, Deserialize)]
pub struct OcoOrderResponse {
    /// Order list ID
    #[serde(rename = "orderListId")]
    pub order_list_id: u64,

    /// Contingency type
    #[serde(rename = "contingencyType")]
    pub contingency_type: ContingencyType,

    /// List status type
    #[serde(rename = "listStatusType")]
    pub list_status_type: OrderListStatus,

    /// List order status
    #[serde(rename = "listOrderStatus")]
    pub list_order_status: OrderListOrderStatus,

    /// List client order ID
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: String,

    /// Transaction time
    #[serde(rename = "transactionTime")]
    pub transaction_time: u64,

    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Orders in the list
    #[serde(rename = "orders")]
    pub orders: Vec<OcoOrder>,

    /// Order reports
    #[serde(rename = "orderReports")]
    pub order_reports: Vec<serde_json::Value>,
}

/// OCO order information
#[derive(Debug, Clone, Deserialize)]
pub struct OcoOrder {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
}

impl RestClient {
    /// Send in a new OCO order
    ///
    /// Send in a new OCO (One-Cancels-Other) order.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#new-oco--trade)
    /// Method: POST /api/v3/order/oco
    /// Weight: 1
    /// Security: TRADE
    pub async fn new_oco_order(&self, params: OcoOrderRequest) -> RestResult<OcoOrderResponse> {
        let body_params: Vec<(&str, String)> = vec![
            ("symbol", params.symbol),
            ("side", params.side.to_string()),
            ("quantity", params.quantity.to_string()),
            ("price", params.price.to_string()),
            ("stopPrice", params.stop_price.to_string()),
        ]
        .into_iter()
        .chain(
            params
                .list_client_order_id
                .map(|v| ("listClientOrderId", v)),
        )
        .chain(
            params
                .limit_client_order_id
                .map(|v| ("limitClientOrderId", v)),
        )
        .chain(
            params
                .limit_iceberg_qty
                .map(|v| ("limitIcebergQty", v.to_string())),
        )
        .chain(
            params
                .stop_client_order_id
                .map(|v| ("stopClientOrderId", v)),
        )
        .chain(
            params
                .stop_limit_price
                .map(|v| ("stopLimitPrice", v.to_string())),
        )
        .chain(
            params
                .stop_limit_time_in_force
                .map(|v| ("stopLimitTimeInForce", v.to_string())),
        )
        .chain(
            params
                .stop_iceberg_qty
                .map(|v| ("stopIcebergQty", v.to_string())),
        )
        .chain(
            params
                .stop_strategy_id
                .map(|v| ("stopStrategyId", v.to_string())),
        )
        .chain(
            params
                .stop_strategy_type
                .map(|v| ("stopStrategyType", v.to_string())),
        )
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
        .chain(params.recv_window.map(|v| ("recvWindow", v.to_string())))
        .collect();

        let body: Vec<(&str, &str)> = body_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.send_request(
            "/api/v3/order/oco",
            reqwest::Method::POST,
            None,
            Some(&body),
            1,
            true,
        )
        .await
    }
}
