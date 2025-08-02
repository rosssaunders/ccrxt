use serde::{Deserialize, Serialize};

use super::RestClient;

const MARGIN_FUNDING_ACCOUNTS_ENDPOINT: &str = "/margin/funding_accounts";
const MARGIN_TRANSFERABLE_ENDPOINT: &str = "/margin/transferable";
const MARGIN_BORROWABLE_ENDPOINT: &str = "/margin/borrowable";
const MARGIN_AUTO_REPAY_ENDPOINT: &str = "/margin/auto_repay";

/// Request parameters for funding accounts
#[derive(Debug, Clone, Serialize, Default)]
pub struct FundingAccountsRequest {
    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Funding account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingAccount {
    /// Currency
    pub currency: String,

    /// Available balance
    pub available: String,

    /// Locked balance
    pub locked: String,

    /// Lent amount
    pub lent: String,

    /// Total lending balance
    pub total_lent: String,
}

/// Request parameters for transferable amount
#[derive(Debug, Clone, Serialize)]
pub struct TransferableRequest {
    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,
}

/// Transferable amount information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferableAmount {
    /// Currency
    pub currency: String,

    /// Available amount for transfer
    pub amount: String,
}

/// Request parameters for borrowable amount
#[derive(Debug, Clone, Serialize)]
pub struct BorrowableRequest {
    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,
}

/// Borrowable amount information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowableAmount {
    /// Currency
    pub currency: String,

    /// Amount available for borrowing
    pub amount: String,
}

/// Request parameters for auto repay settings
#[derive(Debug, Clone, Serialize, Default)]
pub struct AutoRepayRequest {
    /// Status (on/off)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Auto repay settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoRepaySetting {
    /// Auto repay status
    pub status: String,
}

impl RestClient {
    /// Get funding accounts
    ///
    /// This endpoint returns funding account balances for margin trading.
    /// Funding accounts hold assets that can be lent out for margin trading.
    pub async fn get_funding_accounts(
        &self,
        params: FundingAccountsRequest,
    ) -> crate::gateio::spot::Result<Vec<FundingAccount>> {
        self.get_with_query(MARGIN_FUNDING_ACCOUNTS_ENDPOINT, &params)
            .await
    }

    /// Get transferable amount
    ///
    /// This endpoint returns the amount that can be transferred for a specific
    /// currency and currency pair in margin trading.
    pub async fn get_transferable(
        &self,
        params: TransferableRequest,
    ) -> crate::gateio::spot::Result<TransferableAmount> {
        self.get_with_query(MARGIN_TRANSFERABLE_ENDPOINT, &params).await
    }

    /// Get borrowable amount
    ///
    /// This endpoint returns the amount that can be borrowed for a specific
    /// currency and currency pair in margin trading.
    pub async fn get_borrowable(
        &self,
        params: BorrowableRequest,
    ) -> crate::gateio::spot::Result<BorrowableAmount> {
        self.get_with_query(MARGIN_BORROWABLE_ENDPOINT, &params).await
    }

    /// Get auto repay settings
    ///
    /// This endpoint returns the current auto repay settings for margin trading.
    pub async fn get_auto_repay(&self) -> crate::gateio::spot::Result<AutoRepaySetting> {
        self.get(MARGIN_AUTO_REPAY_ENDPOINT).await
    }

    /// Update auto repay settings
    ///
    /// This endpoint updates the auto repay settings for margin trading.
    pub async fn update_auto_repay(
        &self,
        params: AutoRepayRequest,
    ) -> crate::gateio::spot::Result<AutoRepaySetting> {
        self.post(MARGIN_AUTO_REPAY_ENDPOINT, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funding_accounts_request_default() {
        let request = FundingAccountsRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty()); // No fields should be serialized when None
    }

    #[test]
    fn test_funding_accounts_request_with_currency() {
        let request = FundingAccountsRequest {
            currency: Some("BTC".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
    }

    #[test]
    fn test_funding_accounts_request_different_currencies() {
        let currencies = vec!["BTC", "ETH", "USDT", "USDC", "BNB", "SOL"];

        for currency in currencies {
            let request = FundingAccountsRequest {
                currency: Some(currency.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency"], currency);
        }
    }

    #[test]
    fn test_funding_account_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "available": "1.5",
            "locked": "0.5",
            "lent": "0.2",
            "total_lent": "2.0"
        }"#;

        let account: FundingAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.currency, "BTC");
        assert_eq!(account.available, "1.5");
        assert_eq!(account.locked, "0.5");
        assert_eq!(account.lent, "0.2");
        assert_eq!(account.total_lent, "2.0");
    }

    #[test]
    fn test_funding_account_multiple_currencies() {
        let currencies = vec![
            ("BTC", "1.5", "0.5", "0.2", "2.0"),
            ("ETH", "10.0", "2.0", "1.0", "15.0"),
            ("USDT", "50000.0", "10000.0", "5000.0", "80000.0"),
            ("USDC", "25000.0", "5000.0", "2500.0", "40000.0"),
        ];

        for (currency, available, locked, lent, total_lent) in currencies {
            let json = format!(r#"{{
                "currency": "{}",
                "available": "{}",
                "locked": "{}",
                "lent": "{}",
                "total_lent": "{}"
            }}"#, currency, available, locked, lent, total_lent);

            let account: FundingAccount = serde_json::from_str(&json).unwrap();
            assert_eq!(account.currency, currency);
            assert_eq!(account.available, available);
            assert_eq!(account.locked, locked);
            assert_eq!(account.lent, lent);
            assert_eq!(account.total_lent, total_lent);
        }
    }

    #[test]
    fn test_transferable_request_serialization() {
        let request = TransferableRequest {
            currency: "BTC".to_string(),
            currency_pair: "BTC_USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["currency_pair"], "BTC_USDT");
    }

    #[test]
    fn test_transferable_request_different_pairs() {
        let pairs = vec![
            ("BTC", "BTC_USDT"),
            ("ETH", "ETH_USDT"),
            ("BNB", "BNB_USDT"),
            ("SOL", "SOL_USDC"),
            ("ETH", "ETH_BTC"),
            ("USDC", "USDC_USDT"),
        ];

        for (currency, currency_pair) in pairs {
            let request = TransferableRequest {
                currency: currency.to_string(),
                currency_pair: currency_pair.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency"], currency);
            assert_eq!(json["currency_pair"], currency_pair);
        }
    }

    #[test]
    fn test_transferable_amount_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "amount": "0.5"
        }"#;

        let transferable: TransferableAmount = serde_json::from_str(json).unwrap();
        assert_eq!(transferable.currency, "BTC");
        assert_eq!(transferable.amount, "0.5");
    }

    #[test]
    fn test_transferable_amount_different_values() {
        let amounts = vec![
            ("BTC", "0.5"),
            ("ETH", "10.0"),
            ("USDT", "50000.0"),
            ("USDC", "25000.0"),
            ("BNB", "100.0"),
            ("SOL", "500.0"),
        ];

        for (currency, amount) in amounts {
            let json = format!(r#"{{
                "currency": "{}",
                "amount": "{}"
            }}"#, currency, amount);

            let transferable: TransferableAmount = serde_json::from_str(&json).unwrap();
            assert_eq!(transferable.currency, currency);
            assert_eq!(transferable.amount, amount);
        }
    }

    #[test]
    fn test_borrowable_request_serialization() {
        let request = BorrowableRequest {
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["currency_pair"], "BTC_USDT");
    }

    #[test]
    fn test_borrowable_request_different_scenarios() {
        let scenarios = vec![
            ("USDT", "BTC_USDT", "Borrow USDT for BTC pair"),
            ("BTC", "BTC_USDT", "Borrow BTC for BTC pair"),
            ("ETH", "ETH_USDT", "Borrow ETH for ETH pair"),
            ("USDT", "ETH_USDT", "Borrow USDT for ETH pair"),
            ("USDC", "SOL_USDC", "Borrow USDC for SOL pair"),
            ("BTC", "ETH_BTC", "Borrow BTC for ETH/BTC pair"),
        ];

        for (currency, currency_pair, _description) in scenarios {
            let request = BorrowableRequest {
                currency: currency.to_string(),
                currency_pair: currency_pair.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency"], currency);
            assert_eq!(json["currency_pair"], currency_pair);
        }
    }

    #[test]
    fn test_borrowable_amount_deserialization() {
        let json = r#"{
            "currency": "USDT",
            "amount": "10000.0"
        }"#;

        let borrowable: BorrowableAmount = serde_json::from_str(json).unwrap();
        assert_eq!(borrowable.currency, "USDT");
        assert_eq!(borrowable.amount, "10000.0");
    }

    #[test]
    fn test_borrowable_amount_different_limits() {
        let limits = vec![
            ("BTC", "0.1"),
            ("ETH", "5.0"),
            ("USDT", "100000.0"),
            ("USDC", "75000.0"),
            ("BNB", "500.0"),
            ("SOL", "2000.0"),
        ];

        for (currency, amount) in limits {
            let json = format!(r#"{{
                "currency": "{}",
                "amount": "{}"
            }}"#, currency, amount);

            let borrowable: BorrowableAmount = serde_json::from_str(&json).unwrap();
            assert_eq!(borrowable.currency, currency);
            assert_eq!(borrowable.amount, amount);
        }
    }

    #[test]
    fn test_auto_repay_request_default() {
        let request = AutoRepayRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty()); // No fields should be serialized when None
    }

    #[test]
    fn test_auto_repay_request_with_status() {
        let request = AutoRepayRequest {
            status: Some("on".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["status"], "on");
    }

    #[test]
    fn test_auto_repay_request_different_statuses() {
        let statuses = vec!["on", "off"];

        for status in statuses {
            let request = AutoRepayRequest {
                status: Some(status.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["status"], status);
        }
    }

    #[test]
    fn test_auto_repay_setting_deserialization() {
        let json = r#"{
            "status": "on"
        }"#;

        let setting: AutoRepaySetting = serde_json::from_str(json).unwrap();
        assert_eq!(setting.status, "on");
    }

    #[test]
    fn test_auto_repay_setting_different_statuses() {
        let statuses = vec!["on", "off"];

        for status in statuses {
            let json = format!(r#"{{
                "status": "{}"
            }}"#, status);

            let setting: AutoRepaySetting = serde_json::from_str(&json).unwrap();
            assert_eq!(setting.status, status);
        }
    }

    #[test]
    fn test_funding_accounts_request_realistic_all_currencies_scenario() {
        // Scenario: Get all funding accounts overview
        let request = FundingAccountsRequest {
            currency: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty()); // No filter, get all currencies
    }

    #[test]
    fn test_funding_accounts_request_realistic_specific_currency_scenario() {
        // Scenario: Check specific BTC funding account
        let request = FundingAccountsRequest {
            currency: Some("BTC".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
    }

    #[test]
    fn test_funding_account_realistic_lending_scenario() {
        let json = r#"{
            "currency": "USDT",
            "available": "50000.0",
            "locked": "10000.0",
            "lent": "25000.0",
            "total_lent": "85000.0"
        }"#;

        let account: FundingAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.currency, "USDT");
        
        // Verify lending calculations
        let available: f64 = account.available.parse().unwrap();
        let locked: f64 = account.locked.parse().unwrap();
        let lent: f64 = account.lent.parse().unwrap();
        let total_balance = available + locked + lent;
        assert_eq!(total_balance, 85000.0);
    }

    #[test]
    fn test_transferable_request_realistic_margin_transfer_scenario() {
        // Scenario: Check transferable BTC for margin trading
        let request = TransferableRequest {
            currency: "BTC".to_string(),
            currency_pair: "BTC_USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["currency_pair"], "BTC_USDT");
    }

    #[test]
    fn test_transferable_amount_realistic_available_balance_scenario() {
        let json = r#"{
            "currency": "BTC",
            "amount": "0.75"
        }"#;

        let transferable: TransferableAmount = serde_json::from_str(json).unwrap();
        assert_eq!(transferable.currency, "BTC");
        assert_eq!(transferable.amount, "0.75");

        // Verify amount is a valid decimal
        let amount: f64 = transferable.amount.parse().unwrap();
        assert!(amount > 0.0);
    }

    #[test]
    fn test_borrowable_request_realistic_leverage_scenario() {
        // Scenario: Check borrowable USDT for leveraged BTC position
        let request = BorrowableRequest {
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["currency_pair"], "BTC_USDT");
    }

    #[test]
    fn test_borrowable_amount_realistic_margin_limit_scenario() {
        let json = r#"{
            "currency": "USDT",
            "amount": "150000.0"
        }"#;

        let borrowable: BorrowableAmount = serde_json::from_str(json).unwrap();
        assert_eq!(borrowable.currency, "USDT");
        assert_eq!(borrowable.amount, "150000.0");

        // Verify high borrowing limit for USDT
        let amount: f64 = borrowable.amount.parse().unwrap();
        assert!(amount >= 100000.0); // High limit for stablecoin
    }

    #[test]
    fn test_auto_repay_request_realistic_enable_scenario() {
        // Scenario: Enable auto repay for risk management
        let request = AutoRepayRequest {
            status: Some("on".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["status"], "on");
    }

    #[test]
    fn test_auto_repay_request_realistic_disable_scenario() {
        // Scenario: Disable auto repay for manual control
        let request = AutoRepayRequest {
            status: Some("off".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["status"], "off");
    }

    #[test]
    fn test_auto_repay_setting_realistic_enabled_scenario() {
        let json = r#"{
            "status": "on"
        }"#;

        let setting: AutoRepaySetting = serde_json::from_str(json).unwrap();
        assert_eq!(setting.status, "on");
        assert!(setting.status == "on" || setting.status == "off");
    }

    #[test]
    fn test_funding_account_high_precision_amounts() {
        let json = r#"{
            "currency": "BTC",
            "available": "1.23456789",
            "locked": "0.12345678",
            "lent": "0.01234567",
            "total_lent": "1.37036035"
        }"#;

        let account: FundingAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.available, "1.23456789");
        assert_eq!(account.locked, "0.12345678");
        assert_eq!(account.lent, "0.01234567");
        assert_eq!(account.total_lent, "1.37036035");
    }

    #[test]
    fn test_transferable_amount_zero_balance() {
        let json = r#"{
            "currency": "ETH",
            "amount": "0"
        }"#;

        let transferable: TransferableAmount = serde_json::from_str(json).unwrap();
        assert_eq!(transferable.currency, "ETH");
        assert_eq!(transferable.amount, "0");
    }

    #[test]
    fn test_borrowable_amount_maximum_precision() {
        let json = r#"{
            "currency": "BTC",
            "amount": "0.00000001"
        }"#;

        let borrowable: BorrowableAmount = serde_json::from_str(json).unwrap();
        assert_eq!(borrowable.currency, "BTC");
        assert_eq!(borrowable.amount, "0.00000001");
    }

    #[test]
    fn test_funding_accounts_request_clone() {
        let original = FundingAccountsRequest {
            currency: Some("BTC".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency, original.currency);
    }

    #[test]
    fn test_funding_account_clone() {
        let original = FundingAccount {
            currency: "BTC".to_string(),
            available: "1.5".to_string(),
            locked: "0.5".to_string(),
            lent: "0.2".to_string(),
            total_lent: "2.0".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.available, original.available);
        assert_eq!(cloned.locked, original.locked);
        assert_eq!(cloned.lent, original.lent);
        assert_eq!(cloned.total_lent, original.total_lent);
    }

    #[test]
    fn test_transferable_request_clone() {
        let original = TransferableRequest {
            currency: "BTC".to_string(),
            currency_pair: "BTC_USDT".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.currency_pair, original.currency_pair);
    }

    #[test]
    fn test_transferable_amount_clone() {
        let original = TransferableAmount {
            currency: "BTC".to_string(),
            amount: "0.5".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.amount, original.amount);
    }

    #[test]
    fn test_borrowable_request_clone() {
        let original = BorrowableRequest {
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.currency_pair, original.currency_pair);
    }

    #[test]
    fn test_borrowable_amount_clone() {
        let original = BorrowableAmount {
            currency: "USDT".to_string(),
            amount: "10000.0".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.amount, original.amount);
    }

    #[test]
    fn test_auto_repay_request_clone() {
        let original = AutoRepayRequest {
            status: Some("on".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.status, original.status);
    }

    #[test]
    fn test_auto_repay_setting_clone() {
        let original = AutoRepaySetting {
            status: "on".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(cloned.status, original.status);
    }

    #[test]
    fn test_funding_accounts_request_debug() {
        let request = FundingAccountsRequest {
            currency: Some("BTC".to_string()),
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("FundingAccountsRequest"));
        assert!(debug_str.contains("BTC"));
    }

    #[test]
    fn test_funding_account_debug() {
        let account = FundingAccount {
            currency: "BTC".to_string(),
            available: "1.5".to_string(),
            locked: "0.5".to_string(),
            lent: "0.2".to_string(),
            total_lent: "2.0".to_string(),
        };

        let debug_str = format!("{:?}", account);
        assert!(debug_str.contains("FundingAccount"));
        assert!(debug_str.contains("BTC"));
    }

    #[test]
    fn test_auto_repay_setting_debug() {
        let setting = AutoRepaySetting {
            status: "on".to_string(),
        };

        let debug_str = format!("{:?}", setting);
        assert!(debug_str.contains("AutoRepaySetting"));
        assert!(debug_str.contains("on"));
    }

    #[test]
    fn test_funding_account_serialization() {
        let account = FundingAccount {
            currency: "BTC".to_string(),
            available: "1.5".to_string(),
            locked: "0.5".to_string(),
            lent: "0.2".to_string(),
            total_lent: "2.0".to_string(),
        };

        let json = serde_json::to_value(&account).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["available"], "1.5");
        assert_eq!(json["locked"], "0.5");
        assert_eq!(json["lent"], "0.2");
        assert_eq!(json["total_lent"], "2.0");
    }

    #[test]
    fn test_transferable_amount_serialization() {
        let transferable = TransferableAmount {
            currency: "BTC".to_string(),
            amount: "0.5".to_string(),
        };

        let json = serde_json::to_value(&transferable).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["amount"], "0.5");
    }

    #[test]
    fn test_borrowable_amount_serialization() {
        let borrowable = BorrowableAmount {
            currency: "USDT".to_string(),
            amount: "10000.0".to_string(),
        };

        let json = serde_json::to_value(&borrowable).unwrap();
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["amount"], "10000.0");
    }

    #[test]
    fn test_auto_repay_setting_serialization() {
        let setting = AutoRepaySetting {
            status: "on".to_string(),
        };

        let json = serde_json::to_value(&setting).unwrap();
        assert_eq!(json["status"], "on");
    }

    #[test]
    fn test_funding_accounts_request_endpoint_validation() {
        let request = FundingAccountsRequest {
            currency: Some("BTC".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json.as_object().unwrap().contains_key("currency"));
        
        // Verify currency is a string
        assert!(json["currency"].is_string());
    }

    #[test]
    fn test_transferable_request_endpoint_validation() {
        let request = TransferableRequest {
            currency: "BTC".to_string(),
            currency_pair: "BTC_USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json.as_object().unwrap().contains_key("currency"));
        assert!(json.as_object().unwrap().contains_key("currency_pair"));
        
        // Verify required fields are strings
        assert!(json["currency"].is_string());
        assert!(json["currency_pair"].is_string());
    }

    #[test]
    fn test_borrowable_request_endpoint_validation() {
        let request = BorrowableRequest {
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json.as_object().unwrap().contains_key("currency"));
        assert!(json.as_object().unwrap().contains_key("currency_pair"));
        
        // Verify required fields are strings
        assert!(json["currency"].is_string());
        assert!(json["currency_pair"].is_string());
    }

    #[test]
    fn test_auto_repay_request_endpoint_validation() {
        let request = AutoRepayRequest {
            status: Some("on".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json.as_object().unwrap().contains_key("status"));
        
        // Verify status is a string
        assert!(json["status"].is_string());
    }

    #[test]
    fn test_funding_account_round_trip() {
        let original = FundingAccount {
            currency: "BTC".to_string(),
            available: "1.5".to_string(),
            locked: "0.5".to_string(),
            lent: "0.2".to_string(),
            total_lent: "2.0".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: FundingAccount = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.currency, original.currency);
        assert_eq!(deserialized.available, original.available);
        assert_eq!(deserialized.locked, original.locked);
        assert_eq!(deserialized.lent, original.lent);
        assert_eq!(deserialized.total_lent, original.total_lent);
    }

    #[test]
    fn test_transferable_amount_round_trip() {
        let original = TransferableAmount {
            currency: "BTC".to_string(),
            amount: "0.5".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TransferableAmount = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.currency, original.currency);
        assert_eq!(deserialized.amount, original.amount);
    }

    #[test]
    fn test_borrowable_amount_round_trip() {
        let original = BorrowableAmount {
            currency: "USDT".to_string(),
            amount: "10000.0".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: BorrowableAmount = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.currency, original.currency);
        assert_eq!(deserialized.amount, original.amount);
    }

    #[test]
    fn test_auto_repay_setting_round_trip() {
        let original = AutoRepaySetting {
            status: "on".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: AutoRepaySetting = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.status, original.status);
    }

    #[test]
    fn test_funding_accounts_request_optional_currency_behavior() {
        // Test with currency
        let request_with_currency = FundingAccountsRequest {
            currency: Some("BTC".to_string()),
        };

        // Test without currency
        let request_without_currency = FundingAccountsRequest {
            currency: None,
        };

        let json_with = serde_json::to_value(&request_with_currency).unwrap();
        let json_without = serde_json::to_value(&request_without_currency).unwrap();

        // With currency - should be included
        let obj_with = json_with.as_object().unwrap();
        assert!(obj_with.contains_key("currency"));
        assert_eq!(obj_with.len(), 1);

        // Without currency - should be omitted
        let obj_without = json_without.as_object().unwrap();
        assert!(!obj_without.contains_key("currency"));
        assert_eq!(obj_without.len(), 0);
    }

    #[test]
    fn test_auto_repay_request_optional_status_behavior() {
        // Test with status
        let request_with_status = AutoRepayRequest {
            status: Some("on".to_string()),
        };

        // Test without status
        let request_without_status = AutoRepayRequest {
            status: None,
        };

        let json_with = serde_json::to_value(&request_with_status).unwrap();
        let json_without = serde_json::to_value(&request_without_status).unwrap();

        // With status - should be included
        let obj_with = json_with.as_object().unwrap();
        assert!(obj_with.contains_key("status"));
        assert_eq!(obj_with.len(), 1);

        // Without status - should be omitted
        let obj_without = json_without.as_object().unwrap();
        assert!(!obj_without.contains_key("status"));
        assert_eq!(obj_without.len(), 0);
    }

    #[test]
    fn test_funding_account_balance_calculations() {
        let json = r#"{
            "currency": "USDT",
            "available": "10000.0",
            "locked": "5000.0",
            "lent": "15000.0",
            "total_lent": "30000.0"
        }"#;

        let account: FundingAccount = serde_json::from_str(json).unwrap();
        
        // Verify balance calculations
        let available: f64 = account.available.parse().unwrap();
        let locked: f64 = account.locked.parse().unwrap();
        let lent: f64 = account.lent.parse().unwrap();
        
        let current_balance = available + locked + lent;
        assert_eq!(current_balance, 30000.0);
        
        let total_lent: f64 = account.total_lent.parse().unwrap();
        assert_eq!(total_lent, 30000.0);
    }
}
