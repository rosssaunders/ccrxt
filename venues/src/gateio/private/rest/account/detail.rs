use serde::Deserialize;

use super::{RestClient, RestResult};

/// Endpoint path for retrieving user account information.
const ACCOUNT_DETAIL_ENDPOINT: &str = "/account/detail";

/// API Key details for the account.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyInfo {
    /// Mode: 1 - Classic mode, 2 - Legacy unified mode
    pub mode: i32,
}

/// Response for retrieving user account information.
///
/// Contains account details such as whitelists, user ID, VIP tier, API key info, and copy trading role.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountDetail {
    /// IP Whitelist. May be empty if not set.
    #[serde(default)]
    pub ip_whitelist: Vec<String>,

    /// Trading pair whitelist. May be empty if not set.
    #[serde(default)]
    pub currency_pairs: Vec<String>,

    /// User ID (unique identifier for the account).
    pub user_id: i64,

    /// User VIP level (tier).
    pub tier: i64,

    /// API Key details for the account.
    pub key: ApiKeyInfo,

    /// User role:
    /// - 0: Normal user
    /// - 1: Copy trading leader
    /// - 2: Follower
    /// - 3: Both leader and follower
    pub copy_trading_role: i32,
}

impl RestClient {
    /// Retrieve user account information
    ///
    /// Returns account details including whitelists, user ID, VIP tier, API key info, and copy trading role.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#retrieve-user-account-information)
    ///
    /// Rate limit: See venue documentation for details.
    ///
    /// # Returns
    /// * [`AccountDetail`] - Account details for the authenticated user.
    pub async fn get_account_detail(&self) -> RestResult<AccountDetail> {
        self.send_get_request(ACCOUNT_DETAIL_ENDPOINT, Option::<&()>::None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_detail_fields_full() {
        let json = r#"{
            "user_id": 1667201533,
            "ip_whitelist": ["127.0.0.1"],
            "currency_pairs": ["USDT_BTC"],
            "key": {"mode": 1},
            "tier": 2,
            "copy_trading_role": 1
        }"#;
        let detail: AccountDetail = serde_json::from_str(json).unwrap();
        assert_eq!(detail.user_id, 1667201533);
        assert_eq!(detail.key.mode, 1);
        assert_eq!(detail.tier, 2);
        assert_eq!(detail.copy_trading_role, 1);
        assert_eq!(detail.ip_whitelist, vec!["127.0.0.1"]);
        assert_eq!(detail.currency_pairs, vec!["USDT_BTC"]);
    }

    #[test]
    fn test_account_detail_fields_minimal() {
        let json = r#"{
            "user_id": 123456,
            "key": {"mode": 2},
            "tier": 1,
            "copy_trading_role": 0
        }"#;
        let detail: AccountDetail = serde_json::from_str(json).unwrap();
        assert_eq!(detail.user_id, 123456);
        assert_eq!(detail.key.mode, 2);
        assert_eq!(detail.tier, 1);
        assert_eq!(detail.copy_trading_role, 0);
        assert!(detail.ip_whitelist.is_empty());
        assert!(detail.currency_pairs.is_empty());
    }
}
