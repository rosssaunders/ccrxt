use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const MORTGAGE_ENDPOINT: &str = "/loan/multi_collateral/mortgage";

/// List mortgage assets for multi-collateral loans
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListMortgageQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Mortgage asset entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MortgageEntry {
    pub order_id: String,

    pub currency: String,

    pub amount: String,
}

impl RestClient {
    /// List mortgages
    pub async fn list_mortgages(&self, query: ListMortgageQuery) -> RestResult<Vec<MortgageEntry>> {
        self.send_get_request(MORTGAGE_ENDPOINT, Some(&query)).await
    }
}
