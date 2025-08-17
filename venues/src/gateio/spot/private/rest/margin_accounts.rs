use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::spot::RestResult;

const MARGIN_ACCOUNTS_ENDPOINT: &str = "/margin/accounts";

/// Request parameters for querying margin account information with optional currency pair filtering.
///
/// Used to retrieve margin account details including balances, borrowed amounts,
/// risk levels, and currency-specific information for margin trading pairs with
/// comprehensive account status and lending activity overview.
#[derive(Debug, Clone, Serialize, Default)]
pub struct MarginAccountsRequest {
    /// Optional trading pair filter for specific margin account (e.g., "BTC_USDT", "ETH_USDT").
    ///
    /// When specified, returns only the margin account for the requested trading pair.
    /// When omitted, returns all margin accounts for all available trading pairs.
    /// Format: "{BASE}_{QUOTE}" following Gate.io pair naming convention.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
}

/// Comprehensive margin account information with trading pair balance and risk details.
///
/// Represents complete margin account status including base and quote currency
/// balances, borrowed amounts, interest charges, and risk assessment for a
/// specific trading pair with locked status and leverage information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginAccount {
    /// Trading pair identifier in "{BASE}_{QUOTE}" format (e.g., "BTC_USDT", "ETH_USDT").
    pub currency_pair: String,

    /// Account lock status indicating if trading is restricted due to high risk or liquidation.
    ///
    /// When true, the account may be restricted from new borrowing or trading activities
    /// due to risk levels or margin requirements not being met.
    pub locked: bool,

    /// Current risk ratio as decimal string representing margin utilization level.
    ///
    /// Higher values (closer to 1.0) indicate higher risk of liquidation.
    /// Typically ranges from 0.0 (low risk) to 1.0 (maximum risk before liquidation).
    pub risk: String,

    /// Base currency balance details including available, locked, and borrowed amounts.
    pub base: MarginBalance,

    /// Quote currency balance details including available, locked, and borrowed amounts.
    pub quote: MarginBalance,
}

/// Detailed balance information for a single currency in margin trading account.
///
/// Contains comprehensive balance breakdown including available funds, locked amounts,
/// borrowed balances, and accrued interest charges for margin trading operations
/// with precision string formatting to preserve exact decimal values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginBalance {
    /// Currency code identifier (e.g., "BTC", "ETH", "USDT").
    pub currency: String,

    /// Available balance that can be used for trading or withdrawal as decimal string.
    ///
    /// Represents the portion of balance not locked in orders or used as collateral.
    /// This amount can be used to place new orders or transferred out of the account.
    pub available: String,

    /// Balance currently locked in open orders or pending operations as decimal string.
    ///
    /// Includes funds reserved for open orders, pending transfers, or other operations
    /// that temporarily restrict access to the balance until completion.
    pub locked: String,

    /// Total amount borrowed for this currency as decimal string.
    ///
    /// Represents outstanding loan principal that must be repaid along with accrued
    /// interest. Zero indicates no borrowing for this currency.
    pub borrowed: String,

    /// Accrued interest charges on borrowed amount as decimal string.
    ///
    /// Interest accumulates over time on borrowed balances and must be repaid
    /// along with the principal. Zero when no borrowing or fully paid interest.
    pub interest: String,
}

impl RestClient {
    /// Margin Accounts
    ///
    /// Retrieve margin account information including balances, borrowed amounts,
    /// and risk levels for all or specific currency pairs with comprehensive
    /// account status and lending activity details.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/#margin-accounts)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - Request parameters for filtering margin accounts
    ///
    /// # Returns
    /// List of margin account information for requested trading pairs
    pub async fn get_margin_accounts(
        &self,
        params: MarginAccountsRequest,
    ) -> RestResult<Vec<MarginAccount>> {
        self.get_with_query(MARGIN_ACCOUNTS_ENDPOINT, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test margin accounts request serialization with default parameters.
    #[test]
    fn test_margin_accounts_request_default() {
        let request = MarginAccountsRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty()); // No fields should be serialized when None
    }

    #[test]
    fn test_margin_accounts_request_with_currency_pair() {
        let request = MarginAccountsRequest {
            currency_pair: Some("BTC_USDT".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
    }

    #[test]
    fn test_margin_accounts_request_different_currency_pairs() {
        let pairs = vec!["BTC_USDT", "ETH_USDT", "LTC_BTC", "XRP_USDT", "DOT_USDT"];

        for pair in pairs {
            let request = MarginAccountsRequest {
                currency_pair: Some(pair.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency_pair"], pair);
        }
    }

    #[test]
    fn test_margin_account_deserialization() {
        let json = r#"{
            "currency_pair": "BTC_USDT",
            "locked": false,
            "risk": "0.1",
            "base": {
                "currency": "BTC",
                "available": "1.0",
                "locked": "0.5",
                "borrowed": "0.0",
                "interest": "0.0"
            },
            "quote": {
                "currency": "USDT",
                "available": "10000.0",
                "locked": "5000.0",
                "borrowed": "2000.0",
                "interest": "0.5"
            }
        }"#;

        let account: MarginAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.currency_pair, "BTC_USDT");
        assert!(!account.locked);
        assert_eq!(account.risk, "0.1");
        assert_eq!(account.base.currency, "BTC");
        assert_eq!(account.base.available, "1.0");
        assert_eq!(account.quote.currency, "USDT");
        assert_eq!(account.quote.borrowed, "2000.0");
    }

    #[test]
    fn test_margin_account_with_different_risk_levels() {
        let risk_levels = vec!["0.0", "0.1", "0.5", "0.75", "0.9", "1.0"];

        for risk in risk_levels {
            let json = format!(
                r#"{{
                    "currency_pair": "BTC_USDT",
                    "locked": false,
                    "risk": "{}",
                    "base": {{
                        "currency": "BTC",
                        "available": "1.0",
                        "locked": "0.0",
                        "borrowed": "0.0",
                        "interest": "0.0"
                    }},
                    "quote": {{
                        "currency": "USDT",
                        "available": "10000.0",
                        "locked": "0.0",
                        "borrowed": "0.0",
                        "interest": "0.0"
                    }}
                }}"#,
                risk
            );

            let account: MarginAccount = serde_json::from_str(&json).unwrap();
            assert_eq!(account.risk, risk);
        }
    }

    #[test]
    fn test_margin_balance_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "available": "1.5",
            "locked": "0.5",
            "borrowed": "0.2",
            "interest": "0.001"
        }"#;

        let balance: MarginBalance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.currency, "BTC");
        assert_eq!(balance.available, "1.5");
        assert_eq!(balance.locked, "0.5");
        assert_eq!(balance.borrowed, "0.2");
        assert_eq!(balance.interest, "0.001");
    }

    #[test]
    fn test_margin_account_locked_status() {
        let json_locked = r#"{
            "currency_pair": "BTC_USDT",
            "locked": true,
            "risk": "0.95",
            "base": {
                "currency": "BTC",
                "available": "0.0",
                "locked": "1.0",
                "borrowed": "0.0",
                "interest": "0.0"
            },
            "quote": {
                "currency": "USDT",
                "available": "0.0",
                "locked": "0.0",
                "borrowed": "10000.0",
                "interest": "50.0"
            }
        }"#;

        let account: MarginAccount = serde_json::from_str(json_locked).unwrap();
        assert!(account.locked);
        assert_eq!(account.risk, "0.95");
    }

    #[test]
    fn test_margin_account_realistic_scenario() {
        // Scenario: User has borrowed USDT against BTC collateral
        let json = r#"{
            "currency_pair": "BTC_USDT",
            "locked": false,
            "risk": "0.4",
            "base": {
                "currency": "BTC",
                "available": "0.8",
                "locked": "0.2",
                "borrowed": "0.0",
                "interest": "0.0"
            },
            "quote": {
                "currency": "USDT",
                "available": "5000.0",
                "locked": "1000.0",
                "borrowed": "10000.0",
                "interest": "2.5"
            }
        }"#;

        let account: MarginAccount = serde_json::from_str(json).unwrap();

        // Verify BTC is used as collateral (not borrowed)
        assert_eq!(account.base.borrowed, "0.0");

        // Verify USDT is borrowed
        assert_eq!(account.quote.borrowed, "10000.0");

        // Verify there's interest on the borrowed amount
        assert_eq!(account.quote.interest, "2.5");

        // Verify risk level is moderate
        assert_eq!(account.risk, "0.4");
    }

    #[test]
    fn test_margin_balance_with_calculations() {
        let balance = MarginBalance {
            currency: "BTC".to_string(),
            available: "1.0".to_string(),
            locked: "0.5".to_string(),
            borrowed: "0.0".to_string(),
            interest: "0.0".to_string(),
        };

        // Verify balance calculations
        let available: f64 = balance.available.parse().unwrap();
        let locked: f64 = balance.locked.parse().unwrap();
        let total = available + locked;
        assert_eq!(total, 1.5);
    }

    #[test]
    fn test_margin_accounts_request_serialization_omits_null() {
        let request = MarginAccountsRequest {
            currency_pair: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();

        // Verify that None values are not serialized
        assert!(!obj.contains_key("currency_pair"));
    }

    #[test]
    fn test_margin_accounts_request_clone() {
        let original = MarginAccountsRequest {
            currency_pair: Some("BTC_USDT".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency_pair, original.currency_pair);
    }

    #[test]
    fn test_margin_account_clone() {
        let original = MarginAccount {
            currency_pair: "BTC_USDT".to_string(),
            locked: false,
            risk: "0.1".to_string(),
            base: MarginBalance {
                currency: "BTC".to_string(),
                available: "1.0".to_string(),
                locked: "0.0".to_string(),
                borrowed: "0.0".to_string(),
                interest: "0.0".to_string(),
            },
            quote: MarginBalance {
                currency: "USDT".to_string(),
                available: "10000.0".to_string(),
                locked: "0.0".to_string(),
                borrowed: "0.0".to_string(),
                interest: "0.0".to_string(),
            },
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.locked, original.locked);
        assert_eq!(cloned.risk, original.risk);
        assert_eq!(cloned.base.currency, original.base.currency);
        assert_eq!(cloned.quote.currency, original.quote.currency);
    }

    #[test]
    fn test_margin_balance_clone() {
        let original = MarginBalance {
            currency: "BTC".to_string(),
            available: "1.0".to_string(),
            locked: "0.5".to_string(),
            borrowed: "0.0".to_string(),
            interest: "0.0".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.available, original.available);
        assert_eq!(cloned.locked, original.locked);
        assert_eq!(cloned.borrowed, original.borrowed);
        assert_eq!(cloned.interest, original.interest);
    }

    #[test]
    fn test_margin_account_realistic_high_leverage_scenario() {
        let json = r#"{
            "currency_pair": "ETH_USDT",
            "locked": false,
            "risk": "0.75",
            "base": {
                "currency": "ETH",
                "available": "5.0",
                "locked": "0.0",
                "borrowed": "0.0",
                "interest": "0.0"
            },
            "quote": {
                "currency": "USDT",
                "available": "2000.0",
                "locked": "0.0",
                "borrowed": "30000.0",
                "interest": "15.0"
            }
        }"#;

        let account: MarginAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.risk, "0.75"); // High risk level
        assert_eq!(account.quote.borrowed, "30000.0"); // High borrowed amount
        assert!(!account.locked); // Not yet liquidated despite high risk
    }

    #[test]
    fn test_margin_account_serialization_deserialization_roundtrip() {
        let original = MarginAccount {
            currency_pair: "BTC_USDT".to_string(),
            locked: false,
            risk: "0.1".to_string(),
            base: MarginBalance {
                currency: "BTC".to_string(),
                available: "1.0".to_string(),
                locked: "0.0".to_string(),
                borrowed: "0.0".to_string(),
                interest: "0.0".to_string(),
            },
            quote: MarginBalance {
                currency: "USDT".to_string(),
                available: "10000.0".to_string(),
                locked: "0.0".to_string(),
                borrowed: "0.0".to_string(),
                interest: "0.0".to_string(),
            },
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: MarginAccount = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.currency_pair, original.currency_pair);
        assert_eq!(deserialized.locked, original.locked);
        assert_eq!(deserialized.risk, original.risk);
        assert_eq!(deserialized.base.available, original.base.available);
        assert_eq!(deserialized.quote.borrowed, original.quote.borrowed);
    }
}
