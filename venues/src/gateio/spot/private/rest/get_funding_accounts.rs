use serde::{Deserialize, Serialize};

use super::RestClient;

const MARGIN_FUNDING_ACCOUNTS_ENDPOINT: &str = "/margin/funding_accounts";

/// Request parameters for querying funding accounts in margin trading.
///
/// Used to retrieve funding account balances that are available for lending
/// in margin trading. Can optionally filter by specific currency to get
/// targeted account information.
#[derive(Debug, Clone, Serialize, Default)]
pub struct FundingAccountsRequest {
    /// Optional currency filter to retrieve specific funding account (e.g., "BTC", "USDT"). If omitted, returns all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Comprehensive funding account information for margin trading.
///
/// Contains detailed balance information for assets available for lending
/// in margin trading, including current lending status and total lending history.
/// Used to track funding account capacity and lending activity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingAccount {
    /// Currency symbol for this funding account (e.g., "BTC", "ETH", "USDT").
    pub currency: String,

    /// Available balance that can be used for new lending or withdrawals as a string to preserve precision.
    pub available: String,

    /// Locked balance that is temporarily unavailable (e.g., pending transactions) as a string.
    pub locked: String,

    /// Currently lent amount that is actively earning interest as a string to preserve precision.
    pub lent: String,

    /// Total cumulative amount that has been lent out historically as a string to preserve precision.
    pub total_lent: String,
}

impl RestClient {
    /// List funding accounts
    ///
    /// Retrieves funding account balances that are available for margin trading lending.
    /// Funding accounts contain assets that can be lent to margin traders for interest.
    /// Can filter by specific currency or retrieve all funding accounts.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#retrieve-funding-accounts)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - Optional currency filter for targeted funding account query
    ///
    /// # Returns
    /// List of funding accounts with balance details including available, locked, lent, and total lending amounts
    pub async fn get_funding_accounts(
        &self,
        params: FundingAccountsRequest,
    ) -> crate::gateio::spot::RestResult<Vec<FundingAccount>> {
        self.get_with_query(MARGIN_FUNDING_ACCOUNTS_ENDPOINT, &params)
            .await
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
            let json = format!(
                r#"{{
                "currency": "{}",
                "available": "{}",
                "locked": "{}",
                "lent": "{}",
                "total_lent": "{}"
            }}"#,
                currency, available, locked, lent, total_lent
            );

            let account: FundingAccount = serde_json::from_str(&json).unwrap();
            assert_eq!(account.currency, currency);
            assert_eq!(account.available, available);
            assert_eq!(account.locked, locked);
            assert_eq!(account.lent, lent);
            assert_eq!(account.total_lent, total_lent);
        }
    }

    #[test]
    fn test_funding_accounts_request_realistic_all_currencies_scenario() {
        // Scenario: Get all funding accounts overview
        let request = FundingAccountsRequest { currency: None };

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
    fn test_funding_accounts_request_optional_currency_behavior() {
        // Test with currency
        let request_with_currency = FundingAccountsRequest {
            currency: Some("BTC".to_string()),
        };

        // Test without currency
        let request_without_currency = FundingAccountsRequest { currency: None };

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
