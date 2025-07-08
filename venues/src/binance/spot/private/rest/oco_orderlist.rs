use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    ContingencyType, OrderListOrderStatus, OrderListStatus, OrderResponseType, OrderSide,
    OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

/// Request parameters for OCO orderList
#[derive(Debug, Clone, Serialize)]
pub struct OcoOrderListRequest {
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

    /// Above order type (STOP_LOSS_LIMIT, LIMIT_MAKER)
    #[serde(rename = "aboveType")]
    pub above_type: OrderType,

    /// Above client order ID
    #[serde(rename = "aboveClientOrderId", skip_serializing_if = "Option::is_none")]
    pub above_client_order_id: Option<String>,

    /// Above iceberg quantity
    #[serde(rename = "aboveIcebergQty", skip_serializing_if = "Option::is_none")]
    pub above_iceberg_qty: Option<Decimal>,

    /// Above price
    #[serde(rename = "abovePrice", skip_serializing_if = "Option::is_none")]
    pub above_price: Option<Decimal>,

    /// Above stop price
    #[serde(rename = "aboveStopPrice", skip_serializing_if = "Option::is_none")]
    pub above_stop_price: Option<Decimal>,

    /// Above trailing delta
    #[serde(rename = "aboveTrailingDelta", skip_serializing_if = "Option::is_none")]
    pub above_trailing_delta: Option<u32>,

    /// Above time in force
    #[serde(rename = "aboveTimeInForce", skip_serializing_if = "Option::is_none")]
    pub above_time_in_force: Option<TimeInForce>,

    /// Above strategy ID
    #[serde(rename = "aboveStrategyId", skip_serializing_if = "Option::is_none")]
    pub above_strategy_id: Option<u32>,

    /// Above strategy type
    #[serde(rename = "aboveStrategyType", skip_serializing_if = "Option::is_none")]
    pub above_strategy_type: Option<u32>,

    /// Below order type (STOP_LOSS_LIMIT, LIMIT_MAKER)
    #[serde(rename = "belowType")]
    pub below_type: OrderType,

    /// Below client order ID
    #[serde(rename = "belowClientOrderId", skip_serializing_if = "Option::is_none")]
    pub below_client_order_id: Option<String>,

    /// Below iceberg quantity
    #[serde(rename = "belowIcebergQty", skip_serializing_if = "Option::is_none")]
    pub below_iceberg_qty: Option<Decimal>,

    /// Below price
    #[serde(rename = "belowPrice", skip_serializing_if = "Option::is_none")]
    pub below_price: Option<Decimal>,

    /// Below stop price
    #[serde(rename = "belowStopPrice", skip_serializing_if = "Option::is_none")]
    pub below_stop_price: Option<Decimal>,

    /// Below trailing delta
    #[serde(rename = "belowTrailingDelta", skip_serializing_if = "Option::is_none")]
    pub below_trailing_delta: Option<u32>,

    /// Below time in force
    #[serde(rename = "belowTimeInForce", skip_serializing_if = "Option::is_none")]
    pub below_time_in_force: Option<TimeInForce>,

    /// Below strategy ID
    #[serde(rename = "belowStrategyId", skip_serializing_if = "Option::is_none")]
    pub below_strategy_id: Option<u32>,

    /// Below strategy type
    #[serde(rename = "belowStrategyType", skip_serializing_if = "Option::is_none")]
    pub below_strategy_type: Option<u32>,

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

/// OCO orderList response (same structure as OcoOrderResponse)
#[derive(Debug, Clone, Deserialize)]
pub struct OcoOrderListResponse {
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
    pub orders: Vec<OcoOrderListOrder>,

    /// Order reports
    #[serde(rename = "orderReports")]
    pub order_reports: Vec<serde_json::Value>,
}

/// OCO orderList order information
#[derive(Debug, Clone, Deserialize)]
pub struct OcoOrderListOrder {
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
    /// Place an OCO orderList
    ///
    /// Place an OCO pair.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#new-oco-orderlist--trade)
    /// Method: POST /api/v3/orderList/oco
    /// Weight: 1
    /// Security: TRADE
    pub async fn new_oco_orderlist(
        &self,
        params: OcoOrderListRequest,
    ) -> RestResult<OcoOrderListResponse> {
        let body_params: Vec<(&str, String)> = vec![
            ("symbol", params.symbol),
            ("side", params.side.to_string()),
            ("quantity", params.quantity.to_string()),
            ("aboveType", params.above_type.to_string()),
            ("belowType", params.below_type.to_string()),
        ]
        .into_iter()
        .chain(
            params
                .list_client_order_id
                .map(|v| ("listClientOrderId", v)),
        )
        .chain(
            params
                .above_client_order_id
                .map(|v| ("aboveClientOrderId", v)),
        )
        .chain(
            params
                .above_iceberg_qty
                .map(|v| ("aboveIcebergQty", v.to_string())),
        )
        .chain(params.above_price.map(|v| ("abovePrice", v.to_string())))
        .chain(
            params
                .above_stop_price
                .map(|v| ("aboveStopPrice", v.to_string())),
        )
        .chain(
            params
                .above_trailing_delta
                .map(|v| ("aboveTrailingDelta", v.to_string())),
        )
        .chain(
            params
                .above_time_in_force
                .map(|v| ("aboveTimeInForce", v.to_string())),
        )
        .chain(
            params
                .above_strategy_id
                .map(|v| ("aboveStrategyId", v.to_string())),
        )
        .chain(
            params
                .above_strategy_type
                .map(|v| ("aboveStrategyType", v.to_string())),
        )
        .chain(
            params
                .below_client_order_id
                .map(|v| ("belowClientOrderId", v)),
        )
        .chain(
            params
                .below_iceberg_qty
                .map(|v| ("belowIcebergQty", v.to_string())),
        )
        .chain(params.below_price.map(|v| ("belowPrice", v.to_string())))
        .chain(
            params
                .below_stop_price
                .map(|v| ("belowStopPrice", v.to_string())),
        )
        .chain(
            params
                .below_trailing_delta
                .map(|v| ("belowTrailingDelta", v.to_string())),
        )
        .chain(
            params
                .below_time_in_force
                .map(|v| ("belowTimeInForce", v.to_string())),
        )
        .chain(
            params
                .below_strategy_id
                .map(|v| ("belowStrategyId", v.to_string())),
        )
        .chain(
            params
                .below_strategy_type
                .map(|v| ("belowStrategyType", v.to_string())),
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
            "/api/v3/orderList/oco",
            reqwest::Method::POST,
            None,
            Some(&body),
            1,
            true,
        )
        .await
    }
}
