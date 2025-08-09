use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{AutoDepositStatus, ResponseHeaders, RestResponse, Result};

/// Endpoint URL for Auto Deposit Margin
const AUTO_DEPOSIT_MARGIN_ENDPOINT: &str = "/api/v1/position/margin/auto-deposit-status";

/// Request parameters for enabling or disabling auto deposit margin.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoDepositMarginRequest {
    /// Trading symbol (e.g., "XBTUSDTM").
    pub symbol: String,

    /// Auto deposit status to set (on or off).
    pub status: AutoDepositStatus,
}

/// Response data from the auto deposit margin endpoint.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AutoDepositMarginResponse {
    /// Whether the operation was successful.
    pub result: bool,
}

impl super::RestClient {
    /// Enable/Disable Auto Deposit Margin
    ///
    /// Enable or disable auto deposit margin for a specific futures position.
    /// When enabled, the system will automatically deposit margin to prevent liquidation.
    ///
    /// [docs]: https://www.kucoin.com/docs-new/rest/futures-trading/positions/auto-deposit-margin
    ///
    /// Rate limit: 4
    ///
    /// # Arguments
    /// * `request` - The auto deposit margin request parameters
    ///
    /// # Returns
    /// Result indicating whether the operation was successful
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
        assert!(json.contains("XBTUSDTM"));
        assert!(json.contains("on"));
        assert!(json.contains("symbol"));
        assert!(json.contains("status"));
    }

    #[test]
    fn test_auto_deposit_margin_request_serialization_disabled() {
        let request = AutoDepositMarginRequest {
            symbol: "ETHUSDTM".to_string(),
            status: AutoDepositStatus::Off,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("ETHUSDTM"));
        assert!(json.contains("off"));
        assert!(json.contains("symbol"));
        assert!(json.contains("status"));
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
        assert_eq!(
            AUTO_DEPOSIT_MARGIN_ENDPOINT,
            "/api/v1/position/margin/auto-deposit-status"
        );
    }

    #[test]
    fn test_auto_deposit_margin_request_field_types() {
        let request = AutoDepositMarginRequest {
            symbol: "SOLUSDTM".to_string(),
            status: AutoDepositStatus::On,
        };

        // Verify field types through serialization
        let json = serde_json::to_value(&request).unwrap();

        assert!(json["symbol"].is_string());
        assert!(json["status"].is_string());
    }

    #[test]
    fn test_auto_deposit_margin_request_symbols() {
        let btc_request = AutoDepositMarginRequest {
            symbol: "XBTUSDTM".to_string(),
            status: AutoDepositStatus::On,
        };

        let eth_request = AutoDepositMarginRequest {
            symbol: "ETHUSDTM".to_string(),
            status: AutoDepositStatus::Off,
        };

        let btc_json = serde_json::to_string(&btc_request).unwrap();
        let eth_json = serde_json::to_string(&eth_request).unwrap();

        assert!(btc_json.contains("XBTUSDTM"));
        assert!(btc_json.contains("on"));
        assert!(eth_json.contains("ETHUSDTM"));
        assert!(eth_json.contains("off"));
    }

    #[test]
    fn test_auto_deposit_margin_response_boolean_values() {
        // Test true response
        let json_true = r#"{"result":true}"#;
        let response_true: AutoDepositMarginResponse = serde_json::from_str(json_true).unwrap();
        assert_eq!(response_true.result, true);

        // Test false response
        let json_false = r#"{"result":false}"#;
        let response_false: AutoDepositMarginResponse = serde_json::from_str(json_false).unwrap();
        assert_eq!(response_false.result, false);

        // Verify field type
        let json_value = serde_json::to_value(&response_true).unwrap();
        assert!(json_value["result"].is_boolean());
    }

    #[test]
    fn test_auto_deposit_status_enum_serialization() {
        let on_status = AutoDepositStatus::On;
        let off_status = AutoDepositStatus::Off;

        let on_json = serde_json::to_string(&on_status).unwrap();
        let off_json = serde_json::to_string(&off_status).unwrap();

        assert_eq!(on_json, "\"on\"");
        assert_eq!(off_json, "\"off\"");
    }
}
