use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Endpoint URL for creating sub-account
const CREATE_SUBACCOUNT_ENDPOINT: &str = "api/v5/users/subaccount/create-subaccount";

/// Request to create sub-account
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubaccountRequest {
    /// Sub-account name
    pub sub_acct: String,

    /// Sub-account type
    /// 1: Standard sub-account
    /// 5: Custody trading sub-account - Copper  
    /// 12: Custody trading sub-account - Komainu
    #[serde(rename = "type")]
    pub subaccount_type: String,

    /// Sub-account notes. 6-32 letters (case sensitive), numbers or special characters like *.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Sub-account login password, required for KYB users only
    /// Password must contain:
    /// - 8-32 characters long
    /// - 1 lowercase character (a-z)
    /// - 1 uppercase character (A-Z)  
    /// - 1 number
    /// - 1 special character e.g. ! @ # $ %
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pwd: Option<String>,
}

/// Response from creating sub-account
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubaccountResponse {
    /// Sub-account name
    pub sub_acct: String,

    /// Sub-account notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Sub-account ID
    pub uid: String,

    /// Creation time
    pub ts: String,
}

impl RestClient {
    /// Create sub-account
    ///
    /// Applies to master accounts only and master accounts API Key must be linked to IP addresses.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#sub-account-rest-api-create-sub-account)
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The create sub-account request parameters
    ///
    /// # Returns
    /// A result containing the created sub-account information including UID and timestamp
    pub async fn create_subaccount(
        &self,
        request: CreateSubaccountRequest,
    ) -> RestResult<CreateSubaccountResponse> {
        self.send_post_request(
            CREATE_SUBACCOUNT_ENDPOINT,
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
    fn test_create_subaccount_request_serialization() {
        let request = CreateSubaccountRequest {
            sub_acct: "test_sub_001".to_string(),
            subaccount_type: "1".to_string(),
            label: Some("My Test Sub Account".to_string()),
            pwd: Some("Password123!".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAcct\":\"test_sub_001\""));
        assert!(json.contains("\"type\":\"1\""));
        assert!(json.contains("\"label\":\"My Test Sub Account\""));
        assert!(json.contains("\"pwd\":\"Password123!\""));
    }

    #[test]
    fn test_create_subaccount_request_minimal() {
        let request = CreateSubaccountRequest {
            sub_acct: "test_sub_002".to_string(),
            subaccount_type: "1".to_string(),
            label: None,
            pwd: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAcct\":\"test_sub_002\""));
        assert!(json.contains("\"type\":\"1\""));
        assert!(!json.contains("label"));
        assert!(!json.contains("pwd"));
    }

    #[test]
    fn test_create_subaccount_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "subAcct": "test_sub_001",
                    "label": "My Test Sub Account",
                    "uid": "446556018520336384",
                    "ts": "1597026383085"
                }
            ]
        }"#;

        let response: ApiResponse<CreateSubaccountResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let subacct = &response.data[0];
        assert_eq!(subacct.sub_acct, "test_sub_001");
        assert_eq!(subacct.label, Some("My Test Sub Account".to_string()));
        assert_eq!(subacct.uid, "446556018520336384");
        assert_eq!(subacct.ts, "1597026383085");
    }

    #[test]
    fn test_create_subaccount_response_deserialization_minimal() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "subAcct": "test_sub_002",
                    "uid": "446556018520336385",
                    "ts": "1597026383086"
                }
            ]
        }"#;

        let response: ApiResponse<CreateSubaccountResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let subacct = &response.data[0];
        assert_eq!(subacct.sub_acct, "test_sub_002");
        assert!(subacct.label.is_none());
        assert_eq!(subacct.uid, "446556018520336385");
        assert_eq!(subacct.ts, "1597026383086");
    }
}
