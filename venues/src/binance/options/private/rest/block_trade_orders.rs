use serde::Serialize;

use crate::binance::options::PrivateRestClient as RestClient;
// Re-export the shared response type
pub use super::create_block_trade::BlockTradeOrderResponse;
use crate::binance::options::RestResult;

const BLOCK_TRADE_ORDERS_ENDPOINT: &str = "/eapi/v1/block/order/orders";

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

impl RestClient {
    /// Check block trade order status
    ///
    /// Queries the status of block trade orders.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/block-trade/Check-Block-Trade-Order-Status)
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `params` - The query block trade orders request parameters
    ///
    /// # Returns
    /// List of block trade orders matching the query criteria
    pub async fn get_block_trade_orders(
        &self,
        params: QueryBlockTradeOrdersRequest,
    ) -> RestResult<Vec<BlockTradeOrderResponse>> {
        self.send_get_signed_request(BLOCK_TRADE_ORDERS_ENDPOINT, params, 1, false)
            .await
    }
}
