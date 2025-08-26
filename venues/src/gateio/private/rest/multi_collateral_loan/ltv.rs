use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const LTV_ENDPOINT: &str = "/loan/multi_collateral/ltv";

/// LTV query
#[derive(Debug, Clone, Serialize)]
pub struct LtvQuery {
    pub borrow_currency: String,
}

/// Response shape for LTV by collateral currencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LtvEntry {
    pub collateral_currency: String,

    pub init: String,

    pub liquidation: String,
}

impl RestClient {
    /// Get Loan-to-Value Ratios for Multi-Currency Collateral Loan
    ///
    /// Retrieves the loan-to-value (LTV) ratios for different collateral currencies
    /// when borrowing a specific currency. Includes both initial and liquidation LTV thresholds.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#get-ltv-multi-currency-collateral-loan)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `query` - LTV query containing the borrow currency for which to retrieve collateral ratios
    ///
    /// # Returns
    /// List of LTV entries with initial and liquidation ratios for each supported collateral currency
    pub async fn get_multi_collateral_ltv(&self, query: LtvQuery) -> RestResult<Vec<LtvEntry>> {
        self.send_get_request(LTV_ENDPOINT, Some(&query)).await
    }
}
