use serde::Serialize;

// Re-export the shared response type
pub use super::execute_block_trade::BlockTradeExecution;
use crate::binance::options::{RestResult, private_client::RestClient};

const BLOCK_USER_TRADES_ENDPOINT: &str = "/eapi/v1/block/user-trades";

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

impl RestClient {
    /// Get block trades for a specific account
    ///
    /// Returns block trade history for the current account.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/block-trade/Get-Block-Trades-for-a-Specific-Account)
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `params` - The query block user trades request parameters
    ///
    /// # Returns
    /// List of block trade executions for the account
    pub async fn get_block_user_trades(
        &self,
        params: QueryBlockUserTradesRequest,
    ) -> RestResult<Vec<BlockTradeExecution>> {
        self.send_get_signed_request(BLOCK_USER_TRADES_ENDPOINT, params, 1, false)
            .await
    }
}
