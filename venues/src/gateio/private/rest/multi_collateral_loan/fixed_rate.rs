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
    /// Get fixed borrowing rate for a currency
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
