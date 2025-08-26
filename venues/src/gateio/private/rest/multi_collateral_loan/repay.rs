use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const REPAY_ENDPOINT: &str = "/loan/multi_collateral/repay";

/// Request to repay multi-collateral order
#[derive(Debug, Clone, Serialize)]
pub struct RepayRequest {
    pub order_id: String,

    pub amount: String,
}

/// Repay response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepayResponse {
    pub success: bool,

    pub remaining: Option<String>,
}

impl RestClient {
    /// Repay a multi-collateral order
    pub async fn repay_multi_collateral(
        &self,
        req: RepayRequest,
    ) -> RestResult<RepayResponse> {
        self.send_post_request(REPAY_ENDPOINT, Some(&req)).await
    }
}
