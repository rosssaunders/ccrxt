use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

const ACCOUNT_CONFIG_ENDPOINT: &str = "api/v5/account/config";

/// Request to get account configuration
#[derive(Debug, Clone, Serialize)]
pub struct GetAccountConfigRequest {
    // This endpoint doesn't require any parameters
}

/// Account configuration details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountConfig {
    /// Account ID
    pub uid: String,

    /// Account level
    /// 1: Simple, 2: Single-currency margin, 3: Multi-currency margin, 4: Portfolio margin
    pub acct_lv: String,

    /// Position mode
    /// long_short_mode: long/short, net_mode: net
    pub pos_mode: String,

    /// Whether the account enables auto margin or not
    pub auto_loan: bool,

    /// The user level for Greeks PA/BS mode
    pub greeks_type: String,

    /// Current account level
    pub level: String,

    /// Temporary experience user level
    pub level_tmp: String,

    /// Whether the account can cross borrow and repay
    pub ct_iso_mode: String,

    /// Margin mode
    pub mgn_iso_mode: String,

    /// Risk offset type
    pub spot_offset_type: String,

    /// Role type. 0: General user, 1: Leading trader, 2: Copy trader
    pub role_type: String,

    /// Trade role
    pub trade_role: String,

    /// Maximum available size between 1 and 500
    pub max_size: Option<String>,

    /// Whether to enable quick margin mode for Multi-currency margin
    pub quick_mgn_type: Option<String>,

    /// IP restriction details
    pub ip: Vec<IpRestriction>,

    /// Permission details
    pub perm: Vec<String>,

    /// Label of the account. Only applicable to broker accounts
    pub label: Option<String>,

    /// Main UID. Only applicable to broker accounts
    pub main_uid: Option<String>,

    /// API key permission
    pub op_auth: String,

    /// Kyc level of the account. 0: no kyc, 1: level 1, 2: level 2, 3: level 3
    pub kyc_lv: Option<String>,

    /// KYC level for the account
    pub kyc_lv_new: Option<String>,

    /// KYC level for the account
    pub api_kyc_lv: Option<String>,

    /// Whether the position risk offset is enabled or not
    pub spot_role_type: Option<String>,

    /// Whether the account enables spot trading
    pub spot_trading: Option<bool>,

    /// Whether the account enables futures trading
    pub future_trading: Option<bool>,

    /// Whether the account enables options trading
    pub option_trading: Option<bool>,

    /// The feature enabled for the account. 0: Normal mode, 1: Buy first mode
    pub kmp: Option<String>,
}

/// IP restriction details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpRestriction {
    /// IP address
    pub ip: String,

    /// TS
    pub ts: String,
}

impl RestClient {
    /// Get account configuration
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-account-configuration
    ///
    /// # Returns
    /// A result containing the account configuration or an error
    pub async fn get_account_config(&self) -> RestResult<OkxApiResponse<AccountConfig>> {
        let request = GetAccountConfigRequest {};
        self.send_request(
            ACCOUNT_CONFIG_ENDPOINT,
            reqwest::Method::GET,
            Some(&request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account_config_request_serialization() {
        let request = GetAccountConfigRequest {};
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_account_config_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "uid": "123456789",
                    "acctLv": "2",
                    "posMode": "long_short_mode",
                    "autoLoan": false,
                    "greeksType": "PA",
                    "level": "Lv1",
                    "levelTmp": "",
                    "ctIsoMode": "automatic",
                    "mgnIsoMode": "automatic",
                    "spotOffsetType": "",
                    "roleType": "0",
                    "tradeRole": "general",
                    "maxSize": "500",
                    "quickMgnType": "manual",
                    "ip": [],
                    "perm": ["read_only", "trade", "withdraw"],
                    "label": "",
                    "mainUid": "",
                    "opAuth": "1",
                    "kycLv": "2",
                    "kycLvNew": "2",
                    "apiKycLv": "2",
                    "spotRoleType": "0",
                    "spotTrading": true,
                    "futureTrading": true,
                    "optionTrading": false,
                    "kmp": "0"
                }
            ]
        }"#;

        let response: OkxApiResponse<AccountConfig> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let config = &response.data[0];
        assert_eq!(config.uid, "123456789");
        assert_eq!(config.acct_lv, "2");
        assert_eq!(config.pos_mode, "long_short_mode");
        assert!(!config.auto_loan);
        assert_eq!(config.level, "Lv1");
        assert_eq!(config.perm.len(), 3);
        assert!(config.perm.contains(&"trade".to_string()));
        assert_eq!(config.spot_trading, Some(true));
        assert_eq!(config.future_trading, Some(true));
        assert_eq!(config.option_trading, Some(false));
    }
}
