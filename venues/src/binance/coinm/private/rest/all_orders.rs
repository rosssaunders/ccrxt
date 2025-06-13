// All Orders endpoint implementation for GET /dapi/v1/allOrders
// See: https://binance-docs.github.io/apidocs/delivery/en/#all-orders-user_data

use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::coinm::RestResult;
use serde::{Deserialize, Serialize};

/// Request parameters for all orders (GET /dapi/v1/allOrders).
#[derive(Debug, Clone, Serialize, Default)]
pub struct AllOrdersRequest {
    /// Trading symbol (e.g., "BTCUSD_200925"). Optional.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Trading pair (e.g., "BTCUSD"). Optional.
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,

    /// Order ID. Optional.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Start time (ms since epoch). Optional.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time (ms since epoch). Optional.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Limit (default 50, max 100). Optional.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// The value cannot be greater than 60000. Optional.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp (milliseconds since epoch).
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Response for all orders (GET /dapi/v1/allOrders).
#[derive(Debug, Clone, Deserialize)]
pub struct AllOrder {
    #[serde(rename = "avgPrice")]
    pub avg_price: String,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "cumBase")]
    pub cum_base: String,
    #[serde(rename = "executedQty")]
    pub executed_qty: String,
    #[serde(rename = "orderId")]
    pub order_id: u64,
    #[serde(rename = "origQty")]
    pub orig_qty: String,
    #[serde(rename = "origType")]
    pub orig_type: String,
    pub price: String,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    pub side: String,
    #[serde(rename = "positionSide")]
    pub position_side: String,
    pub status: String,
    #[serde(rename = "stopPrice")]
    pub stop_price: String,
    #[serde(rename = "closePosition")]
    pub close_position: bool,
    pub symbol: String,
    pub pair: String,
    pub time: u64,
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub order_type: String,
    #[serde(rename = "activatePrice")]
    pub activate_price: Option<String>,
    #[serde(rename = "priceRate")]
    pub price_rate: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: u64,
    #[serde(rename = "workingType")]
    pub working_type: String,
    #[serde(rename = "priceProtect")]
    pub price_protect: bool,
    #[serde(rename = "priceMatch")]
    pub price_match: String,
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: String,
}

impl RestClient {
    /// Get all account orders (active, canceled, or filled) on Binance Coin-M Futures.
    ///
    /// See: https://binance-docs.github.io/apidocs/delivery/en/#all-orders-user_data
    /// GET /dapi/v1/allOrders
    /// Weight: 20 with symbol, 40 with pair
    /// Requires API key and signature.
    pub async fn get_all_orders(&self, params: AllOrdersRequest) -> RestResult<Vec<AllOrder>> {
        let weight = if params.pair.is_some() { 40 } else { 20 };
        self.send_signed_request(
            "/dapi/v1/allOrders",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
