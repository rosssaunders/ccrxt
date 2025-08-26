use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const QUOTA_ENDPOINT: &str = "/loan/multi_collateral/currency_quota";

/// Query borrowable quota for a currency
#[derive(Debug, Clone, Serialize)]
pub struct CurrencyQuotaQuery {
    pub borrow_currency: String,
}

/// Quota response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyQuotaResponse {
    pub borrow_currency: String,

    pub max_amount: String,
}

impl RestClient {
    /// Get borrowable quota for a currency under multi-collateral loan
    pub async fn get_currency_quota(
        &self,
        query: CurrencyQuotaQuery,
    ) -> RestResult<CurrencyQuotaResponse> {
        self.send_get_request(QUOTA_ENDPOINT, Some(&query)).await
    }
}
