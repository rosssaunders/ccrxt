use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::RestResult;

/// Request parameters for getting account information
/// Request parameters for getting account information
#[derive(Debug, Clone, Serialize, Default)]
pub struct AccountInfoRequest {
    /// Request timestamp (Unix milliseconds)
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,

    /// Valid time window (Unix milliseconds)
    /// If set, request is valid only when server time is within receiveWindow
    #[serde(rename = "receiveWindow", skip_serializing_if = "Option::is_none")]
    pub receive_window: Option<i64>,
}

/// Account VIP level information
#[derive(Debug, Clone, Deserialize)]
pub struct VipInfo {
    /// VIP level
    #[serde(rename = "level")]
    pub level: String,

    /// Next VIP level
    #[serde(rename = "nextLevel")]
    pub next_level: Option<String>,

    /// Trading volume required for next level (30-day volume in USDT)
    #[serde(rename = "nextLevelVolume")]
    pub next_level_volume: Option<String>,

    /// Current 30-day trading volume in USDT
    #[serde(rename = "currentVolume")]
    pub current_volume: String,
}

/// Account information
#[derive(Debug, Clone, Deserialize)]
pub struct AccountInfo {
    /// User ID
    #[serde(rename = "userId")]
    pub user_id: String,

    /// Spot account ID
    #[serde(rename = "spotAcctId")]
    pub spot_account_id: String,

    /// Account status
    /// - `normal`: Normal account
    /// - `frozen`: Frozen account
    pub status: String,

    /// Whether account can trade
    #[serde(rename = "canTrade")]
    pub can_trade: bool,

    /// Whether account can withdraw
    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,

    /// Whether account can deposit
    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,

    /// Account update time (Unix milliseconds)
    #[serde(rename = "updateTime")]
    pub update_time: i64,

    /// VIP level information
    #[serde(rename = "vipInfo")]
    pub vip_info: VipInfo,

    /// Whether account has kyc verification
    #[serde(rename = "kycFlag")]
    pub kyc_flag: bool,

    /// KYC level
    #[serde(rename = "kycLevel")]
    pub kyc_level: Option<String>,

    /// Parent user ID (for sub-accounts)
    #[serde(rename = "parentUserId")]
    pub parent_user_id: Option<String>,

    /// Account type
    /// - `main`: Main account
    /// - `sub`: Sub account
    #[serde(rename = "accountType")]
    pub account_type: String,
}

/// Response from the account info endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct AccountInfoResponse {
    /// Account information
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
        let query_params = serde_urlencoded::to_string(&request).map_err(|e| {
            crate::bitget::Errors::Error(format!("Failed to serialize query parameters: {e}"))
        })?;

        let query = if query_params.is_empty() {
            None
        } else {
            Some(query_params.as_str())
        };

        self.send_signed_request(
            "/api/v2/spot/account/info",
            reqwest::Method::GET,
            query, // Query parameters
            None,  // No body
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

        assert_eq!(vip_info.level, "VIP1");
        assert_eq!(vip_info.next_level, Some("VIP2".to_string()));
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
        assert_eq!(account_info.status, "normal");
        assert!(account_info.can_trade);
        assert!(account_info.can_withdraw);
        assert!(account_info.can_deposit);
        assert_eq!(account_info.update_time, 1640995200000);
        assert_eq!(account_info.vip_info.level, "VIP1");
        assert!(account_info.kyc_flag);
        assert_eq!(account_info.kyc_level, Some("2".to_string()));
        assert!(account_info.parent_user_id.is_none());
        assert_eq!(account_info.account_type, "main");
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
        assert_eq!(account_info.status, "normal");
        assert!(!account_info.can_trade);
        assert!(!account_info.can_withdraw);
        assert!(account_info.can_deposit);
        assert!(!account_info.kyc_flag);
        assert!(account_info.kyc_level.is_none());
        assert_eq!(account_info.parent_user_id, Some("987654321".to_string()));
        assert_eq!(account_info.account_type, "sub");
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
        assert_eq!(response.account_info.account_type, "main");
    }
}
