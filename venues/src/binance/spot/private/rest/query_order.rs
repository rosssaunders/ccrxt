use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::{
    OrderSide, OrderStatus, OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

use super::client::RestClient;

/// Request parameters for querying an order
#[derive(Debug, Clone, Serialize)]
pub struct QueryOrderRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Query order response
#[derive(Debug, Clone, Deserialize)]
pub struct QueryOrderResponse {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Order list ID
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,

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

    /// Working time
    #[serde(rename = "workingTime")]
    pub working_time: u64,

    /// Original quote order quantity
    #[serde(rename = "origQuoteOrderQty")]
    pub orig_quote_order_qty: Decimal,

    /// Self-trade prevention mode
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}

impl RestClient {
    /// Check an order's status
    ///
    /// Check an order's status.
    /// Either orderId or origClientOrderId must be sent.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#query-order--user_data)
    /// Method: GET /api/v3/order
    /// Weight: 4
    /// Security: USER_DATA
    pub async fn query_order(&self, params: QueryOrderRequest) -> RestResult<QueryOrderResponse> {
        let query_string = serde_urlencoded::to_string(&params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {e}")))?;

        self.send_request(
            "/api/v3/order",
            reqwest::Method::GET,
            Some(&query_string),
            None,
            4,
            false,
        )
        .await
    }
}
