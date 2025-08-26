use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const FIXED_RATE_ENDPOINT: &str = "/loan/multi_collateral/fixed_rate";

/// Fixed rate response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedRateResponse {
    pub borrow_currency: String,

    pub daily_rate: String,
}

impl RestClient {
    /// Get Fixed Borrowing Rate for Multi-Currency Collateral Loan
    ///
    /// Retrieves the fixed borrowing interest rate for a specific currency in multi-currency
    /// collateral loans. Fixed rates remain constant for the loan duration, providing rate certainty.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#get-fixed-rate-multi-currency-collateral-loan)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `borrow_currency` - Currency code for which to retrieve the fixed borrowing rate
    ///
    /// # Returns
    /// Fixed rate response containing the daily borrowing rate for fixed-rate loans
    pub async fn get_fixed_rate(&self, borrow_currency: &str) -> RestResult<FixedRateResponse> {
        #[derive(Serialize)]
        struct Q {
            borrow_currency: String,
        }
        let q = Q {
            borrow_currency: borrow_currency.to_string(),
        };
        self.send_get_request(FIXED_RATE_ENDPOINT, Some(&q)).await
    }
}
