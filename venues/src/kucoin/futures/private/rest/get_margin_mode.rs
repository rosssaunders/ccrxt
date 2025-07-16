use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{MarginMode, ResponseHeaders, RestResponse, Result};

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

impl super::RestClient {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_margin_mode_request_creation() {
        let request = GetMarginModeRequest {
            symbol: "XBTUSDTM".to_string(),
        };
        assert_eq!(request.symbol, "XBTUSDTM");
    }

    #[test]
    fn test_margin_mode_response_deserialization_cross() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "marginMode": "CROSS_MARGIN",
            "crossMarginLeverage": "10",
            "isolatedMarginLeverage": "20"
        }"#;

        let response: MarginModeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "XBTUSDTM");
        assert_eq!(response.margin_mode, MarginMode::CrossMargin);
        assert_eq!(response.cross_margin_leverage, "10");
        assert_eq!(response.isolated_margin_leverage, "20");
    }

    #[test]
    fn test_margin_mode_response_deserialization_isolated() {
        let json = r#"{
            "symbol": "ETHUSDTM",
            "marginMode": "ISOLATED_MARGIN",
            "crossMarginLeverage": "5",
            "isolatedMarginLeverage": "15"
        }"#;

        let response: MarginModeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "ETHUSDTM");
        assert_eq!(response.margin_mode, MarginMode::IsolatedMargin);
        assert_eq!(response.cross_margin_leverage, "5");
        assert_eq!(response.isolated_margin_leverage, "15");
    }

    #[test]
    fn test_get_margin_mode_endpoint() {
        assert_eq!(GET_MARGIN_MODE_ENDPOINT, "/api/v2/position/getMarginMode");
    }
}
