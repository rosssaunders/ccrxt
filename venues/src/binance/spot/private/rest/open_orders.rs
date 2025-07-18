use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    OrderSide, OrderStatus, OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

/// Request parameters for getting open orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct OpenOrdersRequest {
    /// Trading pair symbol (optional - if not provided, returns all open orders)
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Open order information
#[derive(Debug, Clone, Deserialize)]
pub struct OpenOrder {
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
    /// Get all open orders on a symbol or all symbols
    ///
    /// Get all open orders on a symbol. Careful when accessing this with no symbol.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#current-open-orders--user_data)
    /// Method: GET /api/v3/openOrders
    /// Weight: 6 (for one symbol), 80 (for all symbols)
    /// Security: USER_DATA
    pub async fn get_open_orders(
        &self,
        params: Option<OpenOrdersRequest>,
    ) -> RestResult<Vec<OpenOrder>> {
        let (query_string, weight) = if let Some(p) = params {
            let weight = if p.symbol.is_some() { 6 } else { 80 };

            let qs = if p.symbol.is_some() || p.recv_window.is_some() {
                Some(serde_urlencoded::to_string(&p).map_err(|e| {
                    crate::binance::spot::Errors::Error(format!("URL encoding error: {e}"))
                })?)
            } else {
                None
            };

            (qs, weight)
        } else {
            (None, 80) // All symbols
        };

        self.send_request(
            "/api/v3/openOrders",
            reqwest::Method::GET,
            query_string.as_deref(),
            None,
            weight,
            false,
        )
        .await
    }
}
