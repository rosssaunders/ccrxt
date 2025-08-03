use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

const ACCOUNT_SET_ISOLATED_MODE_ENDPOINT: &str = "api/v5/account/set-isolated-mode";
/// Request to set isolated mode
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetIsolatedModeRequest {
    /// Instrument type: MARGIN
    pub inst_type: String,

    /// Isolated mode: automatic, autonomy
    pub is_auto: String,

    /// Instrument type: SPOT, FUTURES, SWAP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// Response for set isolated mode
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetIsolatedModeResponse {
    /// Instrument type
    pub inst_type: String,

    /// Isolated mode
    pub is_auto: String,
}

impl RestClient {
    /// Set isolated mode
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#trading-account-rest-api-isolated-margin-trading-settings
    ///
    /// # Arguments
    /// * `request` - The set isolated mode request
    ///
    /// # Returns
    /// A result containing the set isolated mode response or an error
    pub async fn set_isolated_mode(
        &self,
        request: &SetIsolatedModeRequest,
    ) -> RestResult<OkxApiResponse<SetIsolatedModeResponse>> {
        self.send_request(
            ACCOUNT_SET_ISOLATED_MODE_ENDPOINT,
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
    fn test_set_isolated_mode_request_serialization() {
        let request = SetIsolatedModeRequest {
            inst_type: "MARGIN".to_string(),
            is_auto: "automatic".to_string(),
            type_: Some("SPOT".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instType\":\"MARGIN\""));
        assert!(json.contains("\"isAuto\":\"automatic\""));
        assert!(json.contains("\"type\":\"SPOT\""));
    }

    #[test]
    fn test_set_isolated_mode_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "MARGIN",
                    "isAuto": "automatic"
                }
            ]
        }"#;

        let response: OkxApiResponse<SetIsolatedModeResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let result = response.data.first();
        assert!(result.is_some(), "Expected at least one result in response");
        let result = result.unwrap();
        assert_eq!(result.inst_type, "MARGIN");
        assert_eq!(result.is_auto, "automatic");
    }
}
