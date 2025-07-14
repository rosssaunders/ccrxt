use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{MarginMode, ResponseHeaders, RestResponse, Result};

/// Endpoint URL for change margin mode
pub const CHANGE_MARGIN_MODE_ENDPOINT: &str = "/api/v2/position/changeMarginMode";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMarginModeRequest {
    pub symbol: String,
    pub margin_mode: MarginMode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangeMarginModeResponse {
    pub result: bool,
}

impl super::RestClient {
    /// Change margin mode
    pub async fn change_margin_mode(
        &self,
        request: ChangeMarginModeRequest,
    ) -> Result<(RestResponse<ChangeMarginModeResponse>, ResponseHeaders)> {
        self.post(CHANGE_MARGIN_MODE_ENDPOINT, &request).await
    }
}
