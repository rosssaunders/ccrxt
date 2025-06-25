// Request and response structs, and RestClient method for GET /dapi/v1/openOrders
// See: <https://binance-docs.github.io/apidocs/delivery/en/>

use serde::{Deserialize, Serialize};

use crate::binance::coinm::RestResult;
use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::shared;

/// Request parameters for the Current All Open Orders endpoint (GET /dapi/v1/openOrders).
///
/// See: <https://binance-docs.github.io/apidocs/delivery/en/>
#[derive(Debug, Clone, Serialize, Default)]
pub struct OpenOrdersRequest {
    /// The trading symbol (e.g., "BTCUSD_200925").
    /// If not sent, will return orders for all symbols.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// The trading pair (e.g., "BTCUSD").
    /// If not sent, will return orders for all pairs.
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,

    /// The number of milliseconds the request is valid for after timestamp.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds since epoch.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Represents a single open order returned by GET /dapi/v1/openOrders.
#[derive(Debug, Clone, Deserialize)]
pub struct OpenOrder {
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

    #[serde(rename = "price")]
    pub price: String,

    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    #[serde(rename = "side")]
    pub side: String,

    #[serde(rename = "positionSide")]
    pub position_side: String,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "stopPrice")]
    pub stop_price: String,

    #[serde(rename = "closePosition")]
    pub close_position: bool,

    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "time")]
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
    pub price_match: Option<String>,

    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: Option<String>,
}

impl RestClient {
    /// Fetches all open orders for a symbol or all symbols.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    ///
    /// GET /dapi/v1/openOrders
    /// Weight: 1 for single symbol, 40 for multiple symbols
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`OpenOrdersRequest`])
    ///
    /// # Returns
    /// A vector of [`OpenOrder`] objects.
    pub async fn get_open_orders(&self, params: OpenOrdersRequest) -> RestResult<Vec<OpenOrder>> {
        let weight = if params.symbol.is_some() || params.pair.is_some() {
            1
        } else {
            40
        };
        let result = shared::send_signed_request(
            self,
            "/dapi/v1/openOrders",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await?;
        
        Ok(crate::binance::coinm::RestResponse {
            data: result,
            request_duration: std::time::Duration::ZERO,
            headers: crate::binance::coinm::ResponseHeaders::default(),
        })
    }
}
