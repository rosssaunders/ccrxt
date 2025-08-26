use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const WITHDRAWAL_ENDPOINT: &str = "/withdrawal/withdrawals";

/// Create a withdrawal
#[derive(Debug, Clone, Serialize)]
pub struct CreateWithdrawalRequest {
    pub currency: String,

    pub address: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,

    pub amount: String,

    /// Optional memo/tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// Withdrawal response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalResponse {
    pub id: String,

    pub status: String,
}

impl RestClient {
    /// Create a withdrawal request
    pub async fn create_withdrawal(
        &self,
        req: CreateWithdrawalRequest,
    ) -> RestResult<WithdrawalResponse> {
        self.send_post_request(WITHDRAWAL_ENDPOINT, Some(&req))
            .await
    }
}
