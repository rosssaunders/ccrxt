use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const BORROWABLE_ENDPOINT: &str = "/unified/borrowable";

/// Request parameters for getting maximum borrowable amount
#[derive(Debug, Clone, Serialize)]
pub struct GetBorrowableRequest {
    /// Currency to check borrowable amount for
    pub currency: String,
}

/// Maximum borrowable amount response
#[derive(Debug, Clone, Deserialize)]
pub struct BorrowableResponse {
    /// Currency
    pub currency: String,
    /// Maximum borrowable amount
    pub borrowable: String,
}

impl RestClient {
    /// Query Maximum Borrowable Amount
    ///
    /// Query the maximum borrowable amount for a specific currency in unified account.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#query-maximum-borrowable-amount-for-unified-account)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Request with currency to check borrowable amount for
    ///
    /// # Returns
    /// Maximum borrowable amount for the specified currency
    pub async fn get_unified_borrowable(
        &self,
        req: GetBorrowableRequest,
    ) -> RestResult<BorrowableResponse> {
        self.send_get_request(BORROWABLE_ENDPOINT, Some(&req)).await
    }
}
