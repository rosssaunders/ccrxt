use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::options::PrivateRestClient as RestClient;
use crate::binance::options::{
    OptionsContractType, OptionsOrderSide, OptionsOrderStatus, OptionsOrderType,
    OptionsTimeInForce, RestResult,
};

const GET_HISTORY_ORDERS_ENDPOINT: &str = "/eapi/v1/historyOrders";

/// Request parameters for querying history orders
#[derive(Debug, Clone, Serialize)]
pub struct HistoryOrdersRequest {
    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Returns orders with order ID >= this value (most recent by default)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Start time (within 5 days from end time)
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of result sets returned (default: 100, max: 1000)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Historical order information
#[derive(Debug, Clone, Deserialize)]
pub struct HistoryOrder {
    /// System order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Order quantity
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Number of executed quantity
    #[serde(rename = "executedQty")]
    pub executed_qty: Decimal,

    /// Fee amount
    #[serde(rename = "fee")]
    pub fee: Decimal,

    /// Buy/sell direction
    #[serde(rename = "side")]
    pub side: OptionsOrderSide,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OptionsOrderType,

    /// Time in force method
    #[serde(rename = "timeInForce")]
    pub time_in_force: OptionsTimeInForce,

    /// Reduce only flag
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    /// Post only flag
    #[serde(rename = "postOnly")]
    pub post_only: bool,

    /// Order creation time
    #[serde(rename = "createTime")]
    pub create_time: u64,

    /// Order update time
    #[serde(rename = "updateTime")]
    pub update_time: u64,

    /// Order status
    #[serde(rename = "status")]
    pub status: OptionsOrderStatus,

    /// Average price of completed trades
    #[serde(rename = "avgPrice")]
    pub avg_price: Decimal,

    /// Source of the order
    #[serde(rename = "source")]
    pub source: String,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Price scale
    #[serde(rename = "priceScale")]
    pub price_scale: u32,

    /// Quantity scale
    #[serde(rename = "quantityScale")]
    pub quantity_scale: u32,

    /// Option side (CALL or PUT)
    #[serde(rename = "optionSide")]
    pub option_side: OptionsContractType,

    /// Quote asset
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,

    /// Market maker protection flag
    #[serde(rename = "mmp")]
    pub mmp: bool,
}

impl RestClient {
    /// Query all finished orders within 5 days
    ///
    /// Returns historical orders that have been filled or canceled within the last 5 days.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/trade/Query-All-Finished-Orders-Within-5-Days)
    ///
    /// Method: GET /eapi/v1/historyOrders
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn get_history_orders(
        &self,
        params: HistoryOrdersRequest,
    ) -> RestResult<Vec<HistoryOrder>> {
        self.send_get_signed_request(GET_HISTORY_ORDERS_ENDPOINT, params, 1, false)
            .await
    }
}
