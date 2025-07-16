use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{AutoDepositStatus, ResponseHeaders, RestResponse, Result};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_deposit_margin_request_serialization_enabled() {
        let request = AutoDepositMarginRequest {
            symbol: "XBTUSDTM".to_string(),
            status: AutoDepositStatus::On,
        };

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"symbol":"XBTUSDTM","status":"on"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_auto_deposit_margin_request_serialization_disabled() {
        let request = AutoDepositMarginRequest {
            symbol: "ETHUSDTM".to_string(),
            status: AutoDepositStatus::Off,
        };

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"symbol":"ETHUSDTM","status":"off"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_auto_deposit_margin_response_deserialization_success() {
        let json = r#"{"result":true}"#;
        let response: AutoDepositMarginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.result, true);
    }

    #[test]
    fn test_auto_deposit_margin_response_deserialization_failure() {
        let json = r#"{"result":false}"#;
        let response: AutoDepositMarginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.result, false);
    }

    #[test]
    fn test_auto_deposit_margin_endpoint() {
        assert_eq!(AUTO_DEPOSIT_MARGIN_ENDPOINT, "/api/v1/position/margin/auto-deposit-status");
    }
}
