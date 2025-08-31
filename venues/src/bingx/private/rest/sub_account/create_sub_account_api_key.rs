use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const CREATE_SUB_ACCOUNT_API_KEY_ENDPOINT: &str = "/openApi/subAccount/v1/apiKey/create";

/// Request to create API key for sub-account
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountApiKeyRequest {
    /// Sub account UID
    pub sub_uid: i64,

    /// Notes
    pub note: String,

    /// Permissions array: 1-Spot Trading, 2-Read, 3-Perpetual Futures Trading, 4-Universal Transfer, 5-Withdraw, 7-Allow internal transfer of sub accounts
    pub permissions: Vec<i32>,

    /// IP whitelist (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<String>>,

    /// Request validity window in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Request timestamp in milliseconds
    pub timestamp: i64,
}

/// Response for creating sub-account API key
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountApiKeyResponse {
    /// API key
    pub api_key: String,

    /// API secret
    pub api_secret: String,

    /// Permissions array: 1-Spot Trading, 2-Read, 3-Perpetual Futures Trading, 4-Universal Transfer, 5-Withdraw, 7-Allow internal transfer of sub accounts
    pub permissions: Vec<i32>,

    /// IP whitelist
    pub ip_addresses: Vec<String>,

    /// Notes
    pub note: String,
}

impl RestClient {
    /// Create API key for sub-account
    ///
    /// Creates a new API key for the specified sub-account.
    /// The user who verifies the signature of this API must be main account.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/common/sub-account#Create%20API%20key%20for%20sub-account)
    ///
    /// Rate limit: UID 5/s & IP 2/s
    ///
    /// # Arguments
    /// * `request` - The create API key request parameters
    ///
    /// # Returns
    /// A result containing the new API key details or an error
    pub async fn create_sub_account_api_key(
        &self,
        request: &CreateSubAccountApiKeyRequest,
    ) -> RestResult<CreateSubAccountApiKeyResponse> {
        self.send_post_signed_request(
            CREATE_SUB_ACCOUNT_API_KEY_ENDPOINT,
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
    fn test_create_sub_account_api_key_request_serialization() {
        let request = CreateSubAccountApiKeyRequest {
            sub_uid: 123456789,
            note: "Test API key".to_string(),
            permissions: vec![1, 2, 3], // Spot Trading, Read, Perpetual Futures Trading
            ip_addresses: Some(vec!["192.168.1.1".to_string()]),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUid\":123456789"));
        assert!(json.contains("\"note\":\"Test API key\""));
        assert!(json.contains("\"permissions\":[1,2,3]"));
        assert!(json.contains("\"ipAddresses\":[\"192.168.1.1\"]"));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_create_sub_account_api_key_response_deserialization() {
        let json = r#"
        {
            "apiKey": "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
            "apiSecret": "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j",
            "permissions": [1, 2, 3],
            "ipAddresses": ["192.168.1.1"],
            "note": "Test API key"
        }
        "#;

        let response: CreateSubAccountApiKeyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response.api_key,
            "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A"
        );
        assert_eq!(
            response.api_secret,
            "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j"
        );
        assert_eq!(response.permissions, vec![1, 2, 3]);
        assert_eq!(response.ip_addresses, vec!["192.168.1.1"]);
        assert_eq!(response.note, "Test API key");
    }

    #[test]
    fn test_create_sub_account_api_key_request_without_optional_fields() {
        let request = CreateSubAccountApiKeyRequest {
            sub_uid: 987654321,
            note: "Minimal API key".to_string(),
            permissions: vec![2], // Read only
            ip_addresses: None,
            recv_window: None,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUid\":987654321"));
        assert!(json.contains("\"note\":\"Minimal API key\""));
        assert!(json.contains("\"permissions\":[2]"));
        assert!(!json.contains("\"ipAddresses\""));
        assert!(!json.contains("\"recvWindow\""));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_create_sub_account_api_key_response_with_multiple_permissions() {
        let json = r#"
        {
            "apiKey": "test-key",
            "apiSecret": "test-secret",
            "permissions": [1, 2, 3, 4, 7],
            "ipAddresses": ["192.168.1.1", "10.0.0.1"],
            "note": "Multi-permission key"
        }
        "#;

        let response: CreateSubAccountApiKeyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.permissions, vec![1, 2, 3, 4, 7]);
        assert_eq!(response.ip_addresses.len(), 2);
    }
}
