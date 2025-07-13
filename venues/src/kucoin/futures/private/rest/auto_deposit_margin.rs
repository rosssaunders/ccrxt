use serde::{Deserialize, Serialize};

use crate::kucoin::{AutoDepositStatus, ResponseHeaders, RestResponse, Result};

/// Endpoint URL for auto deposit margin
pub const AUTO_DEPOSIT_MARGIN_ENDPOINT: &str = "/api/v1/position/margin/auto-deposit-status";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoDepositMarginRequest {
    pub symbol: String,
    pub status: AutoDepositStatus,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AutoDepositMarginResponse {
    pub result: bool,
}

impl super::RestClient {
    /// Enable/disable auto deposit margin
    pub async fn auto_deposit_margin(
        &self,
        request: AutoDepositMarginRequest,
    ) -> Result<(RestResponse<AutoDepositMarginResponse>, ResponseHeaders)> {
        self.post(AUTO_DEPOSIT_MARGIN_ENDPOINT, &request).await
    }
}
