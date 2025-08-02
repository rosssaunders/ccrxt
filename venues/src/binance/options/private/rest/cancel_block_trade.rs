use serde::Serialize;

use super::client::RestClient;
// Re-export the shared response type
pub use super::create_block_trade::BlockTradeOrderResponse;
use crate::binance::options::RestResult;

const CANCEL_BLOCK_TRADE_ENDPOINT: &str = "/eapi/v1/block/order/create";

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

impl RestClient {
    /// Cancel a block trade order
    ///
    /// Cancels an existing block trade order.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/option/block-trade/Cancel-a-Block-Trade-Order
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `params` - The cancel block trade request parameters
    ///
    /// # Returns
    /// Block trade order response with updated status
    pub async fn cancel_block_trade_order(
        &self,
        params: CancelBlockTradeRequest,
    ) -> RestResult<BlockTradeOrderResponse> {
        self.send_delete_signed_request(
            CANCEL_BLOCK_TRADE_ENDPOINT,
            params,
            1,
            true, // is_order = true for order endpoints
        )
        .await
    }
}
