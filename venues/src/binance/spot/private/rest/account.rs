use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

const GET_ACCOUNT_ENDPOINT: &str = "/api/v3/account";

/// Request parameters for getting account information
#[derive(Debug, Clone, Serialize, Default)]
pub struct AccountRequest {
    /// Omit zero balances in the response
    #[serde(rename = "omitZeroBalances", skip_serializing_if = "Option::is_none")]
    pub omit_zero_balances: Option<bool>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Account information response
#[derive(Debug, Clone, Deserialize)]
pub struct AccountResponse {
    /// Maker commission rate (bips)
    #[serde(rename = "makerCommission")]
    pub maker_commission: u32,

    /// Taker commission rate (bips)
    #[serde(rename = "takerCommission")]
    pub taker_commission: u32,

    /// Buyer commission rate (bips)
    #[serde(rename = "buyerCommission")]
    pub buyer_commission: u32,

    /// Seller commission rate (bips)
    #[serde(rename = "sellerCommission")]
    pub seller_commission: u32,

    /// Commission rates
    #[serde(rename = "commissionRates")]
    pub commission_rates: CommissionRates,

    /// Can trade
    #[serde(rename = "canTrade")]
    pub can_trade: bool,

    /// Can withdraw
    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,

    /// Can deposit
    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,

    /// Buy BNB enabled
    #[serde(rename = "brokered")]
    pub brokered: bool,

    /// Require self trade prevention
    #[serde(rename = "requireSelfTradePrevention")]
    pub require_self_trade_prevention: bool,

    /// Prevent SOR
    #[serde(rename = "preventSor")]
    pub prevent_sor: bool,

    /// Update time
    #[serde(rename = "updateTime")]
    pub update_time: u64,

    /// Account type
    #[serde(rename = "accountType")]
    pub account_type: String,

    /// Account balances
    #[serde(rename = "balances")]
    pub balances: Vec<Balance>,

    /// Account permissions
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,

    /// UID
    #[serde(rename = "uid")]
    pub uid: u64,
}

/// Commission rates information
#[derive(Debug, Clone, Deserialize)]
pub struct CommissionRates {
    /// Maker commission rate
    #[serde(rename = "maker")]
    pub maker: Decimal,

    /// Taker commission rate
    #[serde(rename = "taker")]
    pub taker: Decimal,

    /// Buyer commission rate
    #[serde(rename = "buyer")]
    pub buyer: Decimal,

    /// Seller commission rate
    #[serde(rename = "seller")]
    pub seller: Decimal,
}

/// Account balance information
#[derive(Debug, Clone, Deserialize)]
pub struct Balance {
    /// Asset name
    #[serde(rename = "asset")]
    pub asset: String,

    /// Free balance
    #[serde(rename = "free")]
    pub free: Decimal,

    /// Locked balance
    #[serde(rename = "locked")]
    pub locked: Decimal,
}

impl RestClient {
    /// Get current account information
    ///
    /// Get current account information.
    ///
    /// [docs]: (https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#account-information--user_data)
    /// Method: GET /api/v3/account
    /// Weight: 20
    /// Security: USER_DATA
    pub async fn get_account(&self, params: Option<AccountRequest>) -> RestResult<AccountResponse> {
        self.send_get_signed_request(
            GET_ACCOUNT_ENDPOINT,
            params.unwrap_or_default(),
            20,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_request_serialization_empty() {
        let request = AccountRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_account_request_serialization_omit_zero_balances() {
        let request = AccountRequest {
            omit_zero_balances: Some(true),
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "omitZeroBalances=true");
    }

    #[test]
    fn test_account_request_serialization_full() {
        let request = AccountRequest {
            omit_zero_balances: Some(false),
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("omitZeroBalances=false"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_commission_rates_deserialization() {
        let json = r#"{
            "maker": "0.00100000",
            "taker": "0.00100000",
            "buyer": "0.00000000",
            "seller": "0.00000000"
        }"#;

        let rates: CommissionRates = serde_json::from_str(json).unwrap();
        assert_eq!(rates.maker.to_string(), "0.00100000");
        assert_eq!(rates.taker.to_string(), "0.00100000");
        assert_eq!(rates.buyer.to_string(), "0.00000000");
        assert_eq!(rates.seller.to_string(), "0.00000000");
    }

    #[test]
    fn test_balance_deserialization() {
        let json = r#"{
            "asset": "BTC",
            "free": "0.50000000",
            "locked": "0.10000000"
        }"#;

        let balance: Balance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.asset, "BTC");
        assert_eq!(balance.free.to_string(), "0.50000000");
        assert_eq!(balance.locked.to_string(), "0.10000000");
    }

    #[test]
    fn test_account_response_deserialization() {
        let json = r#"{
            "makerCommission": 10,
            "takerCommission": 10,
            "buyerCommission": 0,
            "sellerCommission": 0,
            "commissionRates": {
                "maker": "0.00100000",
                "taker": "0.00100000",
                "buyer": "0.00000000",
                "seller": "0.00000000"
            },
            "canTrade": true,
            "canWithdraw": true,
            "canDeposit": true,
            "brokered": false,
            "requireSelfTradePrevention": false,
            "preventSor": false,
            "updateTime": 1625184000000,
            "accountType": "SPOT",
            "balances": [
                {
                    "asset": "BTC",
                    "free": "4.00000000",
                    "locked": "0.00000000"
                },
                {
                    "asset": "USDT",
                    "free": "1000.00000000",
                    "locked": "500.00000000"
                }
            ],
            "permissions": ["SPOT"],
            "uid": 123456789
        }"#;

        let account: AccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(account.maker_commission, 10);
        assert_eq!(account.taker_commission, 10);
        assert_eq!(account.buyer_commission, 0);
        assert_eq!(account.seller_commission, 0);
        assert!(account.can_trade);
        assert!(account.can_withdraw);
        assert!(account.can_deposit);
        assert!(!account.brokered);
        assert!(!account.require_self_trade_prevention);
        assert!(!account.prevent_sor);
        assert_eq!(account.update_time, 1625184000000);
        assert_eq!(account.account_type, "SPOT");
        assert_eq!(account.balances.len(), 2);
        assert_eq!(account.permissions, vec!["SPOT"]);
        assert_eq!(account.uid, 123456789);
    }

    #[test]
    fn test_account_response_with_multiple_permissions() {
        let json = r#"{
            "makerCommission": 15,
            "takerCommission": 15,
            "buyerCommission": 0,
            "sellerCommission": 0,
            "commissionRates": {
                "maker": "0.00150000",
                "taker": "0.00150000",
                "buyer": "0.00000000",
                "seller": "0.00000000"
            },
            "canTrade": true,
            "canWithdraw": true,
            "canDeposit": true,
            "brokered": false,
            "requireSelfTradePrevention": true,
            "preventSor": true,
            "updateTime": 1625184000000,
            "accountType": "SPOT",
            "balances": [],
            "permissions": ["SPOT", "MARGIN", "FUTURES"],
            "uid": 987654321
        }"#;

        let account: AccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(account.permissions.len(), 3);
        assert_eq!(account.permissions[0], "SPOT");
        assert_eq!(account.permissions[1], "MARGIN");
        assert_eq!(account.permissions[2], "FUTURES");
        assert!(account.require_self_trade_prevention);
        assert!(account.prevent_sor);
    }

    #[test]
    fn test_account_response_empty_balances() {
        let json = r#"{
            "makerCommission": 10,
            "takerCommission": 10,
            "buyerCommission": 0,
            "sellerCommission": 0,
            "commissionRates": {
                "maker": "0.00100000",
                "taker": "0.00100000",
                "buyer": "0.00000000",
                "seller": "0.00000000"
            },
            "canTrade": true,
            "canWithdraw": true,
            "canDeposit": true,
            "brokered": false,
            "requireSelfTradePrevention": false,
            "preventSor": false,
            "updateTime": 1625184000000,
            "accountType": "SPOT",
            "balances": [],
            "permissions": ["SPOT"],
            "uid": 123456789
        }"#;

        let account: AccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(account.balances.len(), 0);
    }

    #[test]
    fn test_balance_zero_amounts() {
        let json = r#"{
            "asset": "XRP",
            "free": "0.00000000",
            "locked": "0.00000000"
        }"#;

        let balance: Balance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.asset, "XRP");
        assert_eq!(balance.free.to_string(), "0.00000000");
        assert_eq!(balance.locked.to_string(), "0.00000000");
    }

    #[test]
    fn test_balance_high_precision() {
        let json = r#"{
            "asset": "DOGE",
            "free": "12345678.90123456",
            "locked": "987654.32109876"
        }"#;

        let balance: Balance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.asset, "DOGE");
        assert_eq!(balance.free.to_string(), "12345678.90123456");
        assert_eq!(balance.locked.to_string(), "987654.32109876");
    }

    #[test]
    fn test_account_response_various_commission_rates() {
        let json = r#"{
            "makerCommission": 5,
            "takerCommission": 10,
            "buyerCommission": 15,
            "sellerCommission": 20,
            "commissionRates": {
                "maker": "0.00050000",
                "taker": "0.00100000",
                "buyer": "0.00150000",
                "seller": "0.00200000"
            },
            "canTrade": true,
            "canWithdraw": false,
            "canDeposit": true,
            "brokered": true,
            "requireSelfTradePrevention": false,
            "preventSor": false,
            "updateTime": 1625184000000,
            "accountType": "SPOT",
            "balances": [
                {
                    "asset": "ETH",
                    "free": "10.50000000",
                    "locked": "2.50000000"
                }
            ],
            "permissions": ["SPOT"],
            "uid": 123456789
        }"#;

        let account: AccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(account.maker_commission, 5);
        assert_eq!(account.taker_commission, 10);
        assert_eq!(account.buyer_commission, 15);
        assert_eq!(account.seller_commission, 20);
        assert!(!account.can_withdraw);
        assert!(account.brokered);
        assert_eq!(account.commission_rates.maker.to_string(), "0.00050000");
        assert_eq!(account.commission_rates.taker.to_string(), "0.00100000");
        assert_eq!(account.commission_rates.buyer.to_string(), "0.00150000");
        assert_eq!(account.commission_rates.seller.to_string(), "0.00200000");
    }
}
