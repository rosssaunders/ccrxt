use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const ACCOUNT_CONFIG_ENDPOINT: &str = "/fapi/v1/accountConfig";

/// Request parameters for the account configuration endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountConfigRequest {
    /// Request timestamp in milliseconds since epoch.
    /// Must be the current server time.
    pub timestamp: u64,

    /// Optional receive window (milliseconds). If not set, default is used by API.
    /// Valid range: 1-60000. Used to specify the number of milliseconds after timestamp the request is valid for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response for user account configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountConfigResponse {
    /// Fee tier for the account. Integer commission tier.
    pub fee_tier: u32,

    /// Whether the account can trade.
    pub can_trade: bool,

    /// Whether the account can deposit (transfer in asset).
    pub can_deposit: bool,

    /// Whether the account can withdraw (transfer out asset).
    pub can_withdraw: bool,

    /// Fee burn option for spot trading. (Not documented, returned by API.)
    #[serde(default)]
    pub fee_burn: bool,

    /// Multi-assets margin enabled.
    pub multi_assets_margin: bool,

    /// Whether dual side position is enabled. (API field: dualSidePosition)
    #[serde(rename = "dualSidePosition")]
    pub dual_side_position: Option<bool>,

    /// Trade group ID. (API field: tradeGroupId)
    #[serde(rename = "tradeGroupId")]
    pub trade_group_id: Option<i32>,

    /// Last update time (milliseconds since epoch). Reserved property, may be zero.
    pub update_time: u64,
}

impl UsdmClient {
    /// Get Account Configuration (GET /fapi/v1/accountConfig)
    ///
    /// Retrieves the current account configuration including fee tier, trading permissions, margin settings, and other account flags.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Account-Config
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `params` - The request parameters
    ///
    /// # Returns
    /// Account configuration response
    pub async fn get_account_config(
        &self,
        params: GetAccountConfigRequest,
    ) -> RestResult<AccountConfigResponse> {
        self.send_signed_request(ACCOUNT_CONFIG_ENDPOINT, Method::GET, params, 5, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account_config_request_serialization() {
        let request = GetAccountConfigRequest {
            timestamp: 1625097600000,
            recv_window: Some(5000),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_account_config_response_deserialization_full() {
        let json = r#"{
            "feeTier": 0,
            "canTrade": true,
            "canDeposit": true,
            "canWithdraw": true,
            "feeBurn": false,
            "multiAssetsMargin": false,
            "dualSidePosition": true,
            "tradeGroupId": -1,
            "updateTime": 1625097600000
        }"#;
        let response: AccountConfigResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.fee_tier, 0);
        assert!(response.can_trade);
        assert!(response.can_deposit);
        assert!(response.can_withdraw);
        assert!(!response.fee_burn);
        assert!(!response.multi_assets_margin);
        assert_eq!(response.dual_side_position, Some(true));
        assert_eq!(response.trade_group_id, Some(-1));
        assert_eq!(response.update_time, 1625097600000);
    }

    #[test]
    fn test_account_config_response_deserialization_minimal() {
        let json = r#"{
            "feeTier": 1,
            "canTrade": false,
            "canDeposit": false,
            "canWithdraw": false,
            "feeBurn": true,
            "multiAssetsMargin": true,
            "updateTime": 0
        }"#;
        let response: AccountConfigResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.fee_tier, 1);
        assert!(!response.can_trade);
        assert!(!response.can_deposit);
        assert!(!response.can_withdraw);
        assert!(response.fee_burn);
        assert!(response.multi_assets_margin);
        assert_eq!(response.dual_side_position, None);
        assert_eq!(response.trade_group_id, None);
        assert_eq!(response.update_time, 0);
    }
}
