use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    ContingencyType, OrderListOrderStatus, OrderListStatus, OrderSide, OrderStatus, OrderType,
    RestResult, SelfTradePreventionMode, TimeInForce,
};

/// Request parameters for getting open order lists
#[derive(Debug, Clone, Serialize, Default)]
pub struct OpenOrderListRequest {
    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Open order list information
#[derive(Debug, Clone, Deserialize)]
pub struct OpenOrderList {
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
    pub orders: Vec<OpenOrderListOrder>,
}

/// Order information in the open order list
#[derive(Debug, Clone, Deserialize)]
pub struct OpenOrderListOrder {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

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

    /// Order creation time
    #[serde(rename = "time")]
    pub time: u64,

    /// Last update time
    #[serde(rename = "updateTime")]
    pub update_time: u64,

    /// Is working (true if the order is active)
    #[serde(rename = "isWorking")]
    pub is_working: bool,

    /// Original quote order quantity
    #[serde(rename = "origQuoteOrderQty")]
    pub orig_quote_order_qty: Decimal,

    /// Self-trade prevention mode
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}

impl RestClient {
    /// Get all open order lists
    ///
    /// Get all open order lists.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#query-open-order-lists--user_data)
    /// Method: GET /api/v3/openOrderList
    /// Weight: 6
    /// Security: USER_DATA
    pub async fn get_open_order_lists(
        &self,
        params: Option<OpenOrderListRequest>,
    ) -> RestResult<Vec<OpenOrderList>> {
        let query_string = if let Some(p) = params {
            if p.recv_window.is_some() {
                Some(serde_urlencoded::to_string(&p).map_err(|e| {
                    crate::binance::spot::Errors::Error(format!("URL encoding error: {e}"))
                })?)
            } else {
                None
            }
        } else {
            None
        };

        self.send_request(
            "/api/v3/openOrderList",
            reqwest::Method::GET,
            query_string.as_deref(),
            None,
            6,
            false,
        )
        .await
    }
}
