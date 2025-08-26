use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const TRANSFER_ENDPOINT: &str = "/subaccount/transfer";

/// Transfer between sub-accounts
#[derive(Debug, Clone, Serialize)]
pub struct SubaccountTransferRequest {
    pub from_uid: String,

    pub to_uid: String,

    pub currency: String,

    pub amount: String,
}

/// Transfer response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubaccountTransferResponse {
    pub transfer_id: String,

    pub status: String,
}

impl RestClient {
    /// Create a transfer between sub-accounts
    pub async fn subaccount_transfer(
        &self,
        req: SubaccountTransferRequest,
    ) -> RestResult<SubaccountTransferResponse> {
        self.send_post_request(TRANSFER_ENDPOINT, Some(&req)).await
    }
}
