use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const CURRENCIES_ENDPOINT: &str = "/loan/multi_collateral/currencies";

/// Multi-collateral currency info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyInfo {
    pub currency: String,

    pub borrowable: bool,

    pub collateral: bool,
}

impl RestClient {
    /// Query Supported Currencies for Multi-Currency Collateral Loan
    ///
    /// Retrieves the complete list of currencies supported for multi-currency collateral loans,
    /// including their availability as borrowable assets or acceptable collateral.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#query-supported-currencies-for-multi-currency-collateral-loan)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Returns
    /// List of currency information with borrowable and collateral eligibility flags
    pub async fn list_multi_collateral_currencies(&self) -> RestResult<Vec<CurrencyInfo>> {
        self.send_get_request::<Vec<CurrencyInfo>, ()>(CURRENCIES_ENDPOINT, None)
            .await
    }
}
