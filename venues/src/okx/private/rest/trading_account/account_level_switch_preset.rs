use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for account level switch preset
const ACCOUNT_LEVEL_SWITCH_PRESET_ENDPOINT: &str = "api/v5/account/account-level-switch-preset";

/// Request parameters for account level switch preset
#[derive(Debug, Clone, Serialize)]
pub struct AccountLevelSwitchPresetRequest {
    /// Account level
    /// 1: Simple
    /// 2: Single-currency margin
    /// 3: Multi-currency margin
    /// 4: Portfolio margin
    #[serde(rename = "acctLv")]
    pub acct_lv: String,
}

/// Account level switch preset response
#[derive(Debug, Clone, Deserialize)]
pub struct AccountLevelSwitchPresetResponse {
    /// Account level
    #[serde(rename = "acctLv")]
    pub acct_lv: String,

    /// Auto loan
    #[serde(rename = "autoLoan")]
    pub auto_loan: bool,

    /// Greeks type
    #[serde(rename = "greeksType")]
    pub greeks_type: String,

    /// Level
    pub level: String,

    /// Level temporary
    #[serde(rename = "levelTmp")]
    pub level_tmp: String,

    /// Position mode
    #[serde(rename = "posMode")]
    pub pos_mode: String,
}

impl RestClient {
    /// Account level switch preset
    ///
    /// Only Applicable to master account. A sub-account can not use this endpoint.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-account-level-switch-preset)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The account level switch preset request parameters
    ///
    /// # Returns
    /// A result containing the account level switch preset response
    pub async fn account_level_switch_preset(
        &self,
        request: AccountLevelSwitchPresetRequest,
    ) -> RestResult<AccountLevelSwitchPresetResponse> {
        self.send_post_request(
            ACCOUNT_LEVEL_SWITCH_PRESET_ENDPOINT,
            Some(&request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_account_level_switch_preset_request_serialization() {
        let request = AccountLevelSwitchPresetRequest {
            acct_lv: "3".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"acctLv\":\"3\""));
    }

    #[test]
    fn test_account_level_switch_preset_response_deserialization() {
        let response_json = json!({
            "acctLv": "3",
            "autoLoan": true,
            "greeksType": "PA",
            "level": "Lv1",
            "levelTmp": "Lv1",
            "posMode": "long_short_mode"
        });

        let response: AccountLevelSwitchPresetResponse =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.acct_lv, "3");
        assert!(response.auto_loan);
        assert_eq!(response.greeks_type, "PA");
        assert_eq!(response.pos_mode, "long_short_mode");
    }

    #[test]
    fn test_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "acctLv": "3",
                    "autoLoan": true,
                    "greeksType": "PA",
                    "level": "Lv1",
                    "levelTmp": "Lv1",
                    "posMode": "long_short_mode"
                }
            ]
        });

        let response: ApiResponse<AccountLevelSwitchPresetResponse> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].acct_lv, "3");
    }
}
