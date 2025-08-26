use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const TRANSFERABLE_ENDPOINT: &str = "/unified/transferable";

/// Request parameters for getting maximum transferable amount
#[derive(Debug, Clone, Serialize)]
pub struct GetTransferableRequest {
    /// Currency to check transferable amount for
    pub currency: String,
}

/// Maximum transferable amount response
#[derive(Debug, Clone, Deserialize)]
pub struct TransferableResponse {
    /// Currency
    pub currency: String,

    /// Maximum transferable amount
    pub amount: String,
}

impl RestClient {
    /// Query Maximum Transferable Amount
    ///
    /// Query the maximum transferable amount for a specific currency in unified account.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#query-maximum-transferable-amount-for-unified-account)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Request with currency to check transferable amount for
    ///
    /// # Returns
    /// Maximum transferable amount for the specified currency
    pub async fn get_unified_transferable(
        &self,
        req: GetTransferableRequest,
    ) -> RestResult<TransferableResponse> {
        self.send_get_request(TRANSFERABLE_ENDPOINT, Some(&req))
            .await
    }
}
