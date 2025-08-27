use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Endpoint URL for setting permission of transfer out
const SET_TRANSFER_PERMISSION_ENDPOINT: &str = "api/v5/users/subaccount/set-transfer-out";

/// Request to set permission of transfer out
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTransferPermissionRequest {
    /// Name of the sub-account. Single sub-account or multiple sub-account (no more than 20) separated with comma.
    pub sub_acct: String,

    /// Whether the sub-account has the right to transfer out. The default is true.
    /// false: cannot transfer out
    /// true: can transfer out
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_trans_out: Option<bool>,
}

/// Response from setting permission of transfer out
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTransferPermissionResponse {
    /// Name of the sub-account
    pub sub_acct: String,

    /// Whether the sub-account has the right to transfer out
    /// false: cannot transfer out
    /// true: can transfer out
    pub can_trans_out: bool,
}

impl RestClient {
    /// Set permission of transfer out
    ///
    /// Set permission of transfer out for sub-account (only applicable to master account API key).
    /// Sub-account can transfer out to master account by default.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#sub-account-rest-api-set-permission-of-transfer-out)
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The transfer permission request parameters
    ///
    /// # Returns
    /// A result containing the updated transfer permission information
    pub async fn set_transfer_permission(
        &self,
        request: SetTransferPermissionRequest,
    ) -> RestResult<SetTransferPermissionResponse> {
        self.send_post_request(
            SET_TRANSFER_PERMISSION_ENDPOINT,
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
    fn test_set_transfer_permission_request_serialization() {
        let request = SetTransferPermissionRequest {
            sub_acct: "test_sub_001".to_string(),
            can_trans_out: Some(false),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAcct\":\"test_sub_001\""));
        assert!(json.contains("\"canTransOut\":false"));
    }

    #[test]
    fn test_set_transfer_permission_request_multiple_subaccounts() {
        let request = SetTransferPermissionRequest {
            sub_acct: "sub_001,sub_002,sub_003".to_string(),
            can_trans_out: Some(true),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAcct\":\"sub_001,sub_002,sub_003\""));
        assert!(json.contains("\"canTransOut\":true"));
    }

    #[test]
    fn test_set_transfer_permission_request_default() {
        let request = SetTransferPermissionRequest {
            sub_acct: "test_sub_default".to_string(),
            can_trans_out: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAcct\":\"test_sub_default\""));
        assert!(!json.contains("canTransOut"));
    }

    #[test]
    fn test_set_transfer_permission_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "subAcct": "test_sub_001",
                    "canTransOut": false
                }
            ]
        }"#;

        let response: ApiResponse<SetTransferPermissionResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let permission = &response.data[0];
        assert_eq!(permission.sub_acct, "test_sub_001");
        assert!(!permission.can_trans_out);
    }

    #[test]
    fn test_set_transfer_permission_response_multiple_subaccounts() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "subAcct": "sub_001",
                    "canTransOut": true
                },
                {
                    "subAcct": "sub_002",
                    "canTransOut": true
                },
                {
                    "subAcct": "sub_003",
                    "canTransOut": true
                }
            ]
        }"#;

        let response: ApiResponse<SetTransferPermissionResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 3);

        let sub_001_permission = &response.data[0];
        assert_eq!(sub_001_permission.sub_acct, "sub_001");
        assert!(sub_001_permission.can_trans_out);

        let sub_002_permission = &response.data[1];
        assert_eq!(sub_002_permission.sub_acct, "sub_002");
        assert!(sub_002_permission.can_trans_out);

        let sub_003_permission = &response.data[2];
        assert_eq!(sub_003_permission.sub_acct, "sub_003");
        assert!(sub_003_permission.can_trans_out);
    }

    #[test]
    fn test_set_transfer_permission_response_disabled() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "subAcct": "restricted_sub",
                    "canTransOut": false
                }
            ]
        }"#;

        let response: ApiResponse<SetTransferPermissionResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let permission = &response.data[0];
        assert_eq!(permission.sub_acct, "restricted_sub");
        assert!(!permission.can_trans_out);
    }

    #[test]
    fn test_boolean_values() {
        // Test true value
        let request_true = SetTransferPermissionRequest {
            sub_acct: "sub_enable".to_string(),
            can_trans_out: Some(true),
        };

        let json_true = serde_json::to_string(&request_true).unwrap();
        assert!(json_true.contains("\"canTransOut\":true"));

        // Test false value
        let request_false = SetTransferPermissionRequest {
            sub_acct: "sub_disable".to_string(),
            can_trans_out: Some(false),
        };

        let json_false = serde_json::to_string(&request_false).unwrap();
        assert!(json_false.contains("\"canTransOut\":false"));
    }
}
