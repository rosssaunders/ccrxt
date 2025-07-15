use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::RestResult;

/// Endpoint path for the get-accounts API
const ACCOUNTS_ENDPOINT: &str = "private/get-accounts";

/// Account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Sub account uuid
    pub uuid: String,

    /// Master account uuid
    pub master_account_uuid: String,

    /// (optional) Margin account uuid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_account_uuid: Option<String>,

    /// Sub account label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// true or false
    pub enabled: bool,

    /// true or false
    pub tradable: bool,

    /// Name of sub account
    pub name: String,

    /// Email of sub account
    pub email: String,

    /// Mobile number of sub account
    pub mobile_number: String,

    /// Country Code of sub account
    pub country_code: String,

    /// Address of sub account
    pub address: String,

    /// DEFAULT or DISABLED
    pub margin_access: String,

    /// DEFAULT or DISABLED
    pub derivatives_access: String,

    /// Creation timestamp (milliseconds since the Unix epoch)
    pub create_time: u64,

    /// Last update timestamp (milliseconds since the Unix epoch)
    pub update_time: u64,

    /// true or false
    pub two_fa_enabled: bool,

    /// Kyc Level
    pub kyc_level: String,

    /// true or false
    pub suspended: bool,

    /// true or false
    pub terminated: bool,
}

/// Response for get accounts endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountsResponse {
    /// Master account information
    pub master_account: Account,

    /// List of sub accounts
    pub sub_account_list: Vec<Account>,
}

/// Request parameters for get accounts endpoint
#[derive(Debug, Clone, Serialize)]
pub struct GetAccountsRequest {
    /// Page size (default: 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,

    /// Page number (default: 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl RestClient {
    /// Get master account and sub-accounts with pagination support
    ///
    /// Returns master account and its sub-accounts information.
    ///
    /// [Official API docs](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#private-get-accounts)
    ///
    /// Rate limit: No rate limit
    ///
    /// # Arguments
    /// * `request` - Parameters for the get accounts call
    ///
    /// # Returns
    /// Master account and sub accounts information
    pub async fn get_accounts(
        &self,
        request: GetAccountsRequest,
    ) -> RestResult<GetAccountsResponse> {
        self.send_signed_request(ACCOUNTS_ENDPOINT, request).await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    use serde_json::json;

    use super::*;

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    #[allow(dead_code)]
    struct PlainTextSecret {
        secret: String,
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl PlainTextSecret {
        #[allow(dead_code)]
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_account_structure() {
        let account_json = json!({
            "uuid": "243d3f39-b193-4eb9-1d60-e98f2fc17707",
            "master_account_uuid": "291879ae-b769-4eb3-4d75-3366ebee7dd6",
            "margin_account_uuid": "69c9ab41-5b95-4d75-b769-e45f2fc16507",
            "label": "Sub Account",
            "enabled": true,
            "tradable": true,
            "name": "",
            "email": "user@crypto.com",
            "mobile_number": "",
            "country_code": "",
            "address": "",
            "margin_access": "DEFAULT",
            "derivatives_access": "DISABLED",
            "create_time": 1620962543792_u64,
            "update_time": 1622019525960_u64,
            "two_fa_enabled": true,
            "kyc_level": "ADVANCED",
            "suspended": false,
            "terminated": false
        });

        let account: Account = serde_json::from_value(account_json).unwrap();
        assert_eq!(account.uuid, "243d3f39-b193-4eb9-1d60-e98f2fc17707");
        assert!(account.enabled);
        assert_eq!(account.email, "user@crypto.com");
        assert_eq!(account.margin_access, "DEFAULT");
        assert_eq!(account.derivatives_access, "DISABLED");
        assert!(account.two_fa_enabled);
        assert_eq!(account.kyc_level, "ADVANCED");
    }

    #[test]
    fn test_account_with_optional_fields() {
        let account_json = json!({
            "uuid": "243d3f39-b193-4eb9-1d60-e98f2fc17707",
            "master_account_uuid": "291879ae-b769-4eb3-4d75-3366ebee7dd6",
            "enabled": true,
            "tradable": true,
            "name": "",
            "email": "user@crypto.com",
            "mobile_number": "",
            "country_code": "",
            "address": "",
            "margin_access": "DEFAULT",
            "derivatives_access": "DISABLED",
            "create_time": 1620962543792_u64,
            "update_time": 1622019525960_u64,
            "two_fa_enabled": true,
            "kyc_level": "ADVANCED",
            "suspended": false,
            "terminated": false
        });

        let account: Account = serde_json::from_value(account_json).unwrap();
        assert_eq!(account.uuid, "243d3f39-b193-4eb9-1d60-e98f2fc17707");
        assert_eq!(account.margin_account_uuid, None);
        assert_eq!(account.label, None);
    }

    #[test]
    fn test_get_accounts_response_structure() {
        let response_json = json!({
            "master_account": {
                "uuid": "243d3f39-b193-4eb9-1d60-e98f2fc17707",
                "master_account_uuid": "291879ae-b769-4eb3-4d75-3366ebee7dd6",
                "margin_account_uuid": "69c9ab41-5b95-4d75-b769-e45f2fc16507",
                "enabled": true,
                "tradable": true,
                "name": "",
                "email": "user@crypto.com",
                "mobile_number": "",
                "country_code": "",
                "address": "",
                "margin_access": "DEFAULT",
                "derivatives_access": "DISABLED",
                "create_time": 1620962543792_u64,
                "update_time": 1622019525960_u64,
                "two_fa_enabled": true,
                "kyc_level": "ADVANCED",
                "suspended": false,
                "terminated": false
            },
            "sub_account_list": [
                {
                    "uuid": "sub-account-uuid",
                    "master_account_uuid": "291879ae-b769-4eb3-4d75-3366ebee7dd6",
                    "enabled": true,
                    "tradable": false,
                    "name": "Sub Account 1",
                    "email": "sub@crypto.com",
                    "mobile_number": "",
                    "country_code": "",
                    "address": "",
                    "margin_access": "DISABLED",
                    "derivatives_access": "DISABLED",
                    "create_time": 1620962543792_u64,
                    "update_time": 1622019525960_u64,
                    "two_fa_enabled": false,
                    "kyc_level": "BASIC",
                    "suspended": false,
                    "terminated": false
                }
            ]
        });

        let response: GetAccountsResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(
            response.master_account.uuid,
            "243d3f39-b193-4eb9-1d60-e98f2fc17707"
        );
        assert_eq!(response.sub_account_list.len(), 1);
        assert_eq!(
            response.sub_account_list.first().unwrap().uuid,
            "sub-account-uuid"
        );
        assert!(!response.sub_account_list.first().unwrap().tradable);
    }

    #[test]
    fn test_get_accounts_request_serialization() {
        let request = GetAccountsRequest {
            page_size: Some(30),
            page: Some(2),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("page_size").unwrap(), 30);
        assert_eq!(json_value.get("page").unwrap(), 2);
    }

    #[test]
    fn test_get_accounts_request_optional_fields() {
        let request = GetAccountsRequest {
            page_size: None,
            page: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value, json!({}));
    }

    #[test]
    fn test_get_accounts_request_partial_fields() {
        let request = GetAccountsRequest {
            page_size: Some(50),
            page: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("page_size").unwrap(), 50);
        assert!(!json_value.as_object().unwrap().contains_key("page"));
    }
}
