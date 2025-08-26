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
    /// List supported currencies for multi-collateral loan
    pub async fn list_multi_collateral_currencies(&self) -> RestResult<Vec<CurrencyInfo>> {
        self.send_get_request::<Vec<CurrencyInfo>, ()>(CURRENCIES_ENDPOINT, None)
            .await
    }
}
