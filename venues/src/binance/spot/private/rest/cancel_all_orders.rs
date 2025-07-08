use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    OrderSide, OrderStatus, OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

/// Request parameters for cancelling all open orders
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllOrdersRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Cancel all orders response item
#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllOrdersResponseItem {
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
    /// Cancel all active orders on a symbol
    ///
    /// Cancels all active orders on a symbol.
    /// This includes OCO orders.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#cancel-all-open-orders-on-a-symbol--trade)
    /// Method: DELETE /api/v3/openOrders
    /// Weight: 1
    /// Security: TRADE
    pub async fn cancel_all_orders(
        &self,
        params: CancelAllOrdersRequest,
    ) -> RestResult<Vec<CancelAllOrdersResponseItem>> {
        let body_params: Vec<(&str, String)> = vec![("symbol", params.symbol)]
            .into_iter()
            .chain(params.recv_window.map(|v| ("recvWindow", v.to_string())))
            .collect();

        let body: Vec<(&str, &str)> = body_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.send_request(
            "/api/v3/openOrders",
            reqwest::Method::DELETE,
            None,
            Some(&body),
            1,
            true,
        )
        .await
    }
}
