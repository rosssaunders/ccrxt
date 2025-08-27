use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Endpoint URL for getting sub-account list
const GET_SUBACCOUNT_LIST_ENDPOINT: &str = "api/v5/users/subaccount/list";

/// Request to get sub-account list
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubaccountListRequest {
    /// Sub-account status
    /// true: Normal, false: Frozen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<String>,

    /// Sub-account name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_acct: Option<String>,

    /// Query the data earlier than the requested subaccount creation timestamp
    /// Unix timestamp in millisecond format, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Query the data newer than the requested subaccount creation timestamp
    /// Unix timestamp in millisecond format, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Number of results per request. The maximum is 100. The default is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Sub-account information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubaccountInfo {
    /// Sub-account type
    /// 1: Standard sub-account
    /// 2: Managed trading sub-account
    /// 5: Custody trading sub-account - Copper
    /// 9: Managed trading sub-account - Copper
    /// 12: Custody trading sub-account - Komainu
    #[serde(rename = "type")]
    pub subaccount_type: String,

    /// Sub-account status
    /// true: Normal, false: Frozen (global)
    pub enable: bool,

    /// Sub-account name
    pub sub_acct: String,

    /// Sub-account uid
    pub uid: String,

    /// Sub-account note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Mobile number that linked with the sub-account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,

    /// If the sub-account switches on the Google Authenticator for login authentication
    /// true: On, false: Off
    pub g_auth: bool,

    /// Frozen functions
    /// trading, convert, transfer, withdrawal, deposit, flexible_loan
    #[serde(default)]
    pub frozen_func: Vec<String>,

    /// Whether the sub-account has the right to transfer out
    /// true: can transfer out, false: cannot transfer out
    pub can_trans_out: bool,

    /// Sub-account creation time, Unix timestamp in millisecond format
    pub ts: String,

    /// Sub-account level
    /// 1: First level sub-account
    /// 2: Second level sub-account
    pub sub_acct_lv: String,

    /// The first level sub-account
    /// For subAcctLv: 1, firstLvSubAcct is equal to subAcct
    /// For subAcctLv: 2, subAcct belongs to firstLvSubAcct
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_lv_sub_acct: Option<String>,

    /// Whether it is dma broker sub-account
    /// true: Dma broker sub-account, false: It is not dma broker sub-account
    pub if_dma: bool,
}

impl RestClient {
    /// Get sub-account list
    ///
    /// Applies to master accounts only
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#sub-account-rest-api-get-sub-account-list)
    ///
    /// Rate limit: 2 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The sub-account list request parameters
    ///
    /// # Returns
    /// A result containing the list of sub-accounts with their details and status
    pub async fn get_subaccount_list(
        &self,
        request: GetSubaccountListRequest,
    ) -> RestResult<SubaccountInfo> {
        self.send_get_request(
            GET_SUBACCOUNT_LIST_ENDPOINT,
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
    fn test_get_subaccount_list_request_serialization() {
        let request = GetSubaccountListRequest {
            enable: Some("true".to_string()),
            sub_acct: Some("test_sub".to_string()),
            after: Some("1597026383085".to_string()),
            before: Some("1597026383086".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("enable=true"));
        assert!(serialized.contains("subAcct=test_sub"));
        assert!(serialized.contains("after=1597026383085"));
        assert!(serialized.contains("before=1597026383086"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_get_subaccount_list_request_empty() {
        let request = GetSubaccountListRequest {
            enable: None,
            sub_acct: None,
            after: None,
            before: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_subaccount_info_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "type": "1",
                    "enable": true,
                    "subAcct": "test_sub_001",
                    "uid": "446556018520336384",
                    "label": "My Sub Account",
                    "mobile": "",
                    "gAuth": false,
                    "frozenFunc": [],
                    "canTransOut": true,
                    "ts": "1597026383085",
                    "subAcctLv": "1",
                    "firstLvSubAcct": "test_sub_001",
                    "ifDma": false
                }
            ]
        }"#;

        let response: ApiResponse<SubaccountInfo> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let subacct = &response.data[0];
        assert_eq!(subacct.subaccount_type, "1");
        assert!(subacct.enable);
        assert_eq!(subacct.sub_acct, "test_sub_001");
        assert_eq!(subacct.uid, "446556018520336384");
        assert_eq!(subacct.label, Some("My Sub Account".to_string()));
        assert!(!subacct.g_auth);
        assert!(subacct.frozen_func.is_empty());
        assert!(subacct.can_trans_out);
        assert_eq!(subacct.ts, "1597026383085");
        assert_eq!(subacct.sub_acct_lv, "1");
        assert_eq!(subacct.first_lv_sub_acct, Some("test_sub_001".to_string()));
        assert!(!subacct.if_dma);
    }

    #[test]
    fn test_subaccount_info_deserialization_minimal() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "type": "1",
                    "enable": false,
                    "subAcct": "test_sub_002",
                    "uid": "446556018520336385",
                    "gAuth": true,
                    "frozenFunc": ["trading", "transfer"],
                    "canTransOut": false,
                    "ts": "1597026383086",
                    "subAcctLv": "2",
                    "ifDma": true
                }
            ]
        }"#;

        let response: ApiResponse<SubaccountInfo> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let subacct = &response.data[0];
        assert_eq!(subacct.subaccount_type, "1");
        assert!(!subacct.enable);
        assert_eq!(subacct.sub_acct, "test_sub_002");
        assert_eq!(subacct.uid, "446556018520336385");
        assert!(subacct.label.is_none());
        assert!(subacct.g_auth);
        assert_eq!(subacct.frozen_func, vec!["trading", "transfer"]);
        assert!(!subacct.can_trans_out);
        assert_eq!(subacct.ts, "1597026383086");
        assert_eq!(subacct.sub_acct_lv, "2");
        assert!(subacct.first_lv_sub_acct.is_none());
        assert!(subacct.if_dma);
    }
}
