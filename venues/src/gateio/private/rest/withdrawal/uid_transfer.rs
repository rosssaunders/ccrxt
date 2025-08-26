use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const UID_TRANSFER_ENDPOINT: &str = "/withdrawals/push";

/// Request to transfer between main spot accounts via UID
#[derive(Debug, Clone, Serialize)]
pub struct UidTransferRequest {
    /// Recipient user ID
    pub receive_uid: String,

    /// Token name to transfer
    pub currency: String,

    /// Transfer amount
    pub amount: String,
}

/// UID transfer response
#[derive(Debug, Clone, Deserialize)]
pub struct UidTransferResponse {
    /// Transfer transaction ID
    pub id: String,

    /// Transfer status
    pub status: String,
}

impl RestClient {
    /// UID Transfer
    ///
    /// Transfer funds between main spot accounts using recipient user ID.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#transfers-between-main-spot-accounts)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `req` - Transfer request with recipient UID, currency, and amount
    ///
    /// # Returns
    /// Transfer response with transaction ID and status
    pub async fn uid_transfer(&self, req: UidTransferRequest) -> RestResult<UidTransferResponse> {
        self.send_post_request(UID_TRANSFER_ENDPOINT, Some(&req))
            .await
    }
}
