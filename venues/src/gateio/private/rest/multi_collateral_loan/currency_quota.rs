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
    /// Get Borrowable Quota for Multi-Currency Collateral Loan
    ///
    /// Retrieves the maximum borrowable amount for a specific currency in multi-currency
    /// collateral loans based on current market conditions and platform liquidity.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#get-borrowable-quota-multi-currency-collateral-loan)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `query` - Currency quota query containing the borrow currency
    ///
    /// # Returns
    /// Quota response with the maximum borrowable amount for the specified currency
    pub async fn get_currency_quota(
        &self,
        query: CurrencyQuotaQuery,
    ) -> RestResult<CurrencyQuotaResponse> {
        self.send_get_request(QUOTA_ENDPOINT, Some(&query)).await
    }
}
