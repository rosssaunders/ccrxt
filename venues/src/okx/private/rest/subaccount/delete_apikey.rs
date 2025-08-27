use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Endpoint URL for deleting API key of sub-account
const DELETE_APIKEY_ENDPOINT: &str = "api/v5/users/subaccount/delete-apikey";

/// Request to delete API key of sub-account
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteApikeyRequest {
    /// Sub-account name
    pub sub_acct: String,

    /// API public key
    pub api_key: String,
}

/// Response from deleting API key of sub-account
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteApikeyResponse {
    /// Sub-account name
    pub sub_acct: String,
}

impl RestClient {
    /// Delete the API Key of sub-accounts
    ///
    /// Applies to master accounts only and master accounts API Key must be linked to IP addresses.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#sub-account-rest-api-delete-the-api-key-of-sub-accounts)
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The delete API key request parameters
    ///
    /// # Returns
    /// A result containing the sub-account name confirmation
    pub async fn delete_apikey(
        &self,
        request: DeleteApikeyRequest,
    ) -> RestResult<DeleteApikeyResponse> {
        self.send_post_request(
            DELETE_APIKEY_ENDPOINT,
            request,
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_delete_apikey_request_serialization() {
        let request = DeleteApikeyRequest {
            sub_acct: "test_sub_001".to_string(),
            api_key: "3b8e8e4f-5c5e-4f5a-9a9a-1b1c1d1e1f1g".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAcct\":\"test_sub_001\""));
        assert!(json.contains("\"apiKey\":\"3b8e8e4f-5c5e-4f5a-9a9a-1b1c1d1e1f1g\""));
    }

    #[test]
    fn test_delete_apikey_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "subAcct": "test_sub_001"
                }
            ]
        }"#;

        let response: ApiResponse<DeleteApikeyResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let delete_result = &response.data[0];
        assert_eq!(delete_result.sub_acct, "test_sub_001");
    }
}
