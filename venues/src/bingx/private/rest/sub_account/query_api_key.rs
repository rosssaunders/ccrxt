use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const QUERY_API_KEY_ENDPOINT: &str = "/openApi/account/v1/apiKey/query";

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
    /// Query API key information
    ///
    /// Query the API key information of the main account or sub-account.
    /// The user who verifies the signature of this API must be main account.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/common/sub-account#Query%20API%20key%20information)
    ///
    /// Rate limit: UID 5/s & IP 2/s
    ///
    /// # Arguments
    /// * `request` - The query API key request parameters
    ///
    /// # Returns
    /// A result containing the API key details or an error
    pub async fn query_api_key(
        &self,
        request: &QueryApiKeyRequest,
    ) -> RestResult<QueryApiKeyResponse> {
        self.send_get_signed_request(QUERY_API_KEY_ENDPOINT, request, EndpointType::Account)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_query_api_key_request_without_optional_fields() {
        let request = QueryApiKeyRequest {
            uid: 987654321,
            api_key: None,
            recv_window: None,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"uid\":987654321"));
        assert!(!json.contains("\"apiKey\""));
        assert!(!json.contains("\"recvWindow\""));
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

    #[test]
    fn test_query_api_key_response_with_multiple_keys() {
        let json = r#"
        {
            "data": [
                {
                    "apiKey": "key1",
                    "note": "First key",
                    "permissions": [1, 2],
                    "ipAddresses": ["192.168.1.1"],
                    "createTime": 1640995200000,
                    "updateTime": 1640995300000
                },
                {
                    "apiKey": "key2",
                    "note": "Second key",
                    "permissions": [2, 3, 4],
                    "ipAddresses": ["10.0.0.1", "10.0.0.2"],
                    "createTime": 1640995400000,
                    "updateTime": 1640995500000
                }
            ]
        }
        "#;

        let response: QueryApiKeyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 2);

        assert_eq!(response.data[0].api_key, "key1");
        assert_eq!(response.data[0].permissions, vec![1, 2]);

        assert_eq!(response.data[1].api_key, "key2");
        assert_eq!(response.data[1].permissions, vec![2, 3, 4]);
        assert_eq!(response.data[1].ip_addresses.len(), 2);
    }

    #[test]
    fn test_query_api_key_response_empty_data() {
        let json = r#"
        {
            "data": []
        }
        "#;

        let response: QueryApiKeyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 0);
    }
}
