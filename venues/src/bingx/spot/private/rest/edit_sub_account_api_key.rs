use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const EDIT_SUB_ACCOUNT_API_KEY_ENDPOINT: &str = "/openApi/subAccount/v1/apiKey/edit";

/// Request to edit sub-account API key permissions
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditSubAccountApiKeyRequest {
    /// Sub account UID
    pub sub_uid: i64,

    /// API key to edit
    pub api_key: String,

    /// Notes
    pub note: String,

    /// Permissions array: 1-Spot Trading, 2-Read, 3-Perpetual Futures Trading, 4-Universal Transfer, 7-Allow internal transfer of sub accounts
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

/// Response for editing sub-account API key
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditSubAccountApiKeyResponse {
    /// Permissions array: 1-Spot Trading, 2-Read, 3-Perpetual Futures Trading, 4-Universal Transfer, 7-Allow internal transfer of sub accounts
    pub permissions: Vec<i32>,

    /// IP whitelist
    pub ip_addresses: Vec<String>,

    /// Notes
    pub note: String,
}

impl RestClient {
    /// Edit API key for sub-account
    ///
    /// Edits the API key remarks, permissions, and IP addresses of the sub-account.
    /// The user who verifies the signature of this API must be main account.
    ///
    /// # Arguments
    /// * `request` - The edit API key request parameters
    ///
    /// # Returns
    /// A result containing the updated API key details or an error
    ///
    /// # Rate Limits
    /// - UID rate limit: 5/s
    /// - IP rate limit: 2/s
    ///
    /// # API Permissions
    /// - Manage Subaccounts permission required
    pub async fn edit_sub_account_api_key(
        &self,
        request: &EditSubAccountApiKeyRequest,
    ) -> RestResult<EditSubAccountApiKeyResponse> {
        self.send_request(
            EDIT_SUB_ACCOUNT_API_KEY_ENDPOINT,
            reqwest::Method::POST,
            Some(request),
            EndpointType::Account,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_sub_account_api_key_request_serialization() {
        let request = EditSubAccountApiKeyRequest {
            sub_uid: 123456789,
            api_key: "test-api-key".to_string(),
            note: "Test API key".to_string(),
            permissions: vec![1, 2, 3], // Spot Trading, Read, Perpetual Futures Trading
            ip_addresses: Some(vec!["192.168.1.1".to_string(), "192.168.1.2".to_string()]),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUid\":123456789"));
        assert!(json.contains("\"apiKey\":\"test-api-key\""));
        assert!(json.contains("\"note\":\"Test API key\""));
        assert!(json.contains("\"permissions\":[1,2,3]"));
        assert!(json.contains("\"ipAddresses\":[\"192.168.1.1\",\"192.168.1.2\"]"));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_edit_sub_account_api_key_response_deserialization() {
        let json = r#"
        {
            "permissions": [1, 2, 3],
            "ipAddresses": ["192.168.1.1", "192.168.1.2"],
            "note": "Test API key"
        }
        "#;

        let response: EditSubAccountApiKeyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.permissions, vec![1, 2, 3]);
        assert_eq!(response.ip_addresses, vec!["192.168.1.1", "192.168.1.2"]);
        assert_eq!(response.note, "Test API key");
    }

    #[test]
    fn test_minimal_edit_request() {
        let request = EditSubAccountApiKeyRequest {
            sub_uid: 123456789,
            api_key: "test-api-key".to_string(),
            note: "Minimal test".to_string(),
            permissions: vec![2], // Read only
            ip_addresses: None,
            recv_window: None,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUid\":123456789"));
        assert!(json.contains("\"apiKey\":\"test-api-key\""));
        assert!(json.contains("\"note\":\"Minimal test\""));
        assert!(json.contains("\"permissions\":[2]"));
        assert!(!json.contains("\"ipAddresses\""));
        assert!(!json.contains("\"recvWindow\""));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }
}
