use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const CREATE_SUB_ACCOUNT_ENDPOINT: &str = "/openApi/subAccount/v1/create";

/// Request for creating a sub-account
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountRequest {
    /// Sub account username (Starting with a letter, containing a number, and longer than 6 characters)
    pub sub_account_string: String,

    /// Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    /// Request validity window in milliseconds
    pub recv_window: i64,

    /// Request timestamp in milliseconds
    pub timestamp: i64,
}

/// Response for creating a sub-account
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountResponse {
    /// Sub account UID
    pub sub_uid: i64,

    /// Sub account username
    pub sub_account_string: String,

    /// Sub account note information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl RestClient {
    /// Create a sub-account
    ///
    /// Used to create a sub-account through the API key of the master account.
    /// The user who verifies the signature of this API must be main account.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/common/sub-account#Create%20a%20sub-account)
    ///
    /// Rate limit: UID 1/s
    ///
    /// # Arguments
    /// * `request` - The create sub-account request
    ///
    /// # Returns
    /// A result containing the create sub-account response or an error
    pub async fn create_sub_account(
        &self,
        request: &CreateSubAccountRequest,
    ) -> RestResult<CreateSubAccountResponse> {
        self.send_post_signed_request(CREATE_SUB_ACCOUNT_ENDPOINT, request, EndpointType::Account)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sub_account_request_serialization() {
        let request = CreateSubAccountRequest {
            sub_account_string: "test_sub_account123".to_string(),
            note: Some("Test sub account".to_string()),
            recv_window: 5000,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAccountString\":\"test_sub_account123\""));
        assert!(json.contains("\"note\":\"Test sub account\""));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_create_sub_account_request_serialization_without_note() {
        let request = CreateSubAccountRequest {
            sub_account_string: "test_sub_account123".to_string(),
            note: None,
            recv_window: 5000,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAccountString\":\"test_sub_account123\""));
        assert!(!json.contains("\"note\""));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_create_sub_account_response_deserialization() {
        let json = r#"
        {
            "subUid": 123456789,
            "subAccountString": "test_sub_account123",
            "note": "Test sub account"
        }
        "#;

        let response: CreateSubAccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.sub_uid, 123456789);
        assert_eq!(response.sub_account_string, "test_sub_account123");
        assert_eq!(response.note, Some("Test sub account".to_string()));
    }

    #[test]
    fn test_create_sub_account_response_deserialization_without_note() {
        let json = r#"
        {
            "subUid": 123456789,
            "subAccountString": "test_sub_account123"
        }
        "#;

        let response: CreateSubAccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.sub_uid, 123456789);
        assert_eq!(response.sub_account_string, "test_sub_account123");
        assert_eq!(response.note, None);
    }
}
