use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

/// Request to get account switch precheck
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountSwitchPrecheckRequest {
    /// Account level: 1 simple, 2 single_currency_margin, 3 multi_currency_margin, 4 portfolio_margin
    pub acct_lv: String,
}

/// Account switch precheck information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSwitchPrecheck {
    /// Can switch or not
    pub can_switch: bool,

    /// Switch reason
    pub switch_reason: String,
}

impl RestClient {
    /// Get account switch precheck
    ///
    /// # Arguments
    /// * `request` - The get account switch precheck request
    ///
    /// # Returns
    /// A result containing the account switch precheck or an error
    pub async fn get_account_switch_precheck(
        &self,
        request: &GetAccountSwitchPrecheckRequest,
    ) -> RestResult<OkxApiResponse<AccountSwitchPrecheck>> {
        self.send_request(
            "api/v5/account/set-account-switch-precheck",
            reqwest::Method::GET,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account_switch_precheck_request_serialization() {
        let request = GetAccountSwitchPrecheckRequest {
            acct_lv: "2".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("acctLv=2"));
    }

    #[test]
    fn test_account_switch_precheck_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "canSwitch": true,
                    "switchReason": "success"
                }
            ]
        }"#;

        let response: OkxApiResponse<AccountSwitchPrecheck> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let precheck = &response.data[0];
        assert!(precheck.can_switch);
        assert_eq!(precheck.switch_reason, "success");
    }
}
