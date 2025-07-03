use serde::{Deserialize, Serialize};
use crate::bingx::enums::SubAccountStatus;

/// Request to freeze/unfreeze sub-account
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FreezeSubAccountRequest {
    /// Sub-account UID
    pub sub_uid: String,
    /// Action: true for freeze, false for unfreeze
    pub is_freeze: bool,
}

/// Response for freeze/unfreeze sub-account
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FreezeSubAccountResponse {
    /// Success indicator
    pub success: bool,
    /// Sub-account details after operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<FreezeSubAccountData>,
}

/// Sub-account data after freeze/unfreeze operation
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FreezeSubAccountData {
    /// Sub-account UID
    pub sub_uid: String,
    /// Sub-account email
    pub email: String,
    /// Account status after operation
    pub status: SubAccountStatus,
    /// Operation timestamp
    pub update_time: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freeze_sub_account_request_serialization() {
        let request = FreezeSubAccountRequest {
            sub_uid: "12345".to_string(),
            is_freeze: true,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUid\":\"12345\""));
        assert!(json.contains("\"isFreeze\":true"));
    }

    #[test]
    fn test_unfreeze_sub_account_request_serialization() {
        let request = FreezeSubAccountRequest {
            sub_uid: "12345".to_string(),
            is_freeze: false,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUid\":\"12345\""));
        assert!(json.contains("\"isFreeze\":false"));
    }

    #[test]
    fn test_freeze_sub_account_response_deserialization() {
        let json = r#"
        {
            "success": true,
            "data": {
                "subUid": "12345",
                "email": "subaccount@example.com",
                "status": "FROZEN",
                "updateTime": 1640995200000
            }
        }
        "#;

        let response: FreezeSubAccountResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        
        let data = response.data.unwrap();
        assert_eq!(data.sub_uid, "12345");
        assert_eq!(data.email, "subaccount@example.com");
        assert_eq!(data.status, SubAccountStatus::Frozen);
        assert_eq!(data.update_time, 1640995200000);
    }

    #[test]
    fn test_unfreeze_response() {
        let json = r#"
        {
            "success": true,
            "data": {
                "subUid": "12345",
                "email": "subaccount@example.com",
                "status": "NORMAL",
                "updateTime": 1640995200000
            }
        }
        "#;

        let response: FreezeSubAccountResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        
        let data = response.data.unwrap();
        assert_eq!(data.status, SubAccountStatus::Normal);
    }
}
