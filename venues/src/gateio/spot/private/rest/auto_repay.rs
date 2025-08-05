use serde::{Deserialize, Serialize};

use super::RestClient;

const MARGIN_AUTO_REPAY_ENDPOINT: &str = "/margin/auto_repay";

/// Request parameters for auto repay settings
#[derive(Debug, Clone, Serialize, Default)]
pub struct AutoRepayRequest {
    /// Status (on/off)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Auto repay settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoRepaySetting {
    /// Auto repay status
    pub status: String,
}

impl RestClient {
    /// Get auto repay settings
    ///
    /// This endpoint returns the current auto repay settings for margin trading.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-auto-repay-settings>
    pub async fn get_auto_repay(&self) -> crate::gateio::spot::RestResult<AutoRepaySetting> {
        self.get(MARGIN_AUTO_REPAY_ENDPOINT).await
    }

    /// Update auto repay settings
    ///
    /// This endpoint updates the auto repay settings for margin trading.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#update-auto-repay-settings>
    pub async fn update_auto_repay(
        &self,
        params: AutoRepayRequest,
    ) -> crate::gateio::spot::RestResult<AutoRepaySetting> {
        self.post(MARGIN_AUTO_REPAY_ENDPOINT, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_repay_request_default() {
        let request = AutoRepayRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty()); // No fields should be serialized when None
    }

    #[test]
    fn test_auto_repay_request_with_status() {
        let request = AutoRepayRequest {
            status: Some("on".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["status"], "on");
    }

    #[test]
    fn test_auto_repay_request_different_statuses() {
        let statuses = vec!["on", "off"];

        for status in statuses {
            let request = AutoRepayRequest {
                status: Some(status.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["status"], status);
        }
    }

    #[test]
    fn test_auto_repay_setting_deserialization() {
        let json = r#"{
            "status": "on"
        }"#;

        let setting: AutoRepaySetting = serde_json::from_str(json).unwrap();
        assert_eq!(setting.status, "on");
    }

    #[test]
    fn test_auto_repay_setting_different_statuses() {
        let statuses = vec!["on", "off"];

        for status in statuses {
            let json = format!(
                r#"{{
                "status": "{}"
            }}"#,
                status
            );

            let setting: AutoRepaySetting = serde_json::from_str(&json).unwrap();
            assert_eq!(setting.status, status);
        }
    }

    #[test]
    fn test_auto_repay_request_realistic_enable_scenario() {
        // Scenario: Enable auto repay for risk management
        let request = AutoRepayRequest {
            status: Some("on".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["status"], "on");
    }

    #[test]
    fn test_auto_repay_request_realistic_disable_scenario() {
        // Scenario: Disable auto repay for manual control
        let request = AutoRepayRequest {
            status: Some("off".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["status"], "off");
    }

    #[test]
    fn test_auto_repay_setting_realistic_enabled_scenario() {
        let json = r#"{
            "status": "on"
        }"#;

        let setting: AutoRepaySetting = serde_json::from_str(json).unwrap();
        assert_eq!(setting.status, "on");
        assert!(setting.status == "on" || setting.status == "off");
    }

    #[test]
    fn test_auto_repay_request_optional_status_behavior() {
        // Test with status
        let request_with_status = AutoRepayRequest {
            status: Some("on".to_string()),
        };

        // Test without status
        let request_without_status = AutoRepayRequest { status: None };

        let json_with = serde_json::to_value(&request_with_status).unwrap();
        let json_without = serde_json::to_value(&request_without_status).unwrap();

        // With status - should be included
        let obj_with = json_with.as_object().unwrap();
        assert!(obj_with.contains_key("status"));
        assert_eq!(obj_with.len(), 1);

        // Without status - should be omitted
        let obj_without = json_without.as_object().unwrap();
        assert!(!obj_without.contains_key("status"));
        assert_eq!(obj_without.len(), 0);
    }

    #[test]
    fn test_auto_repay_request_endpoint_validation() {
        let request = AutoRepayRequest {
            status: Some("on".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json.as_object().unwrap().contains_key("status"));

        // Verify status is a string
        assert!(json["status"].is_string());
    }

    #[test]
    fn test_auto_repay_setting_round_trip() {
        let original = AutoRepaySetting {
            status: "on".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: AutoRepaySetting = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.status, original.status);
    }
}