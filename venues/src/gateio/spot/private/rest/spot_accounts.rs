use serde::{Deserialize, Serialize};

use super::RestClient;

const SPOT_ACCOUNTS_ENDPOINT: &str = "/spot/accounts";

/// Request parameters for listing spot accounts
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListSpotAccountsRequest {
    /// Retrieve data of the specified currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Spot account balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotAccount {
    /// Currency name
    pub currency: String,
    /// Available balance
    pub available: String,
    /// Locked balance
    pub locked: String,
}

/// Implementation for the client
impl RestClient {
    /// List spot accounts
    ///
    /// This endpoint returns all spot account balances or a specific currency balance.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-spot-accounts>
    pub async fn list_spot_accounts(
        &self,
        currency: Option<&str>,
    ) -> crate::gateio::spot::RestResult<Vec<SpotAccount>> {
        let request = ListSpotAccountsRequest {
            currency: currency.map(|s| s.to_string()),
        };

        if currency.is_some() {
            self.get_with_query(SPOT_ACCOUNTS_ENDPOINT, &request).await
        } else {
            self.get(SPOT_ACCOUNTS_ENDPOINT).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_spot_accounts_request_no_currency() {
        let request = ListSpotAccountsRequest { currency: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_list_spot_accounts_request_with_currency() {
        let request = ListSpotAccountsRequest {
            currency: Some("BTC".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "currency=BTC");
    }

    #[test]
    fn test_list_spot_accounts_request_different_currencies() {
        let currencies = vec!["BTC", "ETH", "USDT", "USDC", "BNB", "SOL", "ADA", "DOT"];

        for currency in currencies {
            let request = ListSpotAccountsRequest {
                currency: Some(currency.to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("currency={}", currency));
        }
    }

    #[test]
    fn test_list_spot_accounts_request_default() {
        let request = ListSpotAccountsRequest::default();
        assert_eq!(request.currency, None);
    }

    #[test]
    fn test_spot_account_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "available": "1.234567",
            "locked": "0.001234"
        }"#;

        let account: SpotAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.currency, "BTC");
        assert_eq!(account.available, "1.234567");
        assert_eq!(account.locked, "0.001234");
    }

    #[test]
    fn test_spot_account_zero_balance() {
        let json = r#"{
            "currency": "ETH",
            "available": "0",
            "locked": "0"
        }"#;

        let account: SpotAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.currency, "ETH");
        assert_eq!(account.available, "0");
        assert_eq!(account.locked, "0");
    }

    #[test]
    fn test_spot_account_large_balance() {
        let json = r#"{
            "currency": "USDT",
            "available": "999999999.99999999",
            "locked": "123456789.12345678"
        }"#;

        let account: SpotAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.currency, "USDT");
        assert_eq!(account.available, "999999999.99999999");
        assert_eq!(account.locked, "123456789.12345678");
    }

    #[test]
    fn test_spot_account_small_balance() {
        let json = r#"{
            "currency": "SHIB",
            "available": "0.00000001",
            "locked": "0.00000001"
        }"#;

        let account: SpotAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.currency, "SHIB");
        assert_eq!(account.available, "0.00000001");
        assert_eq!(account.locked, "0.00000001");
    }

    #[test]
    fn test_spot_account_all_available() {
        let json = r#"{
            "currency": "BNB",
            "available": "100.5",
            "locked": "0"
        }"#;

        let account: SpotAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.currency, "BNB");
        assert_eq!(account.available, "100.5");
        assert_eq!(account.locked, "0");
    }

    #[test]
    fn test_spot_account_all_locked() {
        let json = r#"{
            "currency": "SOL",
            "available": "0",
            "locked": "50.75"
        }"#;

        let account: SpotAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.currency, "SOL");
        assert_eq!(account.available, "0");
        assert_eq!(account.locked, "50.75");
    }

    #[test]
    fn test_spot_account_array_deserialization() {
        let json = r#"[
            {
                "currency": "BTC",
                "available": "0.5",
                "locked": "0.1"
            },
            {
                "currency": "ETH",
                "available": "10.0",
                "locked": "2.0"
            },
            {
                "currency": "USDT",
                "available": "5000.0",
                "locked": "1000.0"
            }
        ]"#;

        let accounts: Vec<SpotAccount> = serde_json::from_str(json).unwrap();
        assert_eq!(accounts.len(), 3);

        assert_eq!(accounts[0].currency, "BTC");
        assert_eq!(accounts[0].available, "0.5");
        assert_eq!(accounts[0].locked, "0.1");

        assert_eq!(accounts[1].currency, "ETH");
        assert_eq!(accounts[1].available, "10.0");
        assert_eq!(accounts[1].locked, "2.0");

        assert_eq!(accounts[2].currency, "USDT");
        assert_eq!(accounts[2].available, "5000.0");
        assert_eq!(accounts[2].locked, "1000.0");
    }

    #[test]
    fn test_spot_account_empty_array() {
        let json = r#"[]"#;
        let accounts: Vec<SpotAccount> = serde_json::from_str(json).unwrap();
        assert_eq!(accounts.len(), 0);
    }

    #[test]
    fn test_spot_account_different_currencies() {
        let currencies = vec![
            "BTC", "ETH", "USDT", "USDC", "BNB", "SOL", "ADA", "DOT", "MATIC", "LINK",
        ];

        for currency in currencies {
            let json = format!(
                r#"{{
                "currency": "{}",
                "available": "100.0",
                "locked": "10.0"
            }}"#,
                currency
            );

            let account: SpotAccount = serde_json::from_str(&json).unwrap();
            assert_eq!(account.currency, currency);
        }
    }

    #[test]
    fn test_spot_account_serialization() {
        let account = SpotAccount {
            currency: "BTC".to_string(),
            available: "1.234567".to_string(),
            locked: "0.001234".to_string(),
        };

        let json = serde_json::to_value(&account).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["available"], "1.234567");
        assert_eq!(json["locked"], "0.001234");
    }

    #[test]
    fn test_spot_account_round_trip() {
        let original = SpotAccount {
            currency: "ETH".to_string(),
            available: "100.123456789".to_string(),
            locked: "25.987654321".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: SpotAccount = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.currency, original.currency);
        assert_eq!(deserialized.available, original.available);
        assert_eq!(deserialized.locked, original.locked);
    }

    #[test]
    fn test_spot_account_realistic_portfolio() {
        let json = r#"[
            {
                "currency": "BTC",
                "available": "0.12345678",
                "locked": "0.01"
            },
            {
                "currency": "ETH",
                "available": "2.5",
                "locked": "0.5"
            },
            {
                "currency": "USDT",
                "available": "10000.50",
                "locked": "2500.0"
            },
            {
                "currency": "BNB",
                "available": "5.25",
                "locked": "0"
            },
            {
                "currency": "SOL",
                "available": "0",
                "locked": "10.5"
            }
        ]"#;

        let accounts: Vec<SpotAccount> = serde_json::from_str(json).unwrap();
        assert_eq!(accounts.len(), 5);

        // Check BTC account - partial balance locked
        let btc = &accounts[0];
        assert_eq!(btc.currency, "BTC");
        let btc_available: f64 = btc.available.parse().unwrap();
        let btc_locked: f64 = btc.locked.parse().unwrap();
        assert!(btc_available > btc_locked);

        // Check USDT account - stablecoin with large balance
        let usdt = &accounts[2];
        assert_eq!(usdt.currency, "USDT");
        let usdt_total: f64 =
            usdt.available.parse::<f64>().unwrap() + usdt.locked.parse::<f64>().unwrap();
        assert!(usdt_total > 10000.0);

        // Check BNB account - all available
        let bnb = &accounts[3];
        assert_eq!(bnb.locked, "0");

        // Check SOL account - all locked (in open orders)
        let sol = &accounts[4];
        assert_eq!(sol.available, "0");
    }

    #[test]
    fn test_spot_account_precision_handling() {
        let json = r#"{
            "currency": "BTC",
            "available": "1.23456789012345",
            "locked": "0.98765432109876"
        }"#;

        let account: SpotAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.available, "1.23456789012345");
        assert_eq!(account.locked, "0.98765432109876");
    }

    #[test]
    fn test_spot_account_mixed_case_currency() {
        let json = r#"{
            "currency": "wBTC",
            "available": "0.5",
            "locked": "0.1"
        }"#;

        let account: SpotAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.currency, "wBTC");
    }

    #[test]
    fn test_spot_account_clone() {
        let original = SpotAccount {
            currency: "BTC".to_string(),
            available: "1.234567".to_string(),
            locked: "0.001234".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.available, original.available);
        assert_eq!(cloned.locked, original.locked);
    }

    #[test]
    fn test_spot_account_debug() {
        let account = SpotAccount {
            currency: "BTC".to_string(),
            available: "1.234567".to_string(),
            locked: "0.001234".to_string(),
        };

        let debug_str = format!("{:?}", account);
        assert!(debug_str.contains("SpotAccount"));
        assert!(debug_str.contains("BTC"));
        assert!(debug_str.contains("1.234567"));
        assert!(debug_str.contains("0.001234"));
    }
}
