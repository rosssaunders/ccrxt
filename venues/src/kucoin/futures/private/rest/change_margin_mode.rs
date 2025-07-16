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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_margin_mode_request_serialization_cross() {
        let request = ChangeMarginModeRequest {
            symbol: "XBTUSDTM".to_string(),
            margin_mode: MarginMode::CrossMargin,
        };

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"symbol":"XBTUSDTM","marginMode":"CROSS_MARGIN"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_change_margin_mode_request_serialization_isolated() {
        let request = ChangeMarginModeRequest {
            symbol: "ETHUSDTM".to_string(),
            margin_mode: MarginMode::IsolatedMargin,
        };

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"symbol":"ETHUSDTM","marginMode":"ISOLATED_MARGIN"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_change_margin_mode_response_deserialization_success() {
        let json = r#"{"result":true}"#;
        let response: ChangeMarginModeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.result, true);
    }

    #[test]
    fn test_change_margin_mode_response_deserialization_failure() {
        let json = r#"{"result":false}"#;
        let response: ChangeMarginModeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.result, false);
    }

    #[test]
    fn test_change_margin_mode_endpoint() {
        assert_eq!(CHANGE_MARGIN_MODE_ENDPOINT, "/api/v2/position/changeMarginMode");
    }
}
