use serde::{Deserialize, Serialize};

/// Request to delete sub-account API key
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubAccountApiKeyRequest {
    /// Sub-account UID
    pub sub_uid: String,
    /// API key to delete
    pub api_key: String,
}

/// Response for deleting sub-account API key
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteSubAccountApiKeyResponse {
    /// Success indicator
    pub success: bool,
    /// Message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_sub_account_api_key_request_serialization() {
        let request = DeleteSubAccountApiKeyRequest {
            sub_uid: "12345".to_string(),
            api_key: "test-api-key".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUid\":\"12345\""));
        assert!(json.contains("\"apiKey\":\"test-api-key\""));
    }

    #[test]
    fn test_delete_sub_account_api_key_response_deserialization() {
        let json = r#"
        {
            "success": true,
            "msg": "API key deleted successfully"
        }
        "#;

        let response: DeleteSubAccountApiKeyResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        assert_eq!(response.msg.unwrap(), "API key deleted successfully");
    }

    #[test]
    fn test_delete_response_without_message() {
        let json = r#"
        {
            "success": true
        }
        "#;

        let response: DeleteSubAccountApiKeyResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        assert!(response.msg.is_none());
    }
}
