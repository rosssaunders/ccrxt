use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::options::{
    OptionsContractType, OptionsOrderSide, OptionsOrderStatus, OptionsOrderType,
    OptionsTimeInForce, RestResult, private_client::RestClient,
};

const QUERY_ORDER_ENDPOINT: &str = "/eapi/v1/order";

/// Request parameters for querying a single order
#[derive(Debug, Clone, Serialize)]
pub struct QueryOrderRequest {
    /// Option trading pair (e.g., "BTC-200730-9000-C")
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID (either this or client_order_id must be provided)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// User-defined order ID (either this or order_id must be provided)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Query order response
#[derive(Debug, Clone, Deserialize)]
pub struct QueryOrderResponse {
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

    /// Fee
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

    /// Order source
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
    /// Query a single order status
    ///
    /// Check the status of an order. Either order_id or client_order_id must be provided.
    /// Orders that are CANCELED or REJECTED with no filled trades and created more than
    /// 3 days ago will not be found.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/trade/Query-Single-Order)
    ///
    /// Method: GET /eapi/v1/order
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn query_order(&self, params: QueryOrderRequest) -> RestResult<QueryOrderResponse> {
        // Validate that either order_id or client_order_id is provided
        if params.order_id.is_none() && params.client_order_id.is_none() {
            return Err(crate::binance::options::Errors::Error(
                "Either order_id or client_order_id must be provided".to_string(),
            ));
        }

        self.send_get_signed_request(QUERY_ORDER_ENDPOINT, params, 1, false)
            .await
    }
}
