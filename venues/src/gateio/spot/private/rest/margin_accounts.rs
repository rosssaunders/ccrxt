use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for margin accounts
#[derive(Debug, Clone, Serialize, Default)]
pub struct MarginAccountsRequest {
    /// Currency pair filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
}

/// Margin account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginAccount {
    /// Currency pair
    pub currency_pair: String,

    /// Locked status
    pub locked: bool,

    /// Risk level
    pub risk: String,

    /// Base currency information
    pub base: MarginBalance,

    /// Quote currency information
    pub quote: MarginBalance,
}

/// Margin balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginBalance {
    /// Currency
    pub currency: String,

    /// Available balance
    pub available: String,

    /// Locked balance
    pub locked: String,

    /// Borrowed amount
    pub borrowed: String,

    /// Interest amount
    pub interest: String,
}

impl RestClient {
    /// Get margin accounts
    ///
    /// This endpoint returns margin account information including balances,
    /// borrowed amounts, and risk levels for each currency pair.
    pub async fn get_margin_accounts(
        &self,
        params: MarginAccountsRequest,
    ) -> crate::gateio::spot::Result<Vec<MarginAccount>> {
        self.get_with_query("/margin/accounts", &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let pairs = vec![
            "BTC_USDT",
            "ETH_USDT",
            "LTC_BTC",
            "XRP_USDT",
            "DOT_USDT",
        ];

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