// User's Force Orders (USER_DATA) endpoint implementation for GET /dapi/v1/forceOrders
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Users-Force-Orders>

use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{
        AutoCloseType, OrderSide, OrderStatus, OrderType, PositionSide, RestResult, TimeInForce,
        WorkingType, private::rest::client::RestClient,
    },
    shared,
};

/// Request parameters for getting user's force orders (GET /dapi/v1/forceOrders).
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetForceOrdersRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP"). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Auto close type. "LIQUIDATION" for liquidation orders, "ADL" for ADL orders. Optional.
    #[serde(rename = "autoCloseType", skip_serializing_if = "Option::is_none")]
    pub auto_close_type: Option<AutoCloseType>,

    /// Start time in milliseconds. Optional.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds. Optional.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of records to return. Default 50; max 100. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Individual force order entry.
#[derive(Debug, Clone, Deserialize)]
pub struct ForceOrder {
    /// Order ID.
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Trading symbol.
    pub symbol: String,

    /// Trading pair.
    pub pair: String,

    /// Order status.
    pub status: OrderStatus,

    /// Client order ID.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Order price.
    pub price: String,

    /// Average price.
    #[serde(rename = "avgPrice")]
    pub avg_price: String,

    /// Original quantity.
    #[serde(rename = "origQty")]
    pub orig_qty: String,

    /// Executed quantity.
    #[serde(rename = "executedQty")]
    pub executed_qty: String,

    /// Cumulative base quantity.
    #[serde(rename = "cumBase")]
    pub cum_base: String,

    /// Time in force.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    /// Order type.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Reduce only flag.
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    /// Close position flag.
    #[serde(rename = "closePosition")]
    pub close_position: bool,

    /// Order side.
    pub side: OrderSide,

    /// Position side.
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,

    /// Stop price.
    #[serde(rename = "stopPrice")]
    pub stop_price: String,

    /// Working type.
    #[serde(rename = "workingType")]
    pub working_type: WorkingType,

    /// Price protect flag.
    #[serde(rename = "priceProtect")]
    pub price_protect: bool,

    /// Original order type.
    #[serde(rename = "origType")]
    pub orig_type: OrderType,

    /// Order time.
    pub time: u64,

    /// Update time.
    #[serde(rename = "updateTime")]
    pub update_time: u64,
}

/// Response for getting user's force orders (GET /dapi/v1/forceOrders).
pub type GetForceOrdersResponse = Vec<ForceOrder>;

impl RestClient {
    /// Gets user's force orders (USER_DATA) on Binance Coin-M Futures.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Users-Force-Orders>
    /// GET /dapi/v1/forceOrders
    /// Weight: 20 with symbol, 50 without symbol
    /// Requires API key and signature.
    ///
    /// If "autoCloseType" is not sent, orders with both of the types will be returned.
    /// If "startTime" is not sent, data within 200 days before "endTime" can be queried.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetForceOrdersRequest`])
    ///
    /// # Returns
    /// A [`GetForceOrdersResponse`] - array of force order entries.
    pub async fn get_force_orders(
        &self,
        params: GetForceOrdersRequest,
    ) -> RestResult<GetForceOrdersResponse> {
        let weight = if params.symbol.is_some() { 20 } else { 50 };
        shared::send_signed_request(
            self,
            "/dapi/v1/forceOrders",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
