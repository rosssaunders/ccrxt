use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for cancel all after
const CANCEL_ALL_AFTER_ENDPOINT: &str = "api/v5/rfq/cancel-all-after";

/// Request parameters for cancel all after
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllAfterRequest {
    /// The countdown for quotes cancellation, with second as the unit.
    /// Range of value can be 0, [10, 120].
    /// Setting timeOut to 0 disables Cancel All After.
    #[serde(rename = "timeOut")]
    pub time_out: String,
}

/// Response for cancel all after
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllAfterResponse {
    /// The time the cancellation is triggered.
    /// triggerTime=0 means Cancel All After is disabled.
    #[serde(rename = "triggerTime")]
    pub trigger_time: String,

    /// The time the request is received.
    pub ts: String,
}

impl RestClient {
    /// Cancel All After
    ///
    /// Cancel all quotes after the countdown timeout.
    ///
    /// Users are recommended to send a request to the exchange every second. When the
    /// cancel all after is triggered, the trading engine will cancel quotes on behalf
    /// of the client one by one and this operation may take up to a few seconds. This
    /// feature is intended as a protection mechanism for clients only and clients
    /// should not use this feature as part of their trading strategies.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-cancel-all-after)
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The cancel all after request parameters
    ///
    /// # Returns
    /// Response containing the trigger time and request timestamp
    pub async fn cancel_all_after(
        &self,
        request: CancelAllAfterRequest,
    ) -> RestResult<CancelAllAfterResponse> {
        self.send_post_request(
            CANCEL_ALL_AFTER_ENDPOINT,
            &request,
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_cancel_all_after_request_new() {
        let request = CancelAllAfterRequest {
            time_out: "60".to_string(),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"timeOut\":\"60\""));
    }

    #[test]
    fn test_cancel_all_after_request_disable() {
        let request = CancelAllAfterRequest {
            time_out: "0".to_string(),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"timeOut\":\"0\""));
    }

    #[test]
    fn test_cancel_all_after_response_deserialization() {
        let response_json = json!({
            "triggerTime": "1597026443085",
            "ts": "1597026383085"
        });

        let response: CancelAllAfterResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.trigger_time, "1597026443085");
        assert_eq!(response.ts, "1597026383085");
    }

    #[test]
    fn test_cancel_all_after_response_disabled() {
        let response_json = json!({
            "triggerTime": "0",
            "ts": "1597026383085"
        });

        let response: CancelAllAfterResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.trigger_time, "0");
        assert_eq!(response.ts, "1597026383085");
    }

    #[test]
    fn test_cancel_all_after_api_response() {
        let api_response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "triggerTime": "1597026443085",
                    "ts": "1597026383085"
                }
            ]
        });

        let api_response: ApiResponse<CancelAllAfterResponse> =
            serde_json::from_value(api_response_json).unwrap();
        assert_eq!(api_response.code, "0");
        assert_eq!(api_response.data.len(), 1);
        assert_eq!(api_response.data[0].trigger_time, "1597026443085");
        assert_eq!(api_response.data[0].ts, "1597026383085");
    }

    #[test]
    fn test_cancel_all_after_request_validation_bounds() {
        // Test minimum valid timeout
        let request_min = CancelAllAfterRequest {
            time_out: "10".to_string(),
        };
        assert_eq!(request_min.time_out, "10");

        // Test maximum valid timeout
        let request_max = CancelAllAfterRequest {
            time_out: "120".to_string(),
        };
        assert_eq!(request_max.time_out, "120");

        // Test disable
        let request_disable = CancelAllAfterRequest {
            time_out: "0".to_string(),
        };
        assert_eq!(request_disable.time_out, "0");
    }
}
