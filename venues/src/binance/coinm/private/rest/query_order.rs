// Query Order endpoint implementation for GET /dapi/v1/order
// See: https://binance-docs.github.io/apidocs/delivery/en/>

use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::coinm::RestResult;
use serde::{Deserialize, Serialize};

/// Request parameters for querying an order (GET /dapi/v1/order).
#[derive(Debug, Clone, Serialize, Default)]
pub struct QueryOrderRequest {
    /// Trading symbol (e.g., "BTCUSD_200925").
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID. Either `orderId` or `origClientOrderId` must be sent.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Either `orderId` or `origClientOrderId` must be sent.
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// The value cannot be greater than 60000.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp (milliseconds since epoch).
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Response for querying an order (GET /dapi/v1/order).
#[derive(Debug, Clone, Deserialize)]
pub struct QueryOrderResponse {
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
    /// Query an order's status on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    /// GET /dapi/v1/order
    /// Weight: 1
    /// Requires API key and signature.
    pub async fn get_query_order(
        &self,
        params: QueryOrderRequest,
    ) -> RestResult<QueryOrderResponse> {
        let weight = 1;
        self.send_signed_request(
            "/dapi/v1/order",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
