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
    /// Get LTV values by collateral currency for a borrow currency
    pub async fn get_multi_collateral_ltv(
        &self,
        query: LtvQuery,
    ) -> RestResult<Vec<LtvEntry>> {
        self.send_get_request(LTV_ENDPOINT, Some(&query)).await
    }
}
