use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

/// Request to set auto repay
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoRepayRequest {
    /// Auto repay status: true, false
    pub auto_repay: bool,
}

/// Response for set auto repay
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoRepayResponse {
    /// Auto repay status
    pub auto_repay: bool,
}

impl RestClient {
    /// Set auto repay
    ///
    /// # Arguments
    /// * `request` - The set auto repay request
    ///
    /// # Returns
    /// A result containing the set auto repay response or an error
    pub async fn set_auto_repay(&self, request: &SetAutoRepayRequest) -> RestResult<OkxApiResponse<SetAutoRepayResponse>> {
        self.send_request(
            "api/v5/account/set-auto-repay",
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
    fn test_set_auto_repay_request_serialization() {
        let request = SetAutoRepayRequest { auto_repay: true };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"autoRepay\":true"));
    }

    #[test]
    fn test_set_auto_repay_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "autoRepay": true
                }
            ]
        }"#;

        let response: OkxApiResponse<SetAutoRepayResponse> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let result = &response.data[0];
        assert!(result.auto_repay);
    }
}
