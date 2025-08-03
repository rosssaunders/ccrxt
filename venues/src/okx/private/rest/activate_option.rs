use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

const ACCOUNT_ACTIVATE_OPTION_ENDPOINT: &str = "api/v5/account/activate-option";

/// Request to activate option
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateOptionRequest {
    // Empty request body for this endpoint
}

/// Response for activate option
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateOptionResponse {
    /// Result message
    pub result: String,
}

impl RestClient {
    /// Activate option
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#trading-account-rest-api-activate-option
    ///
    ///  # Arguments
    /// * `request` - The activate option request
    ///
    /// # Returns
    /// A result containing the activate option response or an error
    pub async fn activate_option(
        &self,
        request: &ActivateOptionRequest,
    ) -> RestResult<OkxApiResponse<ActivateOptionResponse>> {
        self.send_request(
            ACCOUNT_ACTIVATE_OPTION_ENDPOINT,
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
    fn test_activate_option_request_serialization() {
        let request = ActivateOptionRequest {};

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_activate_option_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "result": "success"
                }
            ]
        }"#;

        let response: OkxApiResponse<ActivateOptionResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let result = &response.data[0];
        assert_eq!(result.result, "success");
    }
}
