use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

/// Request parameters for cancelling a withdrawal
#[derive(Debug, Clone, Serialize)]
pub struct CancelWithdrawalRequest {
    /// Withdrawal record ID to cancel
    pub withdrawal_id: String,
}

/// Cancel withdrawal response
#[derive(Debug, Clone, Deserialize)]
pub struct CancelWithdrawalResponse {
    /// Withdrawal record ID
    pub id: String,

    /// Updated withdrawal status after cancellation
    pub status: String,
}

impl RestClient {
    /// Cancel Withdrawal
    ///
    /// Cancel a pending withdrawal request by its ID.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#cancel-withdrawal-with-specified-id)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `withdrawal_id` - ID of the withdrawal to cancel
    ///
    /// # Returns
    /// Response with withdrawal ID and updated status
    pub async fn cancel_withdrawal(
        &self,
        withdrawal_id: &str,
    ) -> RestResult<CancelWithdrawalResponse> {
        let endpoint = format!("/withdrawals/{}", withdrawal_id);
        self.send_delete_request(&endpoint, None::<&()>).await
    }
}
