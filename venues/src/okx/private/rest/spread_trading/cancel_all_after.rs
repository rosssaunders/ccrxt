use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const CANCEL_ALL_AFTER_ENDPOINT: &str = "/api/v5/sprd/cancel-all-after";

/// Request parameters for cancel all after
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelAllAfterRequest {
    /// The countdown for order cancellation, with second as the unit
    /// Range of value can be 0, [10, 120]
    /// Setting timeOut to 0 disables Cancel All After
    #[serde(rename = "timeOut")]
    pub time_out: String,
}

/// Response data for cancel all after
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelAllAfterResponse {
    /// The time the cancellation is triggered
    /// triggerTime=0 means Cancel All After is disabled
    #[serde(rename = "triggerTime")]
    pub trigger_time: String,

    /// The time the request is received
    #[serde(rename = "ts")]
    pub ts: String,
}

impl RestClient {
    /// Cancel all spread orders after
    ///
    /// Set up a cancel all after timer for spread orders
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#spread-trading-rest-api-cancel-all-after)
    pub async fn cancel_all_spread_after(
        &self,
        request: CancelAllAfterRequest,
    ) -> RestResult<CancelAllAfterResponse> {
        self.send_post_request(
            CANCEL_ALL_AFTER_ENDPOINT,
            Some(&request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_cancel_all_after_request_serialization() {
        let request = CancelAllAfterRequest {
            time_out: "60".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: CancelAllAfterRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_cancel_all_after_request_disable() {
        let request = CancelAllAfterRequest {
            time_out: "0".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("\"timeOut\":\"0\""));
    }

    #[test]
    fn test_cancel_all_after_response_enabled() {
        let json_response = r#"{
            "triggerTime": "1597026443085",
            "ts": "1597026383085"
        }"#;

        let response: CancelAllAfterResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.trigger_time, "1597026443085");
        assert_eq!(response.ts, "1597026383085");
    }

    #[test]
    fn test_cancel_all_after_response_disabled() {
        let json_response = r#"{
            "triggerTime": "0",
            "ts": "1597026383085"
        }"#;

        let response: CancelAllAfterResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.trigger_time, "0");
        assert_eq!(response.ts, "1597026383085");
    }

    #[test]
    fn test_cancel_all_after_response_serialization() {
        let response = CancelAllAfterResponse {
            trigger_time: "1597026443085".to_string(),
            ts: "1597026383085".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: CancelAllAfterResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_timeout_values() {
        // Test valid timeout values
        let valid_timeouts = vec!["0", "10", "30", "60", "120"];

        for timeout in valid_timeouts {
            let request = CancelAllAfterRequest {
                time_out: timeout.to_string(),
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"timeOut\":\"{}\"", timeout)));
        }
    }

    #[test]
    fn test_trigger_time_interpretation() {
        // When trigger_time is "0", Cancel All After is disabled
        let response_disabled = CancelAllAfterResponse {
            trigger_time: "0".to_string(),
            ts: "1597026383085".to_string(),
        };

        assert_eq!(response_disabled.trigger_time, "0");

        // When trigger_time is not "0", Cancel All After is enabled
        let response_enabled = CancelAllAfterResponse {
            trigger_time: "1597026443085".to_string(),
            ts: "1597026383085".to_string(),
        };

        assert_ne!(response_enabled.trigger_time, "0");
    }
}
