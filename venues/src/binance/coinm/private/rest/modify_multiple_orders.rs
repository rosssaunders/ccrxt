// Modify Multiple Orders (TRADE) endpoint implementation for PUT /dapi/v1/batchOrders
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Multiple-Orders>

use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{
        OrderSide, PriceMatch, RestResult,
        private::rest::{client::RestClient, modify_order::ModifyOrderResponse},
    },
    shared,
};

/// Single order parameters for batch modify operation.
#[derive(Debug, Clone, Serialize)]
pub struct BatchModifyOrderItem {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Order side (BUY or SELL).
    pub side: OrderSide,

    /// Order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// New order quantity. Either quantity or price must be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,

    /// New order price. Either quantity or price must be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Price match mode. Only available for LIMIT/STOP/TAKE_PROFIT orders.
    /// Cannot be passed together with price.
    #[serde(rename = "priceMatch", skip_serializing_if = "Option::is_none")]
    pub price_match: Option<PriceMatch>,
}

/// Request parameters for modifying multiple orders (PUT /dapi/v1/batchOrders).
#[derive(Debug, Clone, Serialize, Default)]
pub struct ModifyMultipleOrdersRequest {
    /// List of orders to modify. Maximum 5 orders.
    #[serde(rename = "batchOrders")]
    pub batch_orders: Vec<BatchModifyOrderItem>,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Error response for a failed order modification in a batch.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchModifyOrderError {
    /// Error code.
    pub code: i32,

    /// Error message.
    pub msg: String,
}

/// Response item for batch modify orders - either success or error.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum BatchModifyOrderResponseItem {
    /// Successful order modification.
    Success(ModifyOrderResponse),
    /// Failed order modification.
    Error(BatchModifyOrderError),
}

/// Response for modifying multiple orders (PUT /dapi/v1/batchOrders).
pub type ModifyMultipleOrdersResponse = Vec<BatchModifyOrderResponseItem>;

impl RestClient {
    /// Modifies multiple orders (TRADE) on Binance Coin-M Futures.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Multiple-Orders>
    /// PUT /dapi/v1/batchOrders
    /// Weight: 5
    /// Requires API key and signature.
    ///
    /// Parameter rules are same as single order modification.
    /// Batch modify orders are processed concurrently, and the order of matching is not guaranteed.
    /// The order of returned contents for batch modify orders is the same as the order of the order list.
    /// One order can only be modified for less than 10000 times.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`ModifyMultipleOrdersRequest`])
    ///
    /// # Returns
    /// A [`ModifyMultipleOrdersResponse`] - array of order results or errors.
    pub async fn modify_multiple_orders(
        &self,
        params: ModifyMultipleOrdersRequest,
    ) -> RestResult<ModifyMultipleOrdersResponse> {
        let weight = 5;
        shared::send_signed_request(
            self,
            "/dapi/v1/batchOrders",
            reqwest::Method::PUT,
            params,
            weight,
            true,
        )
        .await
    }
}
