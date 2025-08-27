use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Endpoint URL for modifying API key of sub-account
const MODIFY_APIKEY_ENDPOINT: &str = "api/v5/users/subaccount/modify-apikey";

/// Request to reset/modify API key of sub-account
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyApikeyRequest {
    /// Sub-account name
    pub sub_acct: String,

    /// Sub-account API Key
    pub api_key: String,

    /// Sub-account API Key label. The label will be reset if this is passed through.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Sub-account API Key permissions
    /// read_only: Read
    /// trade: Trade  
    /// Separate with commas if more than one.
    /// The permission will be reset if this is passed through.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perm: Option<String>,

    /// Sub-account API Key linked IP addresses, separate with commas if more than one.
    /// Support up to 20 IP addresses.
    /// The IP will be reset if this is passed through.
    /// If ip is set to "", then no IP addresses is linked to the APIKey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// Response from modifying API key of sub-account
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyApikeyResponse {
    /// Sub-account name
    pub sub_acct: String,

    /// Sub-account API public key
    pub api_key: String,

    /// Sub-account API Key label
    pub label: String,

    /// Sub-account API Key permissions
    /// read_only: Read, trade: Trade
    pub perm: String,

    /// Sub-account API Key IP addresses that linked with API Key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// Creation time
    pub ts: String,
}

impl RestClient {
    /// Reset the API Key of a sub-account
    ///
    /// Applies to master accounts only and master accounts API Key must be linked to IP addresses.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#sub-account-rest-api-reset-the-api-key-of-a-sub-account)
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The modify API key request parameters
    ///
    /// # Returns
    /// A result containing the modified API key information
    pub async fn modify_apikey(
        &self,
        request: ModifyApikeyRequest,
    ) -> RestResult<ModifyApikeyResponse> {
        self.send_post_request(
            MODIFY_APIKEY_ENDPOINT,
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
    fn test_modify_apikey_request_serialization() {
        let request = ModifyApikeyRequest {
            sub_acct: "test_sub_001".to_string(),
            api_key: "3b8e8e4f-5c5e-4f5a-9a9a-1b1c1d1e1f1g".to_string(),
            label: Some("Updated API Key Label".to_string()),
            perm: Some("trade".to_string()),
            ip: Some("192.168.1.100,192.168.1.101".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAcct\":\"test_sub_001\""));
        assert!(json.contains("\"apiKey\":\"3b8e8e4f-5c5e-4f5a-9a9a-1b1c1d1e1f1g\""));
        assert!(json.contains("\"label\":\"Updated API Key Label\""));
        assert!(json.contains("\"perm\":\"trade\""));
        assert!(json.contains("\"ip\":\"192.168.1.100,192.168.1.101\""));
    }

    #[test]
    fn test_modify_apikey_request_minimal() {
        let request = ModifyApikeyRequest {
            sub_acct: "test_sub_002".to_string(),
            api_key: "5d0d0d6h-7e7g-6h7c-cd1c-3d3e3f3g3h3i".to_string(),
            label: None,
            perm: None,
            ip: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAcct\":\"test_sub_002\""));
        assert!(json.contains("\"apiKey\":\"5d0d0d6h-7e7g-6h7c-cd1c-3d3e3f3g3h3i\""));
        assert!(!json.contains("label"));
        assert!(!json.contains("perm"));
        assert!(!json.contains("ip"));
    }

    #[test]
    fn test_modify_apikey_request_empty_ip() {
        let request = ModifyApikeyRequest {
            sub_acct: "test_sub_003".to_string(),
            api_key: "7f2f2f8j-9g9i-8j9e-ef3e-5f5g5h5i5j5k".to_string(),
            label: Some("No IP Key".to_string()),
            perm: Some("read_only".to_string()),
            ip: Some("".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAcct\":\"test_sub_003\""));
        assert!(json.contains("\"apiKey\":\"7f2f2f8j-9g9i-8j9e-ef3e-5f5g5h5i5j5k\""));
        assert!(json.contains("\"label\":\"No IP Key\""));
        assert!(json.contains("\"perm\":\"read_only\""));
        assert!(json.contains("\"ip\":\"\""));
    }

    #[test]
    fn test_modify_apikey_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "subAcct": "test_sub_001",
                    "apiKey": "3b8e8e4f-5c5e-4f5a-9a9a-1b1c1d1e1f1g",
                    "label": "Updated API Key Label",
                    "perm": "trade",
                    "ip": "192.168.1.100,192.168.1.101",
                    "ts": "1597026383085"
                }
            ]
        }"#;

        let response: ApiResponse<ModifyApikeyResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let apikey = &response.data[0];
        assert_eq!(apikey.sub_acct, "test_sub_001");
        assert_eq!(apikey.api_key, "3b8e8e4f-5c5e-4f5a-9a9a-1b1c1d1e1f1g");
        assert_eq!(apikey.label, "Updated API Key Label");
        assert_eq!(apikey.perm, "trade");
        assert_eq!(apikey.ip, Some("192.168.1.100,192.168.1.101".to_string()));
        assert_eq!(apikey.ts, "1597026383085");
    }

    #[test]
    fn test_modify_apikey_response_deserialization_no_ip() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "subAcct": "test_sub_003",
                    "apiKey": "7f2f2f8j-9g9i-8j9e-ef3e-5f5g5h5i5j5k",
                    "label": "No IP Key",
                    "perm": "read_only",
                    "ts": "1597026383087"
                }
            ]
        }"#;

        let response: ApiResponse<ModifyApikeyResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let apikey = &response.data[0];
        assert_eq!(apikey.sub_acct, "test_sub_003");
        assert_eq!(apikey.api_key, "7f2f2f8j-9g9i-8j9e-ef3e-5f5g5h5i5j5k");
        assert_eq!(apikey.label, "No IP Key");
        assert_eq!(apikey.perm, "read_only");
        assert!(apikey.ip.is_none());
        assert_eq!(apikey.ts, "1597026383087");
    }
}
