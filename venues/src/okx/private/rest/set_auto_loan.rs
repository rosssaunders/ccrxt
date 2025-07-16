use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

const ACCOUNT_SET_AUTO_LOAN_ENDPOINT: &str = "/api/v5/account/set-auto-loan";


/// Request to set auto loan
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoLoanRequest {
    /// Auto loan: true, false
    pub auto_loan: bool,
}

/// Response for set auto loan
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoLoanResponse {
    /// Auto loan status
    pub auto_loan: bool,
}

impl RestClient {
    /// Set auto loan
    ///
    /// # Arguments
    /// * `request` - The set auto loan request
    ///
    /// # Returns
    /// A result containing the set auto loan response or an error
    pub async fn set_auto_loan(
        &self,
        request: &SetAutoLoanRequest,
    ) -> RestResult<OkxApiResponse<SetAutoLoanResponse>> {
        self.send_request(
            ACCOUNT_SET_AUTO_LOAN_ENDPOINT,
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
    fn test_set_auto_loan_request_serialization() {
        let request = SetAutoLoanRequest { auto_loan: true };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"autoLoan\":true"));
    }

    #[test]
    fn test_set_auto_loan_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "autoLoan": true
                }
            ]
        }"#;

        let response: OkxApiResponse<SetAutoLoanResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let result = response.data.first();
        assert!(result.is_some(), "Expected at least one result in response");
        let result = result.unwrap();
        assert!(result.auto_loan);
    }
}
