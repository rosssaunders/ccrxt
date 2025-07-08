use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::{
    options::{
        OptionsContractType, OptionsOrderSide, OptionsOrderStatus, OptionsOrderType,
        OptionsTimeInForce, RestResult,
    },
    shared,
};

/// Single order in batch request
#[derive(Debug, Clone, Serialize)]
pub struct BatchOrderItem {
    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Buy/sell direction
    #[serde(rename = "side")]
    pub side: OptionsOrderSide,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OptionsOrderType,

    /// Order quantity
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Order price
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,

    /// Time in force method
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<OptionsTimeInForce>,

    /// Reduce only flag
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Post only flag
    #[serde(rename = "postOnly", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,

    /// User-defined order ID
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Self-trade prevention mode
    #[serde(rename = "isMmp", skip_serializing_if = "Option::is_none")]
    pub is_mmp: Option<bool>,
}

/// Request parameters for batch orders
#[derive(Debug, Clone, Serialize)]
pub struct BatchOrdersRequest {
    /// Array of orders (max 5 orders)
    #[serde(rename = "orders")]
    pub orders: Vec<BatchOrderItem>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Response for single order in batch
#[derive(Debug, Clone, Deserialize)]
pub struct BatchOrderResponse {
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
    /// Place batch orders
    ///
    /// Places multiple orders in a single request. Maximum 5 orders per request.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/trade/Option-Batch-Orders)
    /// Method: POST /eapi/v1/batchOrders
    /// Weight: 5
    /// Requires: API key and signature
    pub async fn batch_orders(
        &self,
        params: BatchOrdersRequest,
    ) -> RestResult<Vec<BatchOrderResponse>> {
        shared::send_signed_request(
            self,
            "/eapi/v1/batchOrders",
            reqwest::Method::POST,
            params,
            5,
            true, // is_order = true for order endpoints
        )
        .await
    }
}
