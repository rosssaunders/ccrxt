use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const WITHDRAWALS_ENDPOINT: &str = "/withdrawals";

/// Request to withdraw tokens from account
#[derive(Debug, Clone, Serialize)]
pub struct WithdrawRequest {
    /// User-defined order number for tracking (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdraw_order_id: Option<String>,

    /// Token name to withdraw
    pub currency: String,

    /// Withdrawal destination address
    pub address: String,

    /// Token amount to withdraw
    pub amount: String,

    /// Blockchain network name (required)
    pub chain: String,

    /// Memo or tag for certain networks (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// Withdrawal creation response
#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawResponse {
    /// Withdrawal record ID
    pub id: String,

    /// Initial withdrawal status
    pub status: String,
}

impl RestClient {
    /// Withdraw
    ///
    /// Withdraw tokens from account to external address.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#withdraw)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `req` - Withdrawal request with currency, address, amount, and chain
    ///
    /// # Returns
    /// Withdrawal response with ID and initial status
    pub async fn withdraw(&self, req: WithdrawRequest) -> RestResult<WithdrawResponse> {
        self.send_post_request(WITHDRAWALS_ENDPOINT, Some(&req))
            .await
    }
}
