use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::options::{
    OptionsContractType, OptionsOrderSide, RestResult, private_client::RestClient,
};

const EXECUTE_BLOCK_TRADE_ENDPOINT: &str = "/eapi/v1/block/order/execute";

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
    /// Accept a block trade order
    ///
    /// Executes (accepts) a block trade order.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/block-trade/Accept-a-Block-Trade-Order)
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `params` - The execute block trade request parameters
    ///
    /// # Returns
    /// Block trade execution details
    pub async fn execute_block_trade_order(
        &self,
        params: ExecuteBlockTradeRequest,
    ) -> RestResult<BlockTradeExecution> {
        self.send_post_signed_request(
            EXECUTE_BLOCK_TRADE_ENDPOINT,
            params,
            1,
            true, // is_order = true for order endpoints
        )
        .await
    }
}
