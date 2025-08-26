use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const MORTGAGE_ENDPOINT: &str = "/loan/multi_collateral/mortgage";

/// Add collateral to a multi-collateral order
#[derive(Debug, Clone, Serialize)]
pub struct AddMortgageRequest {
    pub order_id: String,

    pub collaterals: Vec<MortgagePart>,
}

/// One mortgage part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MortgagePart {
    pub currency: String,

    pub amount: String,
}

/// Add mortgage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddMortgageResponse {
    pub success: bool,
}

impl RestClient {
    /// Add collateral assets (mortgage) to a multi-collateral order
    pub async fn add_mortgage(&self, req: AddMortgageRequest) -> RestResult<AddMortgageResponse> {
        self.send_post_request(MORTGAGE_ENDPOINT, Some(&req)).await
    }
}
