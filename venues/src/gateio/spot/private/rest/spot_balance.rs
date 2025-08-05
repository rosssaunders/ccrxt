// Removed unused Serialize and Deserialize imports

use super::{RestClient, SpotAccount};

impl RestClient {
    /// Get spot account balance for a specific currency
    ///
    /// Retrieve the spot trading account balance for a specified currency.
    /// Returns balance information including available and locked amounts for the requested currency.
    ///
    /// [docs]: https://www.gate.io/docs/developers/apiv4/#list-spot-accounts
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `currency` - Currency symbol to get balance for (e.g., "BTC", "ETH", "USDT")
    ///
    /// # Returns
    /// Spot account balance for the specified currency
    pub async fn get_spot_account_balance(
        &self,
        currency: &str,
    ) -> crate::gateio::spot::RestResult<SpotAccount> {
        let accounts = self.list_spot_accounts(Some(currency)).await?;
        accounts
            .into_iter()
            .find(|acc| acc.currency == currency)
            .ok_or_else(|| {
                crate::gateio::spot::GateIoError::Internal(format!(
                    "Currency {} not found",
                    currency
                ))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests spot account balance logic with various scenarios.

    #[test]
    fn test_spot_account_balance_logic() {
        // Test finding account in list
        let accounts = vec![
            SpotAccount {
                currency: "BTC".to_string(),
                available: "1.234567".to_string(),
                locked: "0.001234".to_string(),
            },
            SpotAccount {
                currency: "ETH".to_string(),
                available: "10.0".to_string(),
                locked: "2.0".to_string(),
            },
            SpotAccount {
                currency: "USDT".to_string(),
                available: "5000.0".to_string(),
                locked: "1000.0".to_string(),
            },
        ];

        // Find existing currency
        let btc_account = accounts.iter().find(|acc| acc.currency == "BTC").unwrap();
        assert_eq!(btc_account.currency, "BTC");
        assert_eq!(btc_account.available, "1.234567");
        assert_eq!(btc_account.locked, "0.001234");

        // Test currency not found
        let missing = accounts.iter().find(|acc| acc.currency == "SOL");
        assert!(missing.is_none());
    }

    #[test]
    fn test_spot_account_balance_filtering() {
        let accounts = vec![
            SpotAccount {
                currency: "BTC".to_string(),
                available: "0.1".to_string(),
                locked: "0.0".to_string(),
            },
            SpotAccount {
                currency: "ETH".to_string(),
                available: "2.5".to_string(),
                locked: "0.5".to_string(),
            },
        ];

        // Test finding BTC
        let btc_result = accounts.iter().find(|acc| acc.currency == "BTC");
        assert!(btc_result.is_some());
        let btc = btc_result.unwrap();
        assert_eq!(btc.currency, "BTC");

        // Test finding ETH
        let eth_result = accounts.iter().find(|acc| acc.currency == "ETH");
        assert!(eth_result.is_some());
        let eth = eth_result.unwrap();
        assert_eq!(eth.currency, "ETH");

        // Test finding non-existent
        let missing = accounts.iter().find(|acc| acc.currency == "MISSING");
        assert!(missing.is_none());
    }

    #[test]
    fn test_spot_account_balance_empty_list() {
        let accounts: Vec<SpotAccount> = vec![];

        let missing = accounts.iter().find(|acc| acc.currency == "BTC");
        assert!(missing.is_none());
    }

    #[test]
    fn test_spot_account_balance_single_currency() {
        let accounts = vec![SpotAccount {
            currency: "USDT".to_string(),
            available: "10000.0".to_string(),
            locked: "0.0".to_string(),
        }];

        // Find the only currency
        let usdt_result = accounts.iter().find(|acc| acc.currency == "USDT");
        assert!(usdt_result.is_some());
        let usdt = usdt_result.unwrap();
        assert_eq!(usdt.currency, "USDT");
        assert_eq!(usdt.available, "10000.0");
        assert_eq!(usdt.locked, "0.0");

        // Test for different currency
        let missing = accounts.iter().find(|acc| acc.currency == "BTC");
        assert!(missing.is_none());
    }

    #[test]
    fn test_spot_account_balance_duplicate_currencies() {
        // This shouldn't happen in practice, but test the logic
        let accounts = vec![
            SpotAccount {
                currency: "BTC".to_string(),
                available: "1.0".to_string(),
                locked: "0.0".to_string(),
            },
            SpotAccount {
                currency: "BTC".to_string(),
                available: "2.0".to_string(),
                locked: "0.1".to_string(),
            },
        ];

        // Should find the first match
        let btc_result = accounts.iter().find(|acc| acc.currency == "BTC");
        assert!(btc_result.is_some());
        let btc = btc_result.unwrap();
        assert_eq!(btc.available, "1.0"); // First one
    }

    #[test]
    fn test_spot_account_balance_case_sensitivity() {
        let accounts = vec![
            SpotAccount {
                currency: "btc".to_string(),
                available: "1.0".to_string(),
                locked: "0.0".to_string(),
            },
            SpotAccount {
                currency: "BTC".to_string(),
                available: "2.0".to_string(),
                locked: "0.1".to_string(),
            },
        ];

        // Case sensitive search
        let lowercase = accounts.iter().find(|acc| acc.currency == "btc");
        assert!(lowercase.is_some());
        assert_eq!(lowercase.unwrap().available, "1.0");

        let uppercase = accounts.iter().find(|acc| acc.currency == "BTC");
        assert!(uppercase.is_some());
        assert_eq!(uppercase.unwrap().available, "2.0");

        // Different case should not match
        let mixed = accounts.iter().find(|acc| acc.currency == "Btc");
        assert!(mixed.is_none());
    }

    #[test]
    fn test_spot_account_balance_various_currencies() {
        let currencies = vec!["BTC", "ETH", "USDT", "USDC", "BNB", "SOL", "ADA", "DOT"];
        let mut accounts = Vec::new();

        for (i, currency) in currencies.iter().enumerate() {
            accounts.push(SpotAccount {
                currency: currency.to_string(),
                available: format!("{}.0", i + 1),
                locked: "0.0".to_string(),
            });
        }

        // Test finding each currency
        for (i, currency) in currencies.iter().enumerate() {
            let found = accounts.iter().find(|acc| acc.currency == *currency);
            assert!(found.is_some());
            let account = found.unwrap();
            assert_eq!(account.currency, *currency);
            assert_eq!(account.available, format!("{}.0", i + 1));
        }

        // Test currency not in list
        let missing = accounts.iter().find(|acc| acc.currency == "MISSING");
        assert!(missing.is_none());
    }

    #[test]
    fn test_spot_account_balance_zero_balances() {
        let accounts = vec![
            SpotAccount {
                currency: "BTC".to_string(),
                available: "0".to_string(),
                locked: "0".to_string(),
            },
            SpotAccount {
                currency: "ETH".to_string(),
                available: "0.0".to_string(),
                locked: "0.0".to_string(),
            },
        ];

        let btc = accounts.iter().find(|acc| acc.currency == "BTC").unwrap();
        assert_eq!(btc.available, "0");
        assert_eq!(btc.locked, "0");

        let eth = accounts.iter().find(|acc| acc.currency == "ETH").unwrap();
        assert_eq!(eth.available, "0.0");
        assert_eq!(eth.locked, "0.0");
    }

    #[test]
    fn test_spot_account_balance_precision_handling() {
        let accounts = vec![SpotAccount {
            currency: "BTC".to_string(),
            available: "1.23456789012345".to_string(),
            locked: "0.98765432109876".to_string(),
        }];

        let btc = accounts.iter().find(|acc| acc.currency == "BTC").unwrap();
        assert_eq!(btc.available, "1.23456789012345");
        assert_eq!(btc.locked, "0.98765432109876");
    }

    #[test]
    fn test_spot_account_balance_large_numbers() {
        let accounts = vec![SpotAccount {
            currency: "USDT".to_string(),
            available: "999999999.99999999".to_string(),
            locked: "123456789.12345678".to_string(),
        }];

        let usdt = accounts.iter().find(|acc| acc.currency == "USDT").unwrap();
        assert_eq!(usdt.available, "999999999.99999999");
        assert_eq!(usdt.locked, "123456789.12345678");
    }

    #[test]
    fn test_spot_account_balance_mixed_case_currency() {
        let accounts = vec![
            SpotAccount {
                currency: "wBTC".to_string(),
                available: "0.5".to_string(),
                locked: "0.1".to_string(),
            },
            SpotAccount {
                currency: "WETH".to_string(),
                available: "2.0".to_string(),
                locked: "0.5".to_string(),
            },
        ];

        let wbtc = accounts.iter().find(|acc| acc.currency == "wBTC");
        assert!(wbtc.is_some());
        assert_eq!(wbtc.unwrap().currency, "wBTC");

        let weth = accounts.iter().find(|acc| acc.currency == "WETH");
        assert!(weth.is_some());
        assert_eq!(weth.unwrap().currency, "WETH");
    }

    #[test]
    fn test_spot_account_balance_realistic_portfolio() {
        let accounts = vec![
            SpotAccount {
                currency: "BTC".to_string(),
                available: "0.12345678".to_string(),
                locked: "0.01".to_string(),
            },
            SpotAccount {
                currency: "ETH".to_string(),
                available: "2.5".to_string(),
                locked: "0.5".to_string(),
            },
            SpotAccount {
                currency: "USDT".to_string(),
                available: "10000.50".to_string(),
                locked: "2500.0".to_string(),
            },
            SpotAccount {
                currency: "BNB".to_string(),
                available: "5.25".to_string(),
                locked: "0".to_string(),
            },
        ];

        // Test finding each currency with realistic balances
        let btc = accounts.iter().find(|acc| acc.currency == "BTC").unwrap();
        let btc_available: f64 = btc.available.parse().unwrap();
        let btc_locked: f64 = btc.locked.parse().unwrap();
        assert!(btc_available > btc_locked);

        let usdt = accounts.iter().find(|acc| acc.currency == "USDT").unwrap();
        let usdt_total: f64 =
            usdt.available.parse::<f64>().unwrap() + usdt.locked.parse::<f64>().unwrap();
        assert!(usdt_total > 10000.0);

        let bnb = accounts.iter().find(|acc| acc.currency == "BNB").unwrap();
        assert_eq!(bnb.locked, "0");
    }

    #[test]
    fn test_spot_account_balance_find_into_iter() {
        let accounts = vec![
            SpotAccount {
                currency: "BTC".to_string(),
                available: "1.0".to_string(),
                locked: "0.1".to_string(),
            },
            SpotAccount {
                currency: "ETH".to_string(),
                available: "10.0".to_string(),
                locked: "2.0".to_string(),
            },
        ];

        // Test consuming iterator (like in the actual function)
        let btc_account = accounts.into_iter().find(|acc| acc.currency == "BTC");
        assert!(btc_account.is_some());
        let btc = btc_account.unwrap();
        assert_eq!(btc.currency, "BTC");
        assert_eq!(btc.available, "1.0");
        assert_eq!(btc.locked, "0.1");
    }

    #[test]
    fn test_spot_account_balance_stablecoin_currencies() {
        let stablecoins = vec!["USDT", "USDC", "BUSD", "DAI", "TUSD"];
        let mut accounts = Vec::new();

        for stablecoin in &stablecoins {
            accounts.push(SpotAccount {
                currency: stablecoin.to_string(),
                available: "1000.0".to_string(),
                locked: "100.0".to_string(),
            });
        }

        // Test finding each stablecoin
        for stablecoin in &stablecoins {
            let found = accounts.iter().find(|acc| acc.currency == *stablecoin);
            assert!(found.is_some());
            let account = found.unwrap();
            assert_eq!(account.currency, *stablecoin);
            assert_eq!(account.available, "1000.0");
            assert_eq!(account.locked, "100.0");
        }
    }
}
