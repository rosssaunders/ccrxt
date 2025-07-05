use crate::bingx::enums::ApiPermission;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountApiKeyRequest {
    /// Sub-account email
    pub sub_account: String,
    /// Whether the API key can trade
    pub can_trade: bool,
    /// Whether the API key can read margin account
    pub margin_trade: bool,
    /// Whether the API key can read futures account
    pub futures_trade: bool,
    /// Timestamp in ms
    pub recv_window: Option<i64>,
    /// Timestamp in ms
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountApiKeyResponse {
    /// API key
    pub api_key: String,
    /// Secret key
    pub secret_key: String,
    /// Whether the API key can trade
    pub can_trade: bool,
    /// Whether the API key can read margin account
    pub margin_trade: bool,
    /// Whether the API key can read futures account
    pub futures_trade: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountApiKeyRequest {
    /// Sub-account email
    pub sub_account: String,
    /// API key (optional, if not provided returns all keys)
    pub api_key: Option<String>,
    /// Timestamp in ms
    pub recv_window: Option<i64>,
    /// Timestamp in ms
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountApiKeyResponse {
    /// List of API keys
    pub sub_account_api_keys: Vec<SubAccountApiKey>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountApiKey {
    /// Sub-account email
    pub sub_account: String,
    /// API key
    pub api_key: String,
    /// Whether the API key can trade
    pub can_trade: bool,
    /// Whether the API key can read margin account
    pub margin_trade: bool,
    /// Whether the API key can read futures account
    pub futures_trade: bool,
    /// Creation time
    pub create_time: i64,
    /// IP restrictions
    pub ip_restrict: bool,
    /// Enabled status
    pub enable: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubAccountApiKeyRequest {
    /// Sub-account email
    pub sub_account: String,
    /// API key to delete
    pub api_key: String,
    /// Timestamp in ms
    pub recv_window: Option<i64>,
    /// Timestamp in ms
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubAccountApiKeyResponse {
    /// Whether deletion was successful
    pub success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sub_account_api_key_request_serialization() {
        let request = CreateSubAccountApiKeyRequest {
            sub_account: "sub@example.com".to_string(),
            can_trade: true,
            margin_trade: false,
            futures_trade: false,
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAccount\":\"sub@example.com\""));
        assert!(json.contains("\"canTrade\":true"));
        assert!(json.contains("\"marginTrade\":false"));
        assert!(json.contains("\"futuresTrade\":false"));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_create_sub_account_api_key_response_deserialization() {
        let json = r#"
        {
            "apiKey": "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
            "secretKey": "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j",
            "canTrade": true,
            "marginTrade": false,
            "futuresTrade": false
        }
        "#;

        let response: CreateSubAccountApiKeyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response.api_key,
            "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A"
        );
        assert_eq!(
            response.secret_key,
            "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j"
        );
        assert!(response.can_trade);
        assert!(!response.margin_trade);
        assert!(!response.futures_trade);
    }

    #[test]
    fn test_get_sub_account_api_key_response_deserialization() {
        let json = r#"
        {
            "subAccountApiKeys": [
                {
                    "subAccount": "sub@example.com",
                    "apiKey": "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
                    "canTrade": true,
                    "marginTrade": false,
                    "futuresTrade": false,
                    "createTime": 1640995200000,
                    "ipRestrict": false,
                    "enable": true
                }
            ]
        }
        "#;

        let response: GetSubAccountApiKeyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.sub_account_api_keys.len(), 1);
        assert_eq!(
            response.sub_account_api_keys[0].sub_account,
            "sub@example.com"
        );
        assert!(response.sub_account_api_keys[0].can_trade);
        assert!(response.sub_account_api_keys[0].enable);
    }
}
