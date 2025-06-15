//! Order endpoints for Binance Options Private API

use serde::{Deserialize, Serialize};

/// Request parameters for placing a new order
#[derive(Debug, Clone, Serialize)]
pub struct NewOrderRequest {
    /// Option trading pair, e.g. BTC-200730-9000-C
    pub symbol: String,
    /// Buy/sell direction: SELL, BUY  
    pub side: crate::binance::options::OrderSide,
    /// Order Type: LIMIT (only support limit)
    #[serde(rename = "type")]
    pub order_type: crate::binance::options::OptionsOrderType,
    /// Order Quantity
    pub quantity: String,
    /// Order Price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Time in force method (Default GTC)
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<crate::binance::options::TimeInForce>,
    /// Reduce Only (Default false)
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Post Only (Default false)
    #[serde(rename = "postOnly", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    /// "ACK", "RESULT", Default "ACK"
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<crate::binance::options::OrderResponseType>,
    /// User-defined order ID cannot be repeated in pending orders
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// Is market maker protection order, true/false
    #[serde(rename = "isMmp", skip_serializing_if = "Option::is_none")]
    pub is_mmp: Option<bool>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

impl NewOrderRequest {
    pub fn new(
        symbol: String,
        side: crate::binance::options::OrderSide,
        order_type: crate::binance::options::OptionsOrderType,
        quantity: String,
    ) -> Self {
        Self {
            symbol,
            side,
            order_type,
            quantity,
            price: None,
            time_in_force: None,
            reduce_only: None,
            post_only: None,
            new_order_resp_type: None,
            client_order_id: None,
            is_mmp: None,
            recv_window: None,
        }
    }

    pub fn price(mut self, price: String) -> Self {
        self.price = Some(price);
        self
    }

    pub fn time_in_force(mut self, time_in_force: crate::binance::options::TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }

    pub fn reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    pub fn post_only(mut self, post_only: bool) -> Self {
        self.post_only = Some(post_only);
        self
    }

    pub fn new_order_resp_type(mut self, resp_type: crate::binance::options::OrderResponseType) -> Self {
        self.new_order_resp_type = Some(resp_type);
        self
    }

    pub fn client_order_id(mut self, client_order_id: String) -> Self {
        self.client_order_id = Some(client_order_id);
        self
    }

    pub fn is_mmp(mut self, is_mmp: bool) -> Self {
        self.is_mmp = Some(is_mmp);
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

/// Response for new order (ACK type)
#[derive(Debug, Clone, Deserialize)]
pub struct NewOrderAckResponse {
    /// System order number
    #[serde(rename = "orderId")]
    pub order_id: u64,
    /// Option trading pair
    pub symbol: String,
    /// Order Price
    pub price: String,
    /// Order Quantity
    pub quantity: String,
    /// Buy/sell direction
    pub side: String,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: String,
    /// Order Time
    #[serde(rename = "createDate")]
    pub create_date: u64,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "postOnly")]
    pub post_only: bool,
    pub mmp: bool,
}

/// Response for new order (RESULT type)
#[derive(Debug, Clone, Deserialize)]
pub struct NewOrderResultResponse {
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
    /// Order is post only
    #[serde(rename = "postOnly")]
    pub post_only: bool,
    /// Order Time
    #[serde(rename = "createTime")]
    pub create_time: u64,
    /// Update time
    #[serde(rename = "updateTime")]
    pub update_time: u64,
    /// Order status
    pub status: String,
    /// Average price of completed trade
    #[serde(rename = "avgPrice")]
    pub avg_price: String,
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
    /// Place a new order
    ///
    /// # Arguments
    /// * `request` - New order request parameters
    ///
    /// # Returns
    /// Order confirmation (ACK or RESULT based on newOrderRespType)
    ///
    /// # Weight
    /// 1
    pub async fn new_order_ack(&self, request: NewOrderRequest) -> RestResult<NewOrderAckResponse> {
        self.send_signed_request(
            "/eapi/v1/order",
            reqwest::Method::POST,
            request,
            1, // weight
            true, // is order
        )
        .await
    }

    /// Place a new order with RESULT response type
    pub async fn new_order_result(&self, request: NewOrderRequest) -> RestResult<NewOrderResultResponse> {
        self.send_signed_request(
            "/eapi/v1/order",
            reqwest::Method::POST,
            request,
            1, // weight
            true, // is order
        )
        .await
    }
}