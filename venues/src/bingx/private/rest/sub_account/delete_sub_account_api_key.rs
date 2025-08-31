use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const DELETE_SUB_ACCOUNT_API_KEY_ENDPOINT: &str = "/openApi/subAccount/v1/apiKey/del";

/// Request to delete sub-account API key
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubAccountApiKeyRequest {
    /// Sub account UID
    pub sub_uid: i64,

    /// API key to delete
    pub api_key: String,

    /// Request validity window in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Request timestamp in milliseconds
    pub timestamp: i64,
}

/// Response for deleting sub-account API key
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubAccountApiKeyResponse {
    /// Whether deletion was successful
    pub success: bool,
}

impl RestClient {
    /// Delete API key for sub-account
    ///
    /// Deletes an API key for the specified sub-account.
    /// The user who verifies the signature of this API must be main account.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/common/sub-account#Delete%20API%20key%20for%20sub-account)
    ///
    /// Rate limit: UID 5/s & IP 2/s
    ///
    /// # Arguments
    /// * `request` - The delete API key request parameters
    ///
    /// # Returns
    /// A result containing the deletion confirmation or an error
    pub async fn delete_sub_account_api_key(
        &self,
        request: &DeleteSubAccountApiKeyRequest,
    ) -> RestResult<DeleteSubAccountApiKeyResponse> {
        self.send_post_signed_request(
            DELETE_SUB_ACCOUNT_API_KEY_ENDPOINT,
            request,
            EndpointType::Account,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_sub_account_api_key_request_serialization() {
        let request = DeleteSubAccountApiKeyRequest {
            sub_uid: 123456789,
            api_key: "test-api-key".to_string(),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUid\":123456789"));
        assert!(json.contains("\"apiKey\":\"test-api-key\""));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_delete_sub_account_api_key_request_serialization_without_recv_window() {
        let request = DeleteSubAccountApiKeyRequest {
            sub_uid: 123456789,
            api_key: "test-api-key".to_string(),
            recv_window: None,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUid\":123456789"));
        assert!(json.contains("\"apiKey\":\"test-api-key\""));
        assert!(!json.contains("\"recvWindow\""));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_delete_sub_account_api_key_response_deserialization() {
        let json = r#"
        {
            "success": true
        }
        "#;

        let response: DeleteSubAccountApiKeyResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
    }

    #[test]
    fn test_delete_response_failure() {
        let json = r#"
        {
            "success": false
        }
        "#;

        let response: DeleteSubAccountApiKeyResponse = serde_json::from_str(json).unwrap();
        assert!(!response.success);
    }
}
