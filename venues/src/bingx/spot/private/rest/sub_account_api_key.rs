use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const CREATE_SUB_ACCOUNT_API_KEY_ENDPOINT: &str = "/openApi/subAccount/v1/apiKey/create";
const QUERY_API_KEY_ENDPOINT: &str = "/openApi/account/v1/apiKey/query";

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

/// Request to query API key
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryApiKeyRequest {
    /// User UID
    pub uid: i64,

    /// API key (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    /// Request validity window in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Request timestamp in milliseconds
    pub timestamp: i64,
}

/// API key information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyInfo {
    /// API key
    pub api_key: String,

    /// Notes
    pub note: String,

    /// Permissions array: 1-Spot Trading, 2-Read, 3-Perpetual Futures Trading, 4-Universal Transfer, 7-Allow internal transfer of sub accounts
    pub permissions: Vec<i32>,

    /// IP whitelist
    pub ip_addresses: Vec<String>,

    /// Creation time
    pub create_time: i64,

    /// Update time
    pub update_time: i64,
}

/// Response for querying API key
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryApiKeyResponse {
    /// List of API key information
    #[serde(default)]
    pub data: Vec<ApiKeyInfo>,
}

impl RestClient {
    /// Create API key for sub-account
    ///
    /// Creates a new API key for the specified sub-account.
    /// The user who verifies the signature of this API must be main account.
    ///
    /// # Arguments
    /// * `request` - The create API key request parameters
    ///
    /// # Returns
    /// A result containing the new API key details or an error
    ///
    /// # Rate Limits
    /// - UID rate limit: 5/s
    /// - IP rate limit: 2/s
    ///
    /// # API Permissions
    /// - Manage Subaccounts permission required
    pub async fn create_sub_account_api_key(
        &self,
        request: &CreateSubAccountApiKeyRequest,
    ) -> RestResult<CreateSubAccountApiKeyResponse> {
        self.send_post_signed_request(CREATE_SUB_ACCOUNT_API_KEY_ENDPOINT, request, EndpointType::Account,
        )
        .await
    }

    /// Query API key information
    ///
    /// Query the API key information of the main account or sub-account.
    /// The user who verifies the signature of this API must be main account.
    ///
    /// # Arguments
    /// * `request` - The query API key request parameters
    ///
    /// # Returns
    /// A result containing the API key details or an error
    ///
    /// # Rate Limits
    /// - UID rate limit: 5/s
    /// - IP rate limit: 2/s
    ///
    /// # API Permissions
    /// - Read permission required
    pub async fn query_api_key(
        &self,
        request: &QueryApiKeyRequest,
    ) -> RestResult<QueryApiKeyResponse> {
        self.send_get_signed_request(QUERY_API_KEY_ENDPOINT, request, EndpointType::Account,
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
    fn test_query_api_key_request_serialization() {
        let request = QueryApiKeyRequest {
            uid: 123456789,
            api_key: Some("test-api-key".to_string()),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"uid\":123456789"));
        assert!(json.contains("\"apiKey\":\"test-api-key\""));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_query_api_key_response_deserialization() {
        let json = r#"
        {
            "data": [
                {
                    "apiKey": "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
                    "note": "Test API key",
                    "permissions": [1, 2, 3],
                    "ipAddresses": ["192.168.1.1"],
                    "createTime": 1640995200000,
                    "updateTime": 1640995300000
                }
            ]
        }
        "#;

        let response: QueryApiKeyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);

        let api_key_info = &response.data[0];
        assert_eq!(
            api_key_info.api_key,
            "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A"
        );
        assert_eq!(api_key_info.note, "Test API key");
        assert_eq!(api_key_info.permissions, vec![1, 2, 3]);
        assert_eq!(api_key_info.ip_addresses, vec!["192.168.1.1"]);
        assert_eq!(api_key_info.create_time, 1640995200000);
        assert_eq!(api_key_info.update_time, 1640995300000);
    }
}
