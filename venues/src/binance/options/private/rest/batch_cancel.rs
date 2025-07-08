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

/// Request parameters for batch cancel orders
#[derive(Debug, Clone, Serialize)]
pub struct BatchCancelRequest {
    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order IDs to cancel (max 10)
    #[serde(rename = "orderIdList", skip_serializing_if = "Option::is_none")]
    pub order_id_list: Option<Vec<u64>>,

    /// Client order IDs to cancel (max 10)
    #[serde(
        rename = "origClientOrderIdList",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_client_order_id_list: Option<Vec<String>>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Request parameters for canceling all orders by underlying
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllByUnderlyingRequest {
    /// Underlying asset (e.g., "BTCUSDT")
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Request parameters for canceling all orders by symbol
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllBySymbolRequest {
    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Response for canceled order
#[derive(Debug, Clone, Deserialize)]
pub struct CancelResponse {
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
    /// Cancel multiple option orders
    ///
    /// Cancels multiple orders in a single request. Maximum 10 orders per request.
    /// Either orderIdList or origClientOrderIdList must be provided.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/trade/Cancel-Multiple-Option-Orders)
    /// Method: DELETE /eapi/v1/batchOrders
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn batch_cancel_orders(
        &self,
        params: BatchCancelRequest,
    ) -> RestResult<Vec<CancelResponse>> {
        shared::send_signed_request(
            self,
            "/eapi/v1/batchOrders",
            reqwest::Method::DELETE,
            params,
            1,
            true, // is_order = true for order endpoints
        )
        .await
    }

    /// Cancel all active orders on specified underlying
    ///
    /// Cancels all open orders for the specified underlying asset.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/trade/Cancel-all-Active-Orders-on-Specified-Underlying)
    /// Method: DELETE /eapi/v1/allOpenOrdersByUnderlying
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn cancel_all_orders_by_underlying(
        &self,
        params: CancelAllByUnderlyingRequest,
    ) -> RestResult<Vec<CancelResponse>> {
        shared::send_signed_request(
            self,
            "/eapi/v1/allOpenOrdersByUnderlying",
            reqwest::Method::DELETE,
            params,
            1,
            true, // is_order = true for order endpoints
        )
        .await
    }

    /// Cancel all active orders on a symbol
    ///
    /// Cancels all open orders for the specified symbol.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/trade/Cancel-all-Active-Orders-on-a-Symbol)
    /// Method: DELETE /eapi/v1/allOpenOrders
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn cancel_all_orders_by_symbol(
        &self,
        params: CancelAllBySymbolRequest,
    ) -> RestResult<Vec<CancelResponse>> {
        shared::send_signed_request(
            self,
            "/eapi/v1/allOpenOrders",
            reqwest::Method::DELETE,
            params,
            1,
            true, // is_order = true for order endpoints
        )
        .await
    }
}
