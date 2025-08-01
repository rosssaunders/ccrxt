use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};


const ACCOUNT_SET_ACCOUNT_LEVEL_ENDPOINT: &str = "api/v5/account/set-account-level";
/// Request to set account level
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAccountLevelRequest {
    /// Account level: 1 simple, 2 single_currency_margin, 3 multi_currency_margin, 4 portfolio_margin
    pub acct_lv: String,
}

/// Response for set account level
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAccountLevelResponse {
    /// Account level
    pub acct_lv: String,
}

impl RestClient {
    /// Set account level
    ///
    /// # Arguments
    /// * `request` - The set account level request
    ///
    /// # Returns
    /// A result containing the set account level response or an error
    pub async fn set_account_level(
        &self,
        request: &SetAccountLevelRequest,
    ) -> RestResult<OkxApiResponse<SetAccountLevelResponse>> {
        self.send_request(
            ACCOUNT_SET_ACCOUNT_LEVEL_ENDPOINT,
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
    fn test_set_account_level_request_serialization() {
        let request = SetAccountLevelRequest {
            acct_lv: "2".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"acctLv\":\"2\""));
    }

    #[test]
    fn test_set_account_level_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "acctLv": "2"
                }
            ]
        }"#;

        let response: OkxApiResponse<SetAccountLevelResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let result = response.data.first();
        assert!(result.is_some(), "Expected at least one result in response");
        let result = result.unwrap();
        assert_eq!(result.acct_lv, "2");
    }
}
