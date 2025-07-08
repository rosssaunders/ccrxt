use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::{
    options::{OptionsContractType, OptionsOrderSide, RestResult},
    shared,
};

/// Request parameters for creating a block trade order
#[derive(Debug, Clone, Serialize)]
pub struct CreateBlockTradeRequest {
    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Buy/sell direction
    #[serde(rename = "side")]
    pub side: OptionsOrderSide,

    /// Order quantity
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Order price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Counterparty ID
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: u64,

    /// User-defined order ID
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Request parameters for canceling a block trade order
#[derive(Debug, Clone, Serialize)]
pub struct CancelBlockTradeRequest {
    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Block trade order ID (either this or client_order_id required)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Client order ID (either this or order_id required)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Request parameters for extending block trade expire time
#[derive(Debug, Clone, Serialize)]
pub struct ExtendBlockTradeRequest {
    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Block trade order ID (either this or client_order_id required)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Client order ID (either this or order_id required)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Request parameters for querying block trade orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct QueryBlockTradeOrdersRequest {
    /// Option trading pair (if omitted, returns all symbols)
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Returns orders with order ID >= this value (most recent by default)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Start time
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

/// Request parameters for executing a block trade order
#[derive(Debug, Clone, Serialize)]
pub struct ExecuteBlockTradeRequest {
    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Block trade order ID (either this or client_order_id required)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Client order ID (either this or order_id required)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Request parameters for querying block trade execution details
#[derive(Debug, Clone, Serialize)]
pub struct QueryBlockTradeExecutionRequest {
    /// Option trading pair (if omitted, returns all symbols)
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Trade ID to start from (returns trades with ID >= this value)
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Start time
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

/// Request parameters for querying block user trades
#[derive(Debug, Clone, Serialize)]
pub struct QueryBlockUserTradesRequest {
    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Trade ID to start from (returns trades with ID >= this value)
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Start time
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

/// Block trade order response
#[derive(Debug, Clone, Deserialize)]
pub struct BlockTradeOrderResponse {
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

    /// Order creation time
    #[serde(rename = "createTime")]
    pub create_time: u64,

    /// Order update time
    #[serde(rename = "updateTime")]
    pub update_time: u64,

    /// Block trade status
    #[serde(rename = "status")]
    pub status: String,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Option side (CALL or PUT)
    #[serde(rename = "optionSide")]
    pub option_side: OptionsContractType,

    /// Quote asset
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,

    /// Counterparty ID
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: u64,

    /// Order expire time
    #[serde(rename = "expireTime")]
    pub expire_time: u64,
}

/// Block trade execution details
#[derive(Debug, Clone, Deserialize)]
pub struct BlockTradeExecution {
    /// Trade ID
    #[serde(rename = "id")]
    pub id: u64,

    /// Trade ID (same as id)
    #[serde(rename = "tradeId")]
    pub trade_id: u64,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Trade price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Trade quantity
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Quote quantity
    #[serde(rename = "quoteQty")]
    pub quote_qty: Decimal,

    /// Buy/sell direction
    #[serde(rename = "side")]
    pub side: OptionsOrderSide,

    /// Fee amount
    #[serde(rename = "fee")]
    pub fee: Decimal,

    /// Trade time
    #[serde(rename = "time")]
    pub time: u64,

    /// Option side (CALL or PUT)
    #[serde(rename = "optionSide")]
    pub option_side: OptionsContractType,

    /// Quote asset
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
}

impl RestClient {
    /// Send in a new block trade order
    ///
    /// Creates a new block trade order for institutional trading.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/block-trade/Send-in-a-New-Block-Trade-Order)
    /// Method: POST /eapi/v1/block/order/create
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn create_block_trade_order(
        &self,
        params: CreateBlockTradeRequest,
    ) -> RestResult<BlockTradeOrderResponse> {
        shared::send_signed_request(
            self,
            "/eapi/v1/block/order/create",
            reqwest::Method::POST,
            params,
            1,
            true, // is_order = true for order endpoints
        )
        .await
    }

    /// Cancel a block trade order
    ///
    /// Cancels an existing block trade order.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/block-trade/Cancel-a-Block-Trade-Order)
    /// Method: DELETE /eapi/v1/block/order/create
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn cancel_block_trade_order(
        &self,
        params: CancelBlockTradeRequest,
    ) -> RestResult<BlockTradeOrderResponse> {
        shared::send_signed_request(
            self,
            "/eapi/v1/block/order/create",
            reqwest::Method::DELETE,
            params,
            1,
            true, // is_order = true for order endpoints
        )
        .await
    }

    /// Extend block trade expire time
    ///
    /// Extends the expiration time of a block trade order.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/block-trade/Extend-Block-Trade-Expire-Time)
    /// Method: PUT /eapi/v1/block/order/create
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn extend_block_trade_expire_time(
        &self,
        params: ExtendBlockTradeRequest,
    ) -> RestResult<BlockTradeOrderResponse> {
        shared::send_signed_request(
            self,
            "/eapi/v1/block/order/create",
            reqwest::Method::PUT,
            params,
            1,
            true, // is_order = true for order endpoints
        )
        .await
    }

    /// Check block trade order status
    ///
    /// Queries the status of block trade orders.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/block-trade/Check-Block-Trade-Order-Status)
    /// Method: GET /eapi/v1/block/order/orders
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn get_block_trade_orders(
        &self,
        params: QueryBlockTradeOrdersRequest,
    ) -> RestResult<Vec<BlockTradeOrderResponse>> {
        shared::send_signed_request(
            self,
            "/eapi/v1/block/order/orders",
            reqwest::Method::GET,
            params,
            1,
            false,
        )
        .await
    }

    /// Accept a block trade order
    ///
    /// Executes (accepts) a block trade order.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/block-trade/Accept-a-Block-Trade-Order)
    /// Method: POST /eapi/v1/block/order/execute
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn execute_block_trade_order(
        &self,
        params: ExecuteBlockTradeRequest,
    ) -> RestResult<BlockTradeExecution> {
        shared::send_signed_request(
            self,
            "/eapi/v1/block/order/execute",
            reqwest::Method::POST,
            params,
            1,
            true, // is_order = true for order endpoints
        )
        .await
    }

    /// Query block trade details
    ///
    /// Returns details of block trade executions.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/block-trade/Query-Block-Trade-Details)
    /// Method: GET /eapi/v1/block/order/execute
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn get_block_trade_execution_details(
        &self,
        params: QueryBlockTradeExecutionRequest,
    ) -> RestResult<Vec<BlockTradeExecution>> {
        shared::send_signed_request(
            self,
            "/eapi/v1/block/order/execute",
            reqwest::Method::GET,
            params,
            1,
            false,
        )
        .await
    }

    /// Get block trades for a specific account
    ///
    /// Returns block trade history for the current account.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/block-trade/Get-Block-Trades-for-a-Specific-Account)
    /// Method: GET /eapi/v1/block/user-trades
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn get_block_user_trades(
        &self,
        params: QueryBlockUserTradesRequest,
    ) -> RestResult<Vec<BlockTradeExecution>> {
        shared::send_signed_request(
            self,
            "/eapi/v1/block/user-trades",
            reqwest::Method::GET,
            params,
            1,
            false,
        )
        .await
    }
}
