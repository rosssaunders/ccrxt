use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::RestResult;

const ACCOUNT_INFO_ENDPOINT: &str = "/api/v2/spot/account/info";

/// Account status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountStatus {
    /// Normal account status
    Normal,
    /// Frozen account status
    Frozen,
}

/// Account type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    /// Main account
    Main,
    /// Sub account
    Sub,
}

/// VIP level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VipLevel {
    #[serde(rename = "VIP0")]
    Vip0,
    #[serde(rename = "VIP1")]
    Vip1,
    #[serde(rename = "VIP2")]
    Vip2,
    #[serde(rename = "VIP3")]
    Vip3,
    #[serde(rename = "VIP4")]
    Vip4,
    #[serde(rename = "VIP5")]
    Vip5,
    #[serde(rename = "VIP6")]
    Vip6,
    #[serde(rename = "VIP7")]
    Vip7,
    #[serde(rename = "VIP8")]
    Vip8,
    #[serde(rename = "VIP9")]
    Vip9,
}

/// Request parameters for getting account information.
/// This endpoint supports optional request timing parameters.
#[derive(Debug, Clone, Serialize, Default)]
pub struct AccountInfoRequest {
    /// Request timestamp (Unix milliseconds).
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,

    /// Valid time window (Unix milliseconds).
    /// If set, request is valid only when server time is within receiveWindow.
    #[serde(rename = "receiveWindow", skip_serializing_if = "Option::is_none")]
    pub receive_window: Option<i64>,
}

/// Account VIP level information containing level details and volume requirements.
#[derive(Debug, Clone, Deserialize)]
pub struct VipInfo {
    /// Current VIP level of the account.
    #[serde(rename = "level")]
    pub level: VipLevel,

    /// Next available VIP level if applicable.
    #[serde(rename = "nextLevel")]
    pub next_level: Option<VipLevel>,

    /// Trading volume required for next level (30-day volume in USDT).
    #[serde(rename = "nextLevelVolume")]
    pub next_level_volume: Option<String>,

    /// Current 30-day trading volume in USDT.
    #[serde(rename = "currentVolume")]
    pub current_volume: String,
}

/// Account information containing all account details and permissions.
#[derive(Debug, Clone, Deserialize)]
pub struct AccountInfo {
    /// Unique user identifier.
    #[serde(rename = "userId")]
    pub user_id: String,

    /// Spot trading account identifier.
    #[serde(rename = "spotAcctId")]
    pub spot_account_id: String,

    /// Current account status (normal/frozen).
    pub status: AccountStatus,

    /// Whether account is permitted to trade.
    #[serde(rename = "canTrade")]
    pub can_trade: bool,

    /// Whether account is permitted to withdraw funds.
    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,

    /// Whether account is permitted to deposit funds.
    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,

    /// Last account update timestamp (Unix milliseconds).
    #[serde(rename = "updateTime")]
    pub update_time: i64,

    /// VIP level information and volume details.
    #[serde(rename = "vipInfo")]
    pub vip_info: VipInfo,

    /// Whether account has completed KYC verification.
    #[serde(rename = "kycFlag")]
    pub kyc_flag: bool,

    /// KYC verification level if applicable.
    #[serde(rename = "kycLevel")]
    pub kyc_level: Option<String>,

    /// Parent user ID for sub-accounts.
    #[serde(rename = "parentUserId")]
    pub parent_user_id: Option<String>,

    /// Account type (main/sub).
    #[serde(rename = "accountType")]
    pub account_type: AccountType,
}

/// Response from the account info endpoint containing account details.
#[derive(Debug, Clone, Deserialize)]
pub struct AccountInfoResponse {
    /// Complete account information.
    #[serde(flatten)]
    pub account_info: AccountInfo,
}

impl RestClient {
    /// Get spot account information
    ///
    /// Retrieves general information about the account including permissions,
    /// VIP level, and account status.
    ///
    /// # Arguments
    /// * `request` - The account info request parameters
    ///
    /// # Rate Limit
    /// 10 requests per second per UID
    ///
    /// # Returns
    /// A result containing the account information response or an error
    pub async fn account_info(
        &self,
        request: AccountInfoRequest,
    ) -> RestResult<AccountInfoResponse> {
        self.send_get_signed_request(
            ACCOUNT_INFO_ENDPOINT,
            request,
            10,    // 10 requests per second rate limit
            false, // This is not an order placement endpoint
            None,  // No order-specific rate limit
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_info_request_new() {
        let request = AccountInfoRequest {
            request_time: None,
            receive_window: None,
        };

        assert!(request.request_time.is_none());
        assert!(request.receive_window.is_none());
    }

    #[test]
    fn test_account_info_request_builder() {
        let request = AccountInfoRequest {
            request_time: Some(1640995200000),
            receive_window: Some(5000),
        };

        assert_eq!(request.request_time, Some(1640995200000));
        assert_eq!(request.receive_window, Some(5000));
    }

    #[test]
    fn test_account_info_request_serialization() {
        let request = AccountInfoRequest {
            request_time: Some(1640995200000),
            receive_window: None,
        };

        let query = serde_urlencoded::to_string(&request).unwrap();

        assert!(query.contains("requestTime=1640995200000"));
        assert!(!query.contains("receiveWindow"));
    }

    #[test]
    fn test_account_info_request_serialization_empty() {
        let request = AccountInfoRequest {
            request_time: None,
            receive_window: None,
        };
        let query = serde_urlencoded::to_string(&request).unwrap();

        assert!(query.is_empty());
    }

    #[test]
    fn test_vip_info_deserialization() {
        let json = r#"{
            "level": "VIP1",
            "nextLevel": "VIP2",
            "nextLevelVolume": "1000000",
            "currentVolume": "500000"
        }"#;

        let vip_info: VipInfo = serde_json::from_str(json).unwrap();

        assert_eq!(vip_info.level, VipLevel::Vip1);
        assert_eq!(vip_info.next_level, Some(VipLevel::Vip2));
        assert_eq!(vip_info.next_level_volume, Some("1000000".to_string()));
        assert_eq!(vip_info.current_volume, "500000");
    }

    #[test]
    fn test_account_info_deserialization() {
        let json = r#"{
            "userId": "123456789",
            "spotAcctId": "spot_987654321",
            "status": "normal",
            "canTrade": true,
            "canWithdraw": true,
            "canDeposit": true,
            "updateTime": 1640995200000,
            "vipInfo": {
                "level": "VIP1",
                "nextLevel": "VIP2",
                "nextLevelVolume": "1000000",
                "currentVolume": "500000"
            },
            "kycFlag": true,
            "kycLevel": "2",
            "parentUserId": null,
            "accountType": "main"
        }"#;

        let account_info: AccountInfo = serde_json::from_str(json).unwrap();

        assert_eq!(account_info.user_id, "123456789");
        assert_eq!(account_info.spot_account_id, "spot_987654321");
        assert_eq!(account_info.status, AccountStatus::Normal);
        assert!(account_info.can_trade);
        assert!(account_info.can_withdraw);
        assert!(account_info.can_deposit);
        assert_eq!(account_info.update_time, 1640995200000);
        assert_eq!(account_info.vip_info.level, VipLevel::Vip1);
        assert!(account_info.kyc_flag);
        assert_eq!(account_info.kyc_level, Some("2".to_string()));
        assert!(account_info.parent_user_id.is_none());
        assert_eq!(account_info.account_type, AccountType::Main);
    }

    #[test]
    fn test_account_info_deserialization_sub_account() {
        let json = r#"{
            "userId": "123456789",
            "spotAcctId": "spot_987654321",
            "status": "normal",
            "canTrade": false,
            "canWithdraw": false,
            "canDeposit": true,
            "updateTime": 1640995200000,
            "vipInfo": {
                "level": "VIP0",
                "nextLevel": "VIP1",
                "nextLevelVolume": "100000",
                "currentVolume": "0"
            },
            "kycFlag": false,
            "kycLevel": null,
            "parentUserId": "987654321",
            "accountType": "sub"
        }"#;

        let account_info: AccountInfo = serde_json::from_str(json).unwrap();

        assert_eq!(account_info.user_id, "123456789");
        assert_eq!(account_info.status, AccountStatus::Normal);
        assert!(!account_info.can_trade);
        assert!(!account_info.can_withdraw);
        assert!(account_info.can_deposit);
        assert!(!account_info.kyc_flag);
        assert!(account_info.kyc_level.is_none());
        assert_eq!(account_info.parent_user_id, Some("987654321".to_string()));
        assert_eq!(account_info.account_type, AccountType::Sub);
    }

    #[test]
    fn test_account_info_response_deserialization() {
        let json = r#"{
            "userId": "123456789",
            "spotAcctId": "spot_987654321",
            "status": "normal",
            "canTrade": true,
            "canWithdraw": true,
            "canDeposit": true,
            "updateTime": 1640995200000,
            "vipInfo": {
                "level": "VIP1",
                "nextLevel": "VIP2",
                "nextLevelVolume": "1000000",
                "currentVolume": "500000"
            },
            "kycFlag": true,
            "kycLevel": "2",
            "parentUserId": null,
            "accountType": "main"
        }"#;

        let response: AccountInfoResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.account_info.user_id, "123456789");
        assert_eq!(response.account_info.account_type, AccountType::Main);
    }

    #[test]
    fn test_account_status_serialization() {
        assert_eq!(
            serde_json::to_string(&AccountStatus::Normal).unwrap(),
            "\"normal\""
        );
        assert_eq!(
            serde_json::to_string(&AccountStatus::Frozen).unwrap(),
            "\"frozen\""
        );
    }

    #[test]
    fn test_account_status_deserialization() {
        assert_eq!(
            serde_json::from_str::<AccountStatus>("\"normal\"").unwrap(),
            AccountStatus::Normal
        );
        assert_eq!(
            serde_json::from_str::<AccountStatus>("\"frozen\"").unwrap(),
            AccountStatus::Frozen
        );
    }

    #[test]
    fn test_account_type_serialization() {
        assert_eq!(
            serde_json::to_string(&AccountType::Main).unwrap(),
            "\"main\""
        );
        assert_eq!(serde_json::to_string(&AccountType::Sub).unwrap(), "\"sub\"");
    }

    #[test]
    fn test_account_type_deserialization() {
        assert_eq!(
            serde_json::from_str::<AccountType>("\"main\"").unwrap(),
            AccountType::Main
        );
        assert_eq!(
            serde_json::from_str::<AccountType>("\"sub\"").unwrap(),
            AccountType::Sub
        );
    }

    #[test]
    fn test_vip_level_serialization() {
        assert_eq!(serde_json::to_string(&VipLevel::Vip0).unwrap(), "\"VIP0\"");
        assert_eq!(serde_json::to_string(&VipLevel::Vip1).unwrap(), "\"VIP1\"");
        assert_eq!(serde_json::to_string(&VipLevel::Vip9).unwrap(), "\"VIP9\"");
    }

    #[test]
    fn test_vip_level_deserialization() {
        assert_eq!(
            serde_json::from_str::<VipLevel>("\"VIP0\"").unwrap(),
            VipLevel::Vip0
        );
        assert_eq!(
            serde_json::from_str::<VipLevel>("\"VIP1\"").unwrap(),
            VipLevel::Vip1
        );
        assert_eq!(
            serde_json::from_str::<VipLevel>("\"VIP9\"").unwrap(),
            VipLevel::Vip9
        );
    }
}
