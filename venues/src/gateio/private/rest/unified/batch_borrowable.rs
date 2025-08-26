use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const BATCH_BORROWABLE_ENDPOINT: &str = "/unified/batch_borrowable";

/// Request parameters for batch querying borrowable amounts
#[derive(Debug, Clone, Serialize)]
pub struct GetBatchBorrowableRequest {
    /// Comma-separated list of currencies to check borrowable amounts for
    pub currency: String,
}

/// Borrowable amount response for a single currency
#[derive(Debug, Clone, Deserialize)]
pub struct BorrowableItem {
    /// Currency
    pub currency: String,
    /// Maximum borrowable amount
    pub borrowable: String,
}

impl RestClient {
    /// Batch Query Borrowable Amounts
    ///
    /// Batch query unified account maximum borrowable amount for multiple currencies.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#batch-query-unified-account-maximum-borrowable-amount)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Request with comma-separated currencies to check borrowable amounts for
    ///
    /// # Returns
    /// List of borrowable amounts for the specified currencies
    pub async fn get_batch_borrowable(
        &self,
        req: GetBatchBorrowableRequest,
    ) -> RestResult<Vec<BorrowableItem>> {
        self.send_get_request(BATCH_BORROWABLE_ENDPOINT, Some(&req))
            .await
    }
}
