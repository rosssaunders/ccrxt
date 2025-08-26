use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const CURRENT_RATE_ENDPOINT: &str = "/loan/multi_collateral/current_rate";

/// Current rate response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentRateResponse {
    pub borrow_currency: String,

    pub daily_rate: String,
}

impl RestClient {
    /// Get Current Borrowing Rate for Multi-Currency Collateral Loan
    ///
    /// Retrieves the current variable borrowing interest rate for a specific currency
    /// in multi-currency collateral loans. Rates are subject to market conditions and change dynamically.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#get-current-rate-multi-currency-collateral-loan)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `borrow_currency` - Currency code for which to retrieve the current borrowing rate
    ///
    /// # Returns
    /// Current rate response containing the daily borrowing rate
    pub async fn get_current_rate(&self, borrow_currency: &str) -> RestResult<CurrentRateResponse> {
        #[derive(Serialize)]
        struct Q {
            borrow_currency: String,
        }
        let q = Q {
            borrow_currency: borrow_currency.to_string(),
        };
        self.send_get_request(CURRENT_RATE_ENDPOINT, Some(&q)).await
    }
}
