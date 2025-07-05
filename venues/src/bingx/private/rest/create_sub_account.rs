use crate::bingx::enums::SubAccountType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountRequest {
    /// Sub-account name
    pub sub_account: String,
    /// Sub-account type
    pub sub_account_type: SubAccountType,
    /// Timestamp in ms
    pub recv_window: Option<i64>,
    /// Timestamp in ms
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountResponse {
    /// Sub-account email
    pub email: String,
    /// Sub-account UID
    pub sub_account_id: String,
    /// Whether creation was successful
    pub success: bool,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sub_account_request_serialization() {
        let request = CreateSubAccountRequest {
            sub_account: "test_sub_account".to_string(),
            sub_account_type: SubAccountType::Spot,
            timestamp: 1640995200000,
            recv_window: Some(5000),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAccount\":\"test_sub_account\""));
        assert!(json.contains("\"subAccountType\":\"spot\""));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_create_sub_account_response_deserialization() {
        let json = r#"
        {
            "email": "test_sub_account@example.com",
            "subAccountId": "12345678",
            "success": true
        }
        "#;

        let response: CreateSubAccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.email, "test_sub_account@example.com");
        assert_eq!(response.sub_account_id, "12345678");
        assert!(response.success);
    }
}
