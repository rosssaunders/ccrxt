//! Cancel order endpoint for Binance Options Private API

use serde::{Deserialize, Serialize};

/// Request parameters for canceling an order
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    /// Option trading pair, e.g. BTC-200730-9000-C
    pub symbol: String,
    /// Order ID
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    /// User-defined order ID
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

impl CancelOrderRequest {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            order_id: None,
            client_order_id: None,
            recv_window: None,
        }
    }

    pub fn order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn client_order_id(mut self, client_order_id: String) -> Self {
        self.client_order_id = Some(client_order_id);
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

/// Response for cancel order
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderResponse {
    /// System order number
    #[serde(rename = "orderId")]
    pub order_id: u64,
    /// Option trading pair
    pub symbol: String,
    /// Order Price
    pub price: String,
    /// Order Quantity
    pub quantity: String,
    /// Number of executed quantity
    #[serde(rename = "executedQty")]
    pub executed_qty: String,
    /// Fee
    pub fee: String,
    /// Buy/sell direction
    pub side: String,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: String,
    /// Time in force method
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    /// Order is reduce only Y/N
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "postOnly")]
    pub post_only: bool,
    /// Order Time
    #[serde(rename = "createDate")]
    pub create_date: u64,
    /// Update time
    #[serde(rename = "updateTime")]
    pub update_time: u64,
    /// Order status
    pub status: String,
    /// Average price of completed trade
    #[serde(rename = "avgPrice")]
    pub avg_price: String,
    pub source: String,
    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "priceScale")]
    pub price_scale: u32,
    #[serde(rename = "quantityScale")]
    pub quantity_scale: u32,
    #[serde(rename = "optionSide")]
    pub option_side: String,
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
    pub mmp: bool,
}

use crate::binance::options::{PrivateRestClient, RestResult};

impl PrivateRestClient {
    /// Cancel an active order
    ///
    /// # Arguments
    /// * `request` - Cancel order request parameters
    ///
    /// # Returns
    /// Cancelled order details
    ///
    /// # Weight
    /// 1
    pub async fn cancel_order(&self, request: CancelOrderRequest) -> RestResult<CancelOrderResponse> {
        self.send_signed_request(
            "/eapi/v1/order",
            reqwest::Method::DELETE,
            request,
            1, // weight
            true, // is order
        )
        .await
    }
}