use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{AutoDepositStatus, ResponseHeaders, RestResponse, Result};

/// Endpoint URL for Add Margin
const ADD_MARGIN_ENDPOINT: &str = "/api/v1/position/margin/deposit-margin";

#[derive(Debug, Clone, Serialize)]
pub struct AddMarginRequest {
    pub symbol: String,
    pub margin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_no: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMarginResponse {
    pub id: String,
    pub symbol: String,
    pub auto_deposit_status: AutoDepositStatus,
    pub margin: String,
    pub risk_limit: i64,
    pub realized_roi: String,
    pub cross_mode: bool,
    pub deleverage_percentage: f64,
    pub open_size: String,
    pub value: String,
    pub available_balance: f64,
}

impl super::RestClient {
    /// Add margin to position
    pub async fn add_margin(
        &self,
        request: AddMarginRequest,
    ) -> Result<(RestResponse<AddMarginResponse>, ResponseHeaders)> {
        let endpoint = ADD_MARGIN_ENDPOINT;
        self.post(endpoint, &request).await
    }
}
