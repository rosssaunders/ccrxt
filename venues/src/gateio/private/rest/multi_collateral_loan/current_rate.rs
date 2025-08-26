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
    /// Get current borrowing rate for a currency
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
