use serde::{Deserialize, Serialize};

/// Request to edit sub-account API key permissions
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditSubAccountApiKeyRequest {
    /// Sub-account UID
    pub sub_uid: String,
    /// API key to edit
    pub api_key: String,
    /// Can trade spot (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_trade: Option<bool>,
    /// Can access margin (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_trade: Option<bool>,
    /// Can access futures (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub futures_trade: Option<bool>,
    /// IP whitelist (comma-separated, optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// API key permission details
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyPermissions {
    /// Can read account information
    pub read: bool,
    /// Can trade spot
    pub trade: bool,
    /// Can access futures
    pub futures: bool,
    /// Can access margin
    pub margin: bool,
    /// IP restrictions
    pub ip_restriction: bool,
    /// Allowed IP addresses
    pub ip_list: Vec<String>,
}

/// API key information
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyInfo {
    /// API key
    pub api_key: String,
    /// API key status
    pub status: String,
    /// Creation time
    pub create_time: i64,
    /// Permissions
    pub permissions: ApiKeyPermissions,
}

/// Response for editing sub-account API key
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EditSubAccountApiKeyResponse {
    /// Success indicator
    pub success: bool,
    /// API key information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ApiKeyInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_sub_account_api_key_request_serialization() {
        let request = EditSubAccountApiKeyRequest {
            sub_uid: "12345".to_string(),
            api_key: "test-api-key".to_string(),
            can_trade: Some(true),
            margin_trade: Some(false),
            futures_trade: Some(true),
            ip: Some("192.168.1.1,192.168.1.2".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUid\":\"12345\""));
        assert!(json.contains("\"apiKey\":\"test-api-key\""));
        assert!(json.contains("\"canTrade\":true"));
        assert!(json.contains("\"marginTrade\":false"));
        assert!(json.contains("\"futuresTrade\":true"));
        assert!(json.contains("\"ip\":\"192.168.1.1,192.168.1.2\""));
    }

    #[test]
    fn test_edit_sub_account_api_key_response_deserialization() {
        let json = r#"
        {
            "success": true,
            "data": {
                "apiKey": "test-api-key",
                "status": "ACTIVE",
                "createTime": 1640995200000,
                "permissions": {
                    "read": true,
                    "trade": true,
                    "futures": true,
                    "margin": false,
                    "ipRestriction": true,
                    "ipList": ["192.168.1.1", "192.168.1.2"]
                }
            }
        }
        "#;

        let response: EditSubAccountApiKeyResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        
        let data = response.data.unwrap();
        assert_eq!(data.api_key, "test-api-key");
        assert_eq!(data.status, "ACTIVE");
        assert_eq!(data.create_time, 1640995200000);
        
        let permissions = data.permissions;
        assert!(permissions.read);
        assert!(permissions.trade);
        assert!(permissions.futures);
        assert!(!permissions.margin);
        assert!(permissions.ip_restriction);
        assert_eq!(permissions.ip_list, vec!["192.168.1.1", "192.168.1.2"]);
    }

    #[test]
    fn test_minimal_edit_request() {
        let request = EditSubAccountApiKeyRequest {
            sub_uid: "12345".to_string(),
            api_key: "test-api-key".to_string(),
            can_trade: None,
            margin_trade: None,
            futures_trade: None,
            ip: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUid\":\"12345\""));
        assert!(json.contains("\"apiKey\":\"test-api-key\""));
        assert!(!json.contains("canTrade"));
        assert!(!json.contains("marginTrade"));
        assert!(!json.contains("futuresTrade"));
        assert!(!json.contains("ip"));
    }
}
