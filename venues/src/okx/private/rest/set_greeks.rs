use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

const ACCOUNT_SET_GREEKS_ENDPOINT: &str = "api/v5/account/set-greeks";

/// Request to set greeks
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetGreeksRequest {
    /// Greeks display type
    /// "PA": Greeks are shown in coins
    /// "BS": Greeks are shown in dollars  
    pub greeks_type: String,
}

/// Response from setting greeks
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetGreeksResponse {
    /// Greeks display type
    pub greeks_type: String,
}

impl RestClient {
    /// Set greeks
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-set-greeks-pa-bs)
    ///
    /// # Arguments
    /// * `request` - The set greeks request
    ///
    /// # Returns
    /// A result containing the greeks response or an error
    pub async fn set_greeks(&self, request: &SetGreeksRequest) -> RestResult<SetGreeksResponse> {
        self.send_post_request(
            ACCOUNT_SET_GREEKS_ENDPOINT,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_set_greeks_request_serialization() {
        let request = SetGreeksRequest {
            greeks_type: "PA".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"greeksType\":\"PA\""));
    }

    #[test]
    fn test_set_greeks_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "greeksType": "PA"
                }
            ]
        }"#;

        let response: OkxApiResponse<SetGreeksResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let result = response.data.first();
        assert!(result.is_some(), "Expected at least one result in response");
        let result = result.unwrap();
        assert_eq!(result.greeks_type, "PA");
    }
}
