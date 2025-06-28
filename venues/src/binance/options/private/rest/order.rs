use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::options::{
    OptionsContractType, OptionsOrderResponseType, OptionsOrderSide, OptionsOrderStatus,
    OptionsOrderType, OptionsTimeInForce, RestResult,
};
use crate::binance::shared;

use super::client::RestClient;

/// Request parameters for placing a new order
#[derive(Debug, Clone, Serialize)]
pub struct NewOrderRequest {
    /// Option trading pair (e.g., "BTC-200730-9000-C")
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Buy/sell direction
    #[serde(rename = "side")]
    pub side: OptionsOrderSide,

    /// Order type (currently only LIMIT is supported)
    #[serde(rename = "type")]
    pub order_type: OptionsOrderType,

    /// Order quantity
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Order price (required for LIMIT orders)
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,

    /// Time in force method (default: GTC)
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<OptionsTimeInForce>,

    /// Reduce only (default: false)
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Post only (default: false)
    #[serde(rename = "postOnly", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,

    /// Response type: "ACK" or "RESULT" (default: "ACK")
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OptionsOrderResponseType>,

    /// User-defined order ID (cannot be repeated in pending orders)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Market maker protection order flag
    #[serde(rename = "isMmp", skip_serializing_if = "Option::is_none")]
    pub is_mmp: Option<bool>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Order response (ACK type)
#[derive(Debug, Clone, Deserialize)]
pub struct OrderAckResponse {
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

    /// Buy/sell direction
    #[serde(rename = "side")]
    pub side: OptionsOrderSide,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OptionsOrderType,

    /// Order creation time
    #[serde(rename = "createDate")]
    pub create_date: u64,

    /// Reduce only flag
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    /// Post only flag
    #[serde(rename = "postOnly")]
    pub post_only: bool,

    /// Market maker protection flag
    #[serde(rename = "mmp")]
    pub mmp: bool,
}

/// Order response (RESULT type)
#[derive(Debug, Clone, Deserialize)]
pub struct OrderResultResponse {
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
    /// Place a new option order
    ///
    /// Places a new limit order for options contracts. Currently only LIMIT orders are supported.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/trade)
    /// Method: POST /eapi/v1/order
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn new_order(&self, params: NewOrderRequest) -> RestResult<OrderAckResponse> {
        shared::send_signed_request(
            self,
            "/eapi/v1/order",
            reqwest::Method::POST,
            params,
            1,
            true,
        )
        .await
    }

    /// Place a new option order with RESULT response
    ///
    /// Places a new limit order for options contracts and returns detailed execution information.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/trade)
    /// Method: POST /eapi/v1/order
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn new_order_result(
        &self,
        mut params: NewOrderRequest,
    ) -> RestResult<OrderResultResponse> {
        // Force response type to RESULT
        params.new_order_resp_type = Some(OptionsOrderResponseType::Result);

        shared::send_signed_request(
            self,
            "/eapi/v1/order",
            reqwest::Method::POST,
            params,
            1,
            true,
        )
        .await
    }
}
