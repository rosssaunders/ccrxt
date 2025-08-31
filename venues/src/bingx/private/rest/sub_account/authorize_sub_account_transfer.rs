use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const AUTHORIZE_SUB_ACCOUNT_TRANSFER_ENDPOINT: &str =
    "/openApi/account/v1/innerTransfer/authorizeSubAccount";

/// Request to authorize sub-account internal transfers
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeSubAccountTransferRequest {
    /// User uid list, comma separated
    pub sub_uids: String,

    /// Is it allowed? True allows false prohibits
    pub transferable: bool,

    /// Timestamp of initiating the request, Unit: milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Request valid time window value, Unit: milliseconds
    pub timestamp: i64,
}

/// Response for authorizing sub-account internal transfers
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeSubAccountTransferResponse {
    /// User uid list, comma separated
    pub sub_uids: String,

    /// Is it allowed? True allows false prohibits
    pub transferable: bool,

    /// Timestamp of initiating the request, Unit: milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Request valid time window value, Unit: milliseconds
    pub timestamp: i64,
}

impl RestClient {
    /// Authorize sub-account internal transfers
    ///
    /// Used for the main account to set the asset transfer permission of
    /// sub-account in batches, so that the sub-account with this permission can
    /// transfer assets to other accounts under the name of the main account.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/common/sub-account#Authorize%20sub-account%20internal%20transfers)
    ///
    /// Rate limit: UID 10/s & IP rate limit group 2
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
            EndpointType::AccountApiGroup2,
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
            sub_uids: "12345,67890".to_string(),
            transferable: true,
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUids\":\"12345,67890\""));
        assert!(json.contains("\"transferable\":true"));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_revoke_transfer_authorization() {
        let request = AuthorizeSubAccountTransferRequest {
            sub_uids: "12345".to_string(),
            transferable: false,
            recv_window: None,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subUids\":\"12345\""));
        assert!(json.contains("\"transferable\":false"));
        assert!(json.contains("\"timestamp\":1640995200000"));
        // recv_window should be omitted when None
        assert!(!json.contains("recvWindow"));
    }

    #[test]
    fn test_authorize_sub_account_transfer_response_deserialization() {
        let json = r#"
        {
            "subUids": "12345,67890",
            "transferable": true,
            "recvWindow": 5000,
            "timestamp": 1640995200000
        }
        "#;

        let response: AuthorizeSubAccountTransferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.sub_uids, "12345,67890");
        assert!(response.transferable);
        assert_eq!(response.recv_window, Some(5000));
        assert_eq!(response.timestamp, 1640995200000);
    }

    #[test]
    fn test_revoke_authorization_response() {
        let json = r#"
        {
            "subUids": "12345",
            "transferable": false,
            "timestamp": 1640995200000
        }
        "#;

        let response: AuthorizeSubAccountTransferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.sub_uids, "12345");
        assert!(!response.transferable);
        assert_eq!(response.timestamp, 1640995200000);
        assert_eq!(response.recv_window, None);
    }
}
