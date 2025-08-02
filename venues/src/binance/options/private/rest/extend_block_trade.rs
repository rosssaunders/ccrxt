use serde::Serialize;

use super::client::RestClient;
use crate::binance::options::RestResult;

// Re-export the shared response type
pub use super::create_block_trade::BlockTradeOrderResponse;

const EXTEND_BLOCK_TRADE_ENDPOINT: &str = "/eapi/v1/block/order/create";

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

impl RestClient {
    /// Extend block trade expire time
    ///
    /// Extends the expiration time of a block trade order.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/option/block-trade/Extend-Block-Trade-Expire-Time
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `params` - The extend block trade request parameters
    ///
    /// # Returns
    /// Block trade order response with updated expire time
    pub async fn extend_block_trade_expire_time(
        &self,
        params: ExtendBlockTradeRequest,
    ) -> RestResult<BlockTradeOrderResponse> {
        self.send_put_signed_request(
            EXTEND_BLOCK_TRADE_ENDPOINT,
            params,
            1,
            true, // is_order = true for order endpoints
        )
        .await
    }
}
