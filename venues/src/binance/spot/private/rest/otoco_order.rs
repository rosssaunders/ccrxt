use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    ContingencyType, OrderListOrderStatus, OrderListStatus, OrderResponseType, OrderSide,
    OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

/// Request parameters for OTOCO order
#[derive(Debug, Clone, Serialize)]
pub struct OtocoOrderRequest {
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

    // Pending order parameters (side is shared for above/below)
    /// Pending order side
    #[serde(rename = "pendingSide")]
    pub pending_side: OrderSide,

    /// Pending order quantity
    #[serde(rename = "pendingQuantity")]
    pub pending_quantity: Decimal,

    // Pending above parameters
    /// Pending above order type
    #[serde(rename = "pendingAboveType")]
    pub pending_above_type: OrderType,

    /// Pending above client order ID
    #[serde(
        rename = "pendingAboveClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_above_client_order_id: Option<String>,

    /// Pending above order price
    #[serde(rename = "pendingAbovePrice", skip_serializing_if = "Option::is_none")]
    pub pending_above_price: Option<Decimal>,

    /// Pending above stop price
    #[serde(
        rename = "pendingAboveStopPrice",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_above_stop_price: Option<Decimal>,

    /// Pending above trailing delta
    #[serde(
        rename = "pendingAboveTrailingDelta",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_above_trailing_delta: Option<u32>,

    /// Pending above iceberg quantity
    #[serde(
        rename = "pendingAboveIcebergQty",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_above_iceberg_qty: Option<Decimal>,

    /// Pending above time in force
    #[serde(
        rename = "pendingAboveTimeInForce",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_above_time_in_force: Option<TimeInForce>,

    /// Pending above strategy ID
    #[serde(
        rename = "pendingAboveStrategyId",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_above_strategy_id: Option<u32>,

    /// Pending above strategy type
    #[serde(
        rename = "pendingAboveStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_above_strategy_type: Option<u32>,

    // Pending below parameters
    /// Pending below order type
    #[serde(rename = "pendingBelowType")]
    pub pending_below_type: OrderType,

    /// Pending below client order ID
    #[serde(
        rename = "pendingBelowClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_below_client_order_id: Option<String>,

    /// Pending below order price
    #[serde(rename = "pendingBelowPrice", skip_serializing_if = "Option::is_none")]
    pub pending_below_price: Option<Decimal>,

    /// Pending below stop price
    #[serde(
        rename = "pendingBelowStopPrice",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_below_stop_price: Option<Decimal>,

    /// Pending below trailing delta
    #[serde(
        rename = "pendingBelowTrailingDelta",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_below_trailing_delta: Option<u32>,

    /// Pending below iceberg quantity
    #[serde(
        rename = "pendingBelowIcebergQty",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_below_iceberg_qty: Option<Decimal>,

    /// Pending below time in force
    #[serde(
        rename = "pendingBelowTimeInForce",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_below_time_in_force: Option<TimeInForce>,

    /// Pending below strategy ID
    #[serde(
        rename = "pendingBelowStrategyId",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_below_strategy_id: Option<u32>,

    /// Pending below strategy type
    #[serde(
        rename = "pendingBelowStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_below_strategy_type: Option<u32>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// OTOCO order response
#[derive(Debug, Clone, Deserialize)]
pub struct OtocoOrderResponse {
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
    pub orders: Vec<OtocoOrder>,

    /// Order reports
    #[serde(rename = "orderReports")]
    pub order_reports: Vec<serde_json::Value>,
}

/// OTOCO order information
#[derive(Debug, Clone, Deserialize)]
pub struct OtocoOrder {
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
    /// Place an OTOCO order
    ///
    /// Place an OTOCO (One-Triggers-One-Cancels-Other) order.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#new-otoco--trade)
    /// Method: POST /api/v3/orderList/otoco
    /// Weight: 1
    /// Security: TRADE
    pub async fn new_otoco_order(
        &self,
        params: OtocoOrderRequest,
    ) -> RestResult<OtocoOrderResponse> {
        let body_params: Vec<(&str, String)> = vec![
            ("symbol", params.symbol),
            ("workingType", params.working_type.to_string()),
            ("workingSide", params.working_side.to_string()),
            ("workingQuantity", params.working_quantity.to_string()),
            ("workingPrice", params.working_price.to_string()),
            ("pendingSide", params.pending_side.to_string()),
            ("pendingQuantity", params.pending_quantity.to_string()),
            ("pendingAboveType", params.pending_above_type.to_string()),
            ("pendingBelowType", params.pending_below_type.to_string()),
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
                .pending_above_client_order_id
                .map(|v| ("pendingAboveClientOrderId", v)),
        )
        .chain(
            params
                .pending_above_price
                .map(|v| ("pendingAbovePrice", v.to_string())),
        )
        .chain(
            params
                .pending_above_stop_price
                .map(|v| ("pendingAboveStopPrice", v.to_string())),
        )
        .chain(
            params
                .pending_above_trailing_delta
                .map(|v| ("pendingAboveTrailingDelta", v.to_string())),
        )
        .chain(
            params
                .pending_above_iceberg_qty
                .map(|v| ("pendingAboveIcebergQty", v.to_string())),
        )
        .chain(
            params
                .pending_above_time_in_force
                .map(|v| ("pendingAboveTimeInForce", v.to_string())),
        )
        .chain(
            params
                .pending_above_strategy_id
                .map(|v| ("pendingAboveStrategyId", v.to_string())),
        )
        .chain(
            params
                .pending_above_strategy_type
                .map(|v| ("pendingAboveStrategyType", v.to_string())),
        )
        .chain(
            params
                .pending_below_client_order_id
                .map(|v| ("pendingBelowClientOrderId", v)),
        )
        .chain(
            params
                .pending_below_price
                .map(|v| ("pendingBelowPrice", v.to_string())),
        )
        .chain(
            params
                .pending_below_stop_price
                .map(|v| ("pendingBelowStopPrice", v.to_string())),
        )
        .chain(
            params
                .pending_below_trailing_delta
                .map(|v| ("pendingBelowTrailingDelta", v.to_string())),
        )
        .chain(
            params
                .pending_below_iceberg_qty
                .map(|v| ("pendingBelowIcebergQty", v.to_string())),
        )
        .chain(
            params
                .pending_below_time_in_force
                .map(|v| ("pendingBelowTimeInForce", v.to_string())),
        )
        .chain(
            params
                .pending_below_strategy_id
                .map(|v| ("pendingBelowStrategyId", v.to_string())),
        )
        .chain(
            params
                .pending_below_strategy_type
                .map(|v| ("pendingBelowStrategyType", v.to_string())),
        )
        .chain(params.recv_window.map(|v| ("recvWindow", v.to_string())))
        .collect();

        let body: Vec<(&str, &str)> = body_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.send_request(
            "/api/v3/orderList/otoco",
            reqwest::Method::POST,
            None,
            Some(&body),
            1,
            true,
        )
        .await
    }
}
