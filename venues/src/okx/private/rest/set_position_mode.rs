use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

const ACCOUNT_SET_POSITION_MODE_ENDPOINT: &str = "/api/v5/account/set-position-mode";


/// Request to set position mode
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPositionModeRequest {
    /// Position mode
    /// "long_short_mode": long/short mode
    /// "net_mode": net mode
    pub pos_mode: String,
}

/// Response from setting position mode
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPositionModeResponse {
    /// Position mode
    pub pos_mode: String,
}

impl RestClient {
    /// Set position mode
    ///
    /// # Arguments
    /// * `request` - The set position mode request
    ///
    /// # Returns
    /// A result containing the position mode response or an error
    pub async fn set_position_mode(
        &self,
        request: &SetPositionModeRequest,
    ) -> RestResult<OkxApiResponse<SetPositionModeResponse>> {
        self.send_request(
            ACCOUNT_SET_POSITION_MODE_ENDPOINT,
            reqwest::Method::POST,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_position_mode_request_serialization() {
        let request = SetPositionModeRequest {
            pos_mode: "long_short_mode".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"posMode\":\"long_short_mode\""));
    }

    #[test]
    fn test_set_position_mode_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "posMode": "long_short_mode"
                }
            ]
        }"#;

        let response: OkxApiResponse<SetPositionModeResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let result = response.data.first();
        assert!(result.is_some(), "Expected at least one result in response");
        let result = result.unwrap();
        assert_eq!(result.pos_mode, "long_short_mode");
    }
}
