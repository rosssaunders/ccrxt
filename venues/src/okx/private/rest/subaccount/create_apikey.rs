use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Endpoint URL for creating API key for sub-account
const CREATE_APIKEY_ENDPOINT: &str = "api/v5/users/subaccount/apikey";

/// Request to create API key for sub-account
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApikeyRequest {
    /// Sub-account name, supports 6 to 20 characters that include numbers and letters
    /// (case sensitive, space symbol is not supported)
    pub sub_acct: String,

    /// API Key note
    pub label: String,

    /// API Key password, supports 8 to 32 alphanumeric characters containing at least
    /// 1 number, 1 uppercase letter, 1 lowercase letter and 1 special character
    pub passphrase: String,

    /// API Key permissions
    /// read_only: Read only
    /// trade: Trade
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perm: Option<String>,

    /// Link IP addresses, separate with commas if more than one. Support up to 20 addresses.
    /// For security reasons, it is recommended to bind IP addresses.
    /// API keys with trading or withdrawal permissions that are not bound to IPs will
    /// expire after 14 days of inactivity. (API keys in demo trading will not be deleted.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// Response from creating API key for sub-account
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApikeyResponse {
    /// Sub-account name
    pub sub_acct: String,

    /// API Key note
    pub label: String,

    /// API public key
    pub api_key: String,

    /// API private key
    pub secret_key: String,

    /// API Key password
    pub passphrase: String,

    /// API Key access
    /// read_only: Read only, trade: Trade
    pub perm: String,

    /// IP address that linked with API Key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// Creation time
    pub ts: String,
}

impl RestClient {
    /// Create an API Key for a sub-account
    ///
    /// Applies to master accounts only and master accounts API Key must be linked to IP addresses.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#sub-account-rest-api-create-an-api-key-for-a-sub-account)
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The create API key request parameters
    ///
    /// # Returns
    /// A result containing the created API key information including keys and permissions
    pub async fn create_apikey(
        &self,
        request: CreateApikeyRequest,
    ) -> RestResult<CreateApikeyResponse> {
        self.send_post_request(
            CREATE_APIKEY_ENDPOINT,
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
    fn test_create_apikey_request_serialization() {
        let request = CreateApikeyRequest {
            sub_acct: "test_sub_001".to_string(),
            label: "Test API Key".to_string(),
            passphrase: "ApiPass123!".to_string(),
            perm: Some("trade".to_string()),
            ip: Some("192.168.1.1,192.168.1.2".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAcct\":\"test_sub_001\""));
        assert!(json.contains("\"label\":\"Test API Key\""));
        assert!(json.contains("\"passphrase\":\"ApiPass123!\""));
        assert!(json.contains("\"perm\":\"trade\""));
        assert!(json.contains("\"ip\":\"192.168.1.1,192.168.1.2\""));
    }

    #[test]
    fn test_create_apikey_request_minimal() {
        let request = CreateApikeyRequest {
            sub_acct: "test_sub_002".to_string(),
            label: "Read Only Key".to_string(),
            passphrase: "ReadPass456!".to_string(),
            perm: None,
            ip: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAcct\":\"test_sub_002\""));
        assert!(json.contains("\"label\":\"Read Only Key\""));
        assert!(json.contains("\"passphrase\":\"ReadPass456!\""));
        assert!(!json.contains("perm"));
        assert!(!json.contains("ip"));
    }

    #[test]
    fn test_create_apikey_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "subAcct": "test_sub_001",
                    "label": "Test API Key",
                    "apiKey": "3b8e8e4f-5c5e-4f5a-9a9a-1b1c1d1e1f1g",
                    "secretKey": "4c9f9f5g-6d6f-5g6b-ab0b-2c2d2e2f2g2h",
                    "passphrase": "ApiPass123!",
                    "perm": "trade",
                    "ip": "192.168.1.1,192.168.1.2",
                    "ts": "1597026383085"
                }
            ]
        }"#;

        let response: ApiResponse<CreateApikeyResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let apikey = &response.data[0];
        assert_eq!(apikey.sub_acct, "test_sub_001");
        assert_eq!(apikey.label, "Test API Key");
        assert_eq!(apikey.api_key, "3b8e8e4f-5c5e-4f5a-9a9a-1b1c1d1e1f1g");
        assert_eq!(apikey.secret_key, "4c9f9f5g-6d6f-5g6b-ab0b-2c2d2e2f2g2h");
        assert_eq!(apikey.passphrase, "ApiPass123!");
        assert_eq!(apikey.perm, "trade");
        assert_eq!(apikey.ip, Some("192.168.1.1,192.168.1.2".to_string()));
        assert_eq!(apikey.ts, "1597026383085");
    }

    #[test]
    fn test_create_apikey_response_deserialization_no_ip() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "subAcct": "test_sub_002",
                    "label": "Read Only Key",
                    "apiKey": "5d0d0d6h-7e7g-6h7c-cd1c-3d3e3f3g3h3i",
                    "secretKey": "6e1e1e7i-8f8h-7i8d-de2d-4e4f4g4h4i4j",
                    "passphrase": "ReadPass456!",
                    "perm": "read_only",
                    "ts": "1597026383086"
                }
            ]
        }"#;

        let response: ApiResponse<CreateApikeyResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let apikey = &response.data[0];
        assert_eq!(apikey.sub_acct, "test_sub_002");
        assert_eq!(apikey.label, "Read Only Key");
        assert_eq!(apikey.api_key, "5d0d0d6h-7e7g-6h7c-cd1c-3d3e3f3g3h3i");
        assert_eq!(apikey.secret_key, "6e1e1e7i-8f8h-7i8d-de2d-4e4f4g4h4i4j");
        assert_eq!(apikey.passphrase, "ReadPass456!");
        assert_eq!(apikey.perm, "read_only");
        assert!(apikey.ip.is_none());
        assert_eq!(apikey.ts, "1597026383086");
    }
}
