use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Endpoint URL for getting custody trading sub-account list
const GET_CUSTODY_SUBACCOUNT_LIST_ENDPOINT: &str = "api/v5/users/entrust-subaccount-list";

/// Request to get custody trading sub-account list
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCustodySubaccountListRequest {
    /// Sub-account name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_acct: Option<String>,
}

/// Custody trading sub-account information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustodySubaccountInfo {
    /// Sub-account name
    pub sub_acct: String,
}

impl RestClient {
    /// Get custody trading sub-account list
    ///
    /// The trading team uses this interface to view the list of sub-accounts currently under escrow
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#sub-account-rest-api-get-custody-trading-sub-account-list)
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The custody sub-account list request parameters
    ///
    /// # Returns
    /// A result containing the list of custody trading sub-accounts
    pub async fn get_custody_subaccount_list(
        &self,
        request: GetCustodySubaccountListRequest,
    ) -> RestResult<CustodySubaccountInfo> {
        self.send_get_request(
            GET_CUSTODY_SUBACCOUNT_LIST_ENDPOINT,
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
    fn test_get_custody_subaccount_list_request_serialization() {
        let request = GetCustodySubaccountListRequest {
            sub_acct: Some("custody_sub_001".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("subAcct=custody_sub_001"));
    }

    #[test]
    fn test_get_custody_subaccount_list_request_no_filter() {
        let request = GetCustodySubaccountListRequest { sub_acct: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_custody_subaccount_info_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "subAcct": "custody_sub_001"
                }
            ]
        }"#;

        let response: ApiResponse<CustodySubaccountInfo> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let custody_subacct = &response.data[0];
        assert_eq!(custody_subacct.sub_acct, "custody_sub_001");
    }

    #[test]
    fn test_custody_subaccount_info_deserialization_multiple() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "subAcct": "custody_sub_001"
                },
                {
                    "subAcct": "custody_sub_002"
                },
                {
                    "subAcct": "custody_sub_003"
                }
            ]
        }"#;

        let response: ApiResponse<CustodySubaccountInfo> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 3);

        let custody_sub_001 = &response.data[0];
        assert_eq!(custody_sub_001.sub_acct, "custody_sub_001");

        let custody_sub_002 = &response.data[1];
        assert_eq!(custody_sub_002.sub_acct, "custody_sub_002");

        let custody_sub_003 = &response.data[2];
        assert_eq!(custody_sub_003.sub_acct, "custody_sub_003");
    }

    #[test]
    fn test_custody_subaccount_info_deserialization_empty() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": []
        }"#;

        let response: ApiResponse<CustodySubaccountInfo> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 0);
    }
}
