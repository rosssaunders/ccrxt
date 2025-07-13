use serde::{Deserialize, Serialize};

use crate::kucoin::{MarginMode, ResponseHeaders, RestResponse, Result};

/// Endpoint URL for get margin mode
pub const GET_MARGIN_MODE_ENDPOINT: &str = "/api/v2/position/getMarginMode";
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct GetMarginModeRequest {
    pub symbol: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginModeResponse {
    pub symbol: String,
    pub margin_mode: MarginMode,
    pub cross_margin_leverage: String,
    pub isolated_margin_leverage: String,
}

impl super::super::RestClient {
    /// Get margin mode for a symbol
    pub async fn get_margin_mode(
        &self,
        request: GetMarginModeRequest,
    ) -> Result<(RestResponse<MarginModeResponse>, ResponseHeaders)> {
        let endpoint = GET_MARGIN_MODE_ENDPOINT;
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol);
        self.get(endpoint, Some(params)).await
    }
}
