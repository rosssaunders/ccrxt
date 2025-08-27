use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Endpoint URL for querying API key of sub-account
const QUERY_APIKEY_ENDPOINT: &str = "api/v5/users/subaccount/apikey";

/// Request to query API key of sub-account
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryApikeyRequest {
    /// Sub-account name
    pub sub_acct: String,

    /// API public key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
}

/// Response from querying API key of sub-account
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryApikeyResponse {
    /// API Key note
    pub label: String,

    /// API public key
    pub api_key: String,

    /// API Key access
    /// read_only: Read only; trade: Trade
    pub perm: String,

    /// IP address that linked with API Key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// Creation time
    pub ts: String,
}

impl RestClient {
    /// Query the API Key of a sub-account
    ///
    /// Applies to master accounts only
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#sub-account-rest-api-query-the-api-key-of-a-sub-account)
    ///
    /// Rate limit: 20 request per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The query API key request parameters
    ///
    /// # Returns
    /// A result containing the API key information for the sub-account
    pub async fn query_apikey(
        &self,
        request: QueryApikeyRequest,
    ) -> RestResult<QueryApikeyResponse> {
        self.send_get_request(QUERY_APIKEY_ENDPOINT, request, EndpointType::PrivateAccount)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_query_apikey_request_serialization() {
        let request = QueryApikeyRequest {
            sub_acct: "test_sub_001".to_string(),
            api_key: Some("3b8e8e4f-5c5e-4f5a-9a9a-1b1c1d1e1f1g".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("subAcct=test_sub_001"));
        assert!(serialized.contains("apiKey=3b8e8e4f-5c5e-4f5a-9a9a-1b1c1d1e1f1g"));
    }

    #[test]
    fn test_query_apikey_request_without_apikey() {
        let request = QueryApikeyRequest {
            sub_acct: "test_sub_002".to_string(),
            api_key: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("subAcct=test_sub_002"));
        assert!(!serialized.contains("apiKey"));
    }

    #[test]
    fn test_query_apikey_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "label": "Test API Key",
                    "apiKey": "3b8e8e4f-5c5e-4f5a-9a9a-1b1c1d1e1f1g",
                    "perm": "trade",
                    "ip": "192.168.1.1,192.168.1.2",
                    "ts": "1597026383085"
                }
            ]
        }"#;

        let response: ApiResponse<QueryApikeyResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let apikey = &response.data[0];
        assert_eq!(apikey.label, "Test API Key");
        assert_eq!(apikey.api_key, "3b8e8e4f-5c5e-4f5a-9a9a-1b1c1d1e1f1g");
        assert_eq!(apikey.perm, "trade");
        assert_eq!(apikey.ip, Some("192.168.1.1,192.168.1.2".to_string()));
        assert_eq!(apikey.ts, "1597026383085");
    }

    #[test]
    fn test_query_apikey_response_deserialization_no_ip() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "label": "Read Only Key",
                    "apiKey": "5d0d0d6h-7e7g-6h7c-cd1c-3d3e3f3g3h3i",
                    "perm": "read_only",
                    "ts": "1597026383086"
                }
            ]
        }"#;

        let response: ApiResponse<QueryApikeyResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let apikey = &response.data[0];
        assert_eq!(apikey.label, "Read Only Key");
        assert_eq!(apikey.api_key, "5d0d0d6h-7e7g-6h7c-cd1c-3d3e3f3g3h3i");
        assert_eq!(apikey.perm, "read_only");
        assert!(apikey.ip.is_none());
        assert_eq!(apikey.ts, "1597026383086");
    }

    #[test]
    fn test_query_apikey_response_multiple_keys() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "label": "Trading Key",
                    "apiKey": "3b8e8e4f-5c5e-4f5a-9a9a-1b1c1d1e1f1g",
                    "perm": "trade",
                    "ip": "192.168.1.1",
                    "ts": "1597026383085"
                },
                {
                    "label": "Read Only Key",
                    "apiKey": "5d0d0d6h-7e7g-6h7c-cd1c-3d3e3f3g3h3i",
                    "perm": "read_only",
                    "ts": "1597026383086"
                }
            ]
        }"#;

        let response: ApiResponse<QueryApikeyResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);

        let trading_key = &response.data[0];
        assert_eq!(trading_key.label, "Trading Key");
        assert_eq!(trading_key.perm, "trade");
        assert_eq!(trading_key.ip, Some("192.168.1.1".to_string()));

        let readonly_key = &response.data[1];
        assert_eq!(readonly_key.label, "Read Only Key");
        assert_eq!(readonly_key.perm, "read_only");
        assert!(readonly_key.ip.is_none());
    }
}
