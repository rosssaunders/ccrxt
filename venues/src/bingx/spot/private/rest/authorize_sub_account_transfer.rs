use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const AUTHORIZE_SUB_ACCOUNT_TRANSFER_ENDPOINT: &str =
    "/openApi/wallets/v1/capital/sub-account/transfer-auth";

/// Request to authorize sub-account transfers
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeSubAccountTransferRequest {
    /// Sub-account UID
    pub sub_uid: String,
    /// Whether to enable transfer authorization
    pub can_transfer: bool,
    /// Timestamp of initiating the request, Unit: milliseconds
    pub timestamp: i64,
}

/// Response for authorizing sub-account transfers
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeSubAccountTransferResponse {
    /// Success indicator
    pub success: bool,
    /// Sub-account transfer authorization data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SubAccountTransferAuth>,
}

/// Sub-account transfer authorization data
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountTransferAuth {
    /// Sub-account UID
    pub sub_uid: String,
    /// Whether transfers are authorized
    pub can_transfer: bool,
    /// Authorization timestamp
    pub update_time: i64,
}

impl RestClient {
    /// Authorize sub-account transfers
    ///
    /// Authorizes or revokes transfer permissions for a sub-account.
    /// Rate limit: 5/s by UID
    ///
    /// # Arguments
    /// * `request` - The authorize sub-account transfer request
    ///
    /// # Returns
    /// A result containing the authorization response or an error
    pub async fn authorize_sub_account_transfer(
        &self,
        request: &AuthorizeSubAccountTransferRequest,
    ) -> RestResult<AuthorizeSubAccountTransferResponse> {
        self.send_post_signed_request(
            AUTHORIZE_SUB_ACCOUNT_TRANSFER_ENDPOINT,
            request,
            EndpointType::Account,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorize_sub_account_transfer_request_serialization() {
        let request = AuthorizeSubAccountTransferRequest {
            sub_uid: "12345".to_string(),
            can_transfer: true,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUid\":\"12345\""));
        assert!(json.contains("\"canTransfer\":true"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_revoke_transfer_authorization() {
        let request = AuthorizeSubAccountTransferRequest {
            sub_uid: "12345".to_string(),
            can_transfer: false,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUid\":\"12345\""));
        assert!(json.contains("\"canTransfer\":false"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_authorize_sub_account_transfer_response_deserialization() {
        let json = r#"
        {
            "success": true,
            "data": {
                "subUid": "12345",
                "canTransfer": true,
                "updateTime": 1640995200000
            }
        }
        "#;

        let response: AuthorizeSubAccountTransferResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);

        let data = response.data.unwrap();
        assert_eq!(data.sub_uid, "12345");
        assert!(data.can_transfer);
        assert_eq!(data.update_time, 1640995200000);
    }

    #[test]
    fn test_revoke_authorization_response() {
        let json = r#"
        {
            "success": true,
            "data": {
                "subUid": "12345",
                "canTransfer": false,
                "updateTime": 1640995200000
            }
        }
        "#;

        let response: AuthorizeSubAccountTransferResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);

        let data = response.data.unwrap();
        assert!(!data.can_transfer);
    }
}
