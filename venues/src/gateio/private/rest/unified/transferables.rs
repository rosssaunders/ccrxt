use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const TRANSFERABLES_ENDPOINT: &str = "/unified/transferables";

/// Request parameters for batch querying transferable amounts
#[derive(Debug, Clone, Serialize)]
pub struct GetTransferablesRequest {
    /// Comma-separated list of currencies to check transferable amounts for
    pub currency: String,
}

/// Transferable amount response for a single currency
#[derive(Debug, Clone, Deserialize)]
pub struct TransferableItem {
    /// Currency
    pub currency: String,

    /// Transferable amount
    pub amount: String,
}

impl RestClient {
    /// Batch Query Transferable Amounts
    ///
    /// Batch query maximum transferable amount for multiple currencies in unified account.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#batch-query-maximum-transferable-amount-for-unified-accounts)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Request with comma-separated currencies to check transferable amounts for
    ///
    /// # Returns
    /// List of transferable amounts for the specified currencies
    pub async fn get_unified_transferables(
        &self,
        req: GetTransferablesRequest,
    ) -> RestResult<Vec<TransferableItem>> {
        self.send_get_request(TRANSFERABLES_ENDPOINT, Some(&req))
            .await
    }
}
