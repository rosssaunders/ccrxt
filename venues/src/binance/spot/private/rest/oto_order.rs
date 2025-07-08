use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    ContingencyType, OrderListOrderStatus, OrderListStatus, OrderResponseType, OrderSide,
    OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

/// Request parameters for OTO order
#[derive(Debug, Clone, Serialize)]
pub struct OtoOrderRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// List client order ID
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,

    /// Response type
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OrderResponseType>,

    /// Self-trade prevention mode
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    // Working order parameters
    /// Working order type
    #[serde(rename = "workingType")]
    pub working_type: OrderType,

    /// Working order side
    #[serde(rename = "workingSide")]
    pub working_side: OrderSide,

    /// Working client order ID
    #[serde(
        rename = "workingClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pub working_client_order_id: Option<String>,

    /// Working order quantity
    #[serde(rename = "workingQuantity")]
    pub working_quantity: Decimal,

    /// Working order price
    #[serde(rename = "workingPrice")]
    pub working_price: Decimal,

    /// Working time in force
    #[serde(rename = "workingTimeInForce", skip_serializing_if = "Option::is_none")]
    pub working_time_in_force: Option<TimeInForce>,

    /// Working strategy ID
    #[serde(rename = "workingStrategyId", skip_serializing_if = "Option::is_none")]
    pub working_strategy_id: Option<u32>,

    /// Working strategy type
    #[serde(
        rename = "workingStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub working_strategy_type: Option<u32>,

    /// Working iceberg quantity
    #[serde(rename = "workingIcebergQty", skip_serializing_if = "Option::is_none")]
    pub working_iceberg_qty: Option<Decimal>,

    // Pending order parameters
    /// Pending order type
    #[serde(rename = "pendingType")]
    pub pending_type: OrderType,

    /// Pending order side
    #[serde(rename = "pendingSide")]
    pub pending_side: OrderSide,

    /// Pending client order ID
    #[serde(
        rename = "pendingClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_client_order_id: Option<String>,

    /// Pending order quantity
    #[serde(rename = "pendingQuantity")]
    pub pending_quantity: Decimal,

    /// Pending order price
    #[serde(rename = "pendingPrice", skip_serializing_if = "Option::is_none")]
    pub pending_price: Option<Decimal>,

    /// Pending stop price
    #[serde(rename = "pendingStopPrice", skip_serializing_if = "Option::is_none")]
    pub pending_stop_price: Option<Decimal>,

    /// Pending trailing delta
    #[serde(
        rename = "pendingTrailingDelta",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_trailing_delta: Option<u32>,

    /// Pending time in force
    #[serde(rename = "pendingTimeInForce", skip_serializing_if = "Option::is_none")]
    pub pending_time_in_force: Option<TimeInForce>,

    /// Pending strategy ID
    #[serde(rename = "pendingStrategyId", skip_serializing_if = "Option::is_none")]
    pub pending_strategy_id: Option<u32>,

    /// Pending strategy type
    #[serde(
        rename = "pendingStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_strategy_type: Option<u32>,

    /// Pending iceberg quantity
    #[serde(rename = "pendingIcebergQty", skip_serializing_if = "Option::is_none")]
    pub pending_iceberg_qty: Option<Decimal>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// OTO order response
#[derive(Debug, Clone, Deserialize)]
pub struct OtoOrderResponse {
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
    pub orders: Vec<OtoOrder>,

    /// Order reports
    #[serde(rename = "orderReports")]
    pub order_reports: Vec<serde_json::Value>,
}

/// OTO order information
#[derive(Debug, Clone, Deserialize)]
pub struct OtoOrder {
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
    /// Place an OTO order
    ///
    /// Place an OTO (One-Triggers-Other) order.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#new-oto--trade)
    /// Method: POST /api/v3/orderList/oto
    /// Weight: 1
    /// Security: TRADE
    pub async fn new_oto_order(&self, params: OtoOrderRequest) -> RestResult<OtoOrderResponse> {
        let body_params: Vec<(&str, String)> = vec![
            ("symbol", params.symbol),
            ("workingType", params.working_type.to_string()),
            ("workingSide", params.working_side.to_string()),
            ("workingQuantity", params.working_quantity.to_string()),
            ("workingPrice", params.working_price.to_string()),
            ("pendingType", params.pending_type.to_string()),
            ("pendingSide", params.pending_side.to_string()),
            ("pendingQuantity", params.pending_quantity.to_string()),
        ]
        .into_iter()
        .chain(
            params
                .list_client_order_id
                .map(|v| ("listClientOrderId", v)),
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
        .chain(
            params
                .working_client_order_id
                .map(|v| ("workingClientOrderId", v)),
        )
        .chain(
            params
                .working_time_in_force
                .map(|v| ("workingTimeInForce", v.to_string())),
        )
        .chain(
            params
                .working_strategy_id
                .map(|v| ("workingStrategyId", v.to_string())),
        )
        .chain(
            params
                .working_strategy_type
                .map(|v| ("workingStrategyType", v.to_string())),
        )
        .chain(
            params
                .working_iceberg_qty
                .map(|v| ("workingIcebergQty", v.to_string())),
        )
        .chain(
            params
                .pending_client_order_id
                .map(|v| ("pendingClientOrderId", v)),
        )
        .chain(
            params
                .pending_price
                .map(|v| ("pendingPrice", v.to_string())),
        )
        .chain(
            params
                .pending_stop_price
                .map(|v| ("pendingStopPrice", v.to_string())),
        )
        .chain(
            params
                .pending_trailing_delta
                .map(|v| ("pendingTrailingDelta", v.to_string())),
        )
        .chain(
            params
                .pending_time_in_force
                .map(|v| ("pendingTimeInForce", v.to_string())),
        )
        .chain(
            params
                .pending_strategy_id
                .map(|v| ("pendingStrategyId", v.to_string())),
        )
        .chain(
            params
                .pending_strategy_type
                .map(|v| ("pendingStrategyType", v.to_string())),
        )
        .chain(
            params
                .pending_iceberg_qty
                .map(|v| ("pendingIcebergQty", v.to_string())),
        )
        .chain(params.recv_window.map(|v| ("recvWindow", v.to_string())))
        .collect();

        let body: Vec<(&str, &str)> = body_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.send_request(
            "/api/v3/orderList/oto",
            reqwest::Method::POST,
            None,
            Some(&body),
            1,
            true,
        )
        .await
    }
}
