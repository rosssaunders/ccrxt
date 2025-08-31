use serde::{Deserialize, Serialize};

use crate::bingx::{
    EndpointType, PrivateRestClient as RestClient, RestResult, enums::SubAccountStatus,
};

const FREEZE_SUB_ACCOUNT_ENDPOINT: &str = "/openApi/subAccount/v1/freeze";

/// Request to freeze/unfreeze sub-account
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FreezeSubAccountRequest {
    /// Sub-account UID
    pub sub_uid: String,

    /// Action: true for freeze, false for unfreeze
    pub is_freeze: bool,

    /// Request validity window in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Request timestamp in milliseconds
    pub timestamp: i64,
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

impl RestClient {
    /// Freeze or unfreeze a sub-account
    ///
    /// Allows the main account to freeze or unfreeze a sub-account.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/common/sub-account#Freeze%20or%20unfreeze%20a%20sub-account)
    ///
    /// Rate limit: UID 1/s
    ///
    /// # Arguments
    /// * `request` - The freeze/unfreeze sub-account request
    ///
    /// # Returns
    /// A result containing the operation response or an error
    pub async fn freeze_sub_account(
        &self,
        request: &FreezeSubAccountRequest,
    ) -> RestResult<FreezeSubAccountResponse> {
        self.send_post_signed_request(FREEZE_SUB_ACCOUNT_ENDPOINT, request, EndpointType::Account)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freeze_sub_account_request_serialization() {
        let request = FreezeSubAccountRequest {
            sub_uid: "12345".to_string(),
            is_freeze: true,
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUid\":\"12345\""));
        assert!(json.contains("\"isFreeze\":true"));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_unfreeze_sub_account_request_serialization() {
        let request = FreezeSubAccountRequest {
            sub_uid: "12345".to_string(),
            is_freeze: false,
            recv_window: None,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUid\":\"12345\""));
        assert!(json.contains("\"isFreeze\":false"));
        assert!(json.contains("\"timestamp\":1640995200000"));
        assert!(!json.contains("\"recvWindow\""));
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
