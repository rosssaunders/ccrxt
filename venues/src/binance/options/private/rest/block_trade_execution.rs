use serde::Serialize;

use crate::binance::options::PrivateRestClient as RestClient;
// Re-export the shared response type
pub use super::execute_block_trade::BlockTradeExecution;
use crate::binance::options::RestResult;

const BLOCK_TRADE_EXECUTION_ENDPOINT: &str = "/eapi/v1/block/order/execute";

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

impl RestClient {
    /// Query block trade details
    ///
    /// Returns details of block trade executions.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/block-trade/Query-Block-Trade-Details)
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `params` - The query block trade execution request parameters
    ///
    /// # Returns
    /// List of block trade execution details
    pub async fn get_block_trade_execution_details(
        &self,
        params: QueryBlockTradeExecutionRequest,
    ) -> RestResult<Vec<BlockTradeExecution>> {
        self.send_get_signed_request(BLOCK_TRADE_EXECUTION_ENDPOINT, params, 1, false)
            .await
    }
}
