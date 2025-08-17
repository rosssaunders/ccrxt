use super::{RestClient, SpotAccount};

impl RestClient {
    /// List spot accounts (showing only non-zero balances)
    ///
    /// Retrieve all spot account balances, filtering to only show accounts with non-zero available or locked amounts.
    /// This endpoint provides a convenient way to view active balances without zero-balance currencies.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/#list-spot-accounts)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Returns
    /// Vector of spot accounts with non-zero available or locked balances
    pub async fn get_non_zero_spot_balances(
        &self,
    ) -> crate::gateio::spot::RestResult<Vec<SpotAccount>> {
        let accounts = self.list_spot_accounts(None).await?;
        Ok(accounts
            .into_iter()
            .filter(|acc| {
                let available: f64 = acc.available.trim().parse().unwrap_or(0.0);
                let locked: f64 = acc.locked.trim().parse().unwrap_or(0.0);
                available > 0.0 || locked > 0.0
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a test spot account for testing.
    fn create_spot_account(currency: &str, available: &str, locked: &str) -> SpotAccount {
        SpotAccount {
            currency: currency.to_string(),
            available: available.to_string(),
            locked: locked.to_string(),
        }
    }

    /// Filters accounts to only include those with non-zero balances.
    fn filter_non_zero_accounts(accounts: Vec<SpotAccount>) -> Vec<SpotAccount> {
        accounts
            .into_iter()
            .filter(|acc| {
                let available: f64 = acc.available.trim().parse().unwrap_or(0.0);
                let locked: f64 = acc.locked.trim().parse().unwrap_or(0.0);
                available > 0.0 || locked > 0.0
            })
            .collect()
    }

    #[test]
    fn test_filter_non_zero_balances_all_zero() {
        let accounts = vec![
            create_spot_account("BTC", "0", "0"),
            create_spot_account("ETH", "0.0", "0.0"),
            create_spot_account("USDT", "0.00", "0.00"),
            create_spot_account("BNB", "0.000", "0.000"),
        ];

        let filtered = filter_non_zero_accounts(accounts);
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_filter_non_zero_balances_some_with_available() {
        let accounts = vec![
            create_spot_account("BTC", "0.1", "0"),
            create_spot_account("ETH", "0", "0"),
            create_spot_account("USDT", "1000.5", "0"),
            create_spot_account("BNB", "0", "0"),
        ];

        let filtered = filter_non_zero_accounts(accounts);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].currency, "BTC");
        assert_eq!(filtered[1].currency, "USDT");
    }

    #[test]
    fn test_filter_non_zero_balances_some_with_locked() {
        let accounts = vec![
            create_spot_account("BTC", "0", "0.05"),
            create_spot_account("ETH", "0", "0"),
            create_spot_account("USDT", "0", "500.0"),
            create_spot_account("BNB", "0", "0"),
        ];

        let filtered = filter_non_zero_accounts(accounts);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].currency, "BTC");
        assert_eq!(filtered[1].currency, "USDT");
    }

    #[test]
    fn test_filter_non_zero_balances_mixed_available_and_locked() {
        let accounts = vec![
            create_spot_account("BTC", "0.1", "0.05"),  // Both non-zero
            create_spot_account("ETH", "5.0", "0"),     // Available only
            create_spot_account("USDT", "0", "1000.0"), // Locked only
            create_spot_account("BNB", "0", "0"),       // Both zero
            create_spot_account("SOL", "100.0", "50.0"), // Both non-zero
        ];

        let filtered = filter_non_zero_accounts(accounts);
        assert_eq!(filtered.len(), 4);

        let currencies: Vec<&str> = filtered.iter().map(|acc| acc.currency.as_str()).collect();
        assert!(currencies.contains(&"BTC"));
        assert!(currencies.contains(&"ETH"));
        assert!(currencies.contains(&"USDT"));
        assert!(currencies.contains(&"SOL"));
        assert!(!currencies.contains(&"BNB"));
    }

    #[test]
    fn test_filter_non_zero_balances_all_non_zero() {
        let accounts = vec![
            create_spot_account("BTC", "0.1", "0.05"),
            create_spot_account("ETH", "5.0", "2.0"),
            create_spot_account("USDT", "1000.0", "500.0"),
            create_spot_account("BNB", "100.0", "25.0"),
        ];

        let filtered = filter_non_zero_accounts(accounts.clone());
        assert_eq!(filtered.len(), accounts.len());

        for (original, filtered) in accounts.iter().zip(filtered.iter()) {
            assert_eq!(original.currency, filtered.currency);
        }
    }

    #[test]
    fn test_filter_non_zero_balances_empty_list() {
        let accounts: Vec<SpotAccount> = vec![];
        let filtered = filter_non_zero_accounts(accounts);
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_filter_non_zero_balances_very_small_amounts() {
        let accounts = vec![
            create_spot_account("BTC", "0.00000001", "0"), // Smallest BTC unit
            create_spot_account("ETH", "0", "0.000000000000000001"), // Very small locked
            create_spot_account("USDT", "0.000001", "0"),  // Micro USDT
            create_spot_account("BNB", "0", "0"),          // Zero
        ];

        let filtered = filter_non_zero_accounts(accounts);
        assert_eq!(filtered.len(), 3);

        let currencies: Vec<&str> = filtered.iter().map(|acc| acc.currency.as_str()).collect();
        assert!(currencies.contains(&"BTC"));
        assert!(currencies.contains(&"ETH"));
        assert!(currencies.contains(&"USDT"));
        assert!(!currencies.contains(&"BNB"));
    }

    #[test]
    fn test_filter_non_zero_balances_large_amounts() {
        let accounts = vec![
            create_spot_account("BTC", "1000.12345678", "500.87654321"),
            create_spot_account("ETH", "50000.123456789", "0"),
            create_spot_account("USDT", "0", "1000000.999999"),
            create_spot_account("BNB", "0", "0"),
        ];

        let filtered = filter_non_zero_accounts(accounts);
        assert_eq!(filtered.len(), 3);

        // Verify amounts are preserved
        assert_eq!(filtered[0].available, "1000.12345678");
        assert_eq!(filtered[0].locked, "500.87654321");
        assert_eq!(filtered[1].available, "50000.123456789");
        assert_eq!(filtered[2].locked, "1000000.999999");
    }

    #[test]
    fn test_filter_non_zero_balances_scientific_notation() {
        let accounts = vec![
            create_spot_account("BTC", "1e-8", "0"), // Scientific notation
            create_spot_account("ETH", "1.5e-6", "0"), // Scientific notation
            create_spot_account("USDT", "0", "1e6"), // Large scientific notation
            create_spot_account("BNB", "0", "0"),    // Zero
        ];

        let filtered = filter_non_zero_accounts(accounts);
        assert_eq!(filtered.len(), 3);

        let currencies: Vec<&str> = filtered.iter().map(|acc| acc.currency.as_str()).collect();
        assert!(currencies.contains(&"BTC"));
        assert!(currencies.contains(&"ETH"));
        assert!(currencies.contains(&"USDT"));
    }

    #[test]
    fn test_filter_non_zero_balances_negative_amounts() {
        // While negative amounts shouldn't normally occur, test the filter behavior
        let accounts = vec![
            create_spot_account("BTC", "-0.1", "0"), // Negative available
            create_spot_account("ETH", "0", "-0.05"), // Negative locked
            create_spot_account("USDT", "-100", "-50"), // Both negative
            create_spot_account("BNB", "0", "0"),    // Zero
        ];

        let filtered = filter_non_zero_accounts(accounts);
        // All negative amounts should be filtered out as they're < 0.0
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_filter_non_zero_balances_invalid_number_strings() {
        let accounts = vec![
            create_spot_account("BTC", "invalid", "0"), // Invalid available
            create_spot_account("ETH", "0", "not_a_number"), // Invalid locked
            create_spot_account("USDT", "abc", "def"),  // Both invalid
            create_spot_account("BNB", "1.0", "0.5"),   // Valid
        ];

        let filtered = filter_non_zero_accounts(accounts);
        // Only BNB should pass since it has valid numbers
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].currency, "BNB");
    }

    #[test]
    fn test_filter_non_zero_balances_edge_case_whitespace() {
        let accounts = vec![
            create_spot_account("BTC", " 0.1 ", "0"), // Whitespace around number
            create_spot_account("ETH", "0", " 0.05 "), // Whitespace around number
            create_spot_account("USDT", " ", "0"),    // Just whitespace
            create_spot_account("BNB", "", ""),       // Empty strings
        ];

        let filtered = filter_non_zero_accounts(accounts);
        // Whitespace around numbers are trimmed and parse correctly
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].currency, "BTC");
        assert_eq!(filtered[1].currency, "ETH");
    }

    #[test]
    fn test_filter_non_zero_balances_realistic_portfolio_scenario() {
        // Simulate a realistic trading portfolio
        let accounts = vec![
            create_spot_account("BTC", "0.12345678", "0"), // Active BTC position
            create_spot_account("ETH", "2.5", "0.5"),      // ETH with some locked
            create_spot_account("USDT", "1500.50", "0"),   // USDT for trading
            create_spot_account("BNB", "0", "10.0"),       // BNB locked for fees
            create_spot_account("ADA", "0", "0"),          // No ADA holdings
            create_spot_account("SOL", "5.0", "0"),        // SOL position
            create_spot_account("DOT", "0", "0"),          // No DOT holdings
            create_spot_account("MATIC", "0", "0"),        // No MATIC holdings
            create_spot_account("LINK", "1.25", "0.75"),   // LINK with both
            create_spot_account("UNI", "0", "0"),          // No UNI holdings
        ];

        let filtered = filter_non_zero_accounts(accounts);
        assert_eq!(filtered.len(), 6); // BTC, ETH, USDT, BNB, SOL, LINK

        let currencies: Vec<&str> = filtered.iter().map(|acc| acc.currency.as_str()).collect();
        assert!(currencies.contains(&"BTC"));
        assert!(currencies.contains(&"ETH"));
        assert!(currencies.contains(&"USDT"));
        assert!(currencies.contains(&"BNB"));
        assert!(currencies.contains(&"SOL"));
        assert!(currencies.contains(&"LINK"));
        assert!(!currencies.contains(&"ADA"));
        assert!(!currencies.contains(&"DOT"));
        assert!(!currencies.contains(&"MATIC"));
        assert!(!currencies.contains(&"UNI"));
    }

    #[test]
    fn test_filter_non_zero_balances_dust_filtering_scenario() {
        // Test filtering out dust amounts vs keeping them
        let accounts = vec![
            create_spot_account("BTC", "0.00000001", "0"), // 1 satoshi
            create_spot_account("ETH", "0.000000000000000001", "0"), // 1 wei
            create_spot_account("USDT", "0.000001", "0"),  // 1 micro USDT
            create_spot_account("BNB", "0.00000000", "0"), // Zero
            create_spot_account("SOL", "0.000000001", "0"), // 1 nano SOL
        ];

        let filtered = filter_non_zero_accounts(accounts);
        assert_eq!(filtered.len(), 4); // All except BNB should be included

        // Verify that even tiny amounts are preserved
        let btc_account = filtered.iter().find(|acc| acc.currency == "BTC").unwrap();
        assert_eq!(btc_account.available, "0.00000001");

        let eth_account = filtered.iter().find(|acc| acc.currency == "ETH").unwrap();
        assert_eq!(eth_account.available, "0.000000000000000001");
    }

    #[test]
    fn test_filter_non_zero_balances_stablecoin_scenario() {
        // Test with various stablecoins
        let accounts = vec![
            create_spot_account("USDT", "1000.50", "0"), // Tether
            create_spot_account("USDC", "0", "500.25"),  // USD Coin
            create_spot_account("BUSD", "0", "0"),       // Binance USD (deprecated)
            create_spot_account("DAI", "250.75", "0"),   // Dai
            create_spot_account("TUSD", "0", "0"),       // TrueUSD
            create_spot_account("USDP", "100.0", "0"),   // Pax Dollar
        ];

        let filtered = filter_non_zero_accounts(accounts);
        assert_eq!(filtered.len(), 4); // USDT, USDC, DAI, USDP

        let currencies: Vec<&str> = filtered.iter().map(|acc| acc.currency.as_str()).collect();
        assert!(currencies.contains(&"USDT"));
        assert!(currencies.contains(&"USDC"));
        assert!(currencies.contains(&"DAI"));
        assert!(currencies.contains(&"USDP"));
        assert!(!currencies.contains(&"BUSD"));
        assert!(!currencies.contains(&"TUSD"));
    }

    #[test]
    fn test_filter_non_zero_balances_order_preservation() {
        // Test that the order of accounts is preserved
        let accounts = vec![
            create_spot_account("ZEC", "1.0", "0"), // Last alphabetically
            create_spot_account("BTC", "0", "0"),   // Should be filtered
            create_spot_account("ADA", "2.0", "0"), // First alphabetically
            create_spot_account("ETH", "0", "0"),   // Should be filtered
            create_spot_account("SOL", "3.0", "0"), // Middle
        ];

        let filtered = filter_non_zero_accounts(accounts);
        assert_eq!(filtered.len(), 3);

        // Order should be preserved: ZEC, ADA, SOL
        assert_eq!(filtered[0].currency, "ZEC");
        assert_eq!(filtered[1].currency, "ADA");
        assert_eq!(filtered[2].currency, "SOL");
    }

    #[test]
    fn test_filter_non_zero_balances_clone_behavior() {
        let account = create_spot_account("BTC", "1.0", "0.5");
        let accounts = vec![account.clone()];

        let filtered = filter_non_zero_accounts(accounts);
        assert_eq!(filtered.len(), 1);

        // Verify the account data is correctly copied
        assert_eq!(filtered[0].currency, account.currency);
        assert_eq!(filtered[0].available, account.available);
        assert_eq!(filtered[0].locked, account.locked);
    }

    #[test]
    fn test_filter_non_zero_balances_precision_edge_cases() {
        let accounts = vec![
            create_spot_account("BTC", "0.123456789012345", "0"), // High precision
            create_spot_account("ETH", "0", "0.987654321098765"), // High precision locked
            create_spot_account("USDT", "1.0000000000000001", "0"), // Floating point precision
            create_spot_account("BNB", "0.9999999999999999", "0"), // Close to 1.0
            create_spot_account("SOL", "0.0000000000000001", "0"), // Very small precision
        ];

        let filtered = filter_non_zero_accounts(accounts);
        assert_eq!(filtered.len(), 5); // All should be included as they're > 0

        // Verify precision is maintained
        assert_eq!(filtered[0].available, "0.123456789012345");
        assert_eq!(filtered[1].locked, "0.987654321098765");
        assert_eq!(filtered[2].available, "1.0000000000000001");
        assert_eq!(filtered[3].available, "0.9999999999999999");
        assert_eq!(filtered[4].available, "0.0000000000000001");
    }

    #[test]
    fn test_spot_account_debug() {
        let account = create_spot_account("BTC", "1.0", "0.5");
        let debug_str = format!("{:?}", account);
        assert!(debug_str.contains("SpotAccount"));
        assert!(debug_str.contains("BTC"));
        assert!(debug_str.contains("1.0"));
        assert!(debug_str.contains("0.5"));
    }

    #[test]
    fn test_spot_account_clone() {
        let original = create_spot_account("BTC", "1.0", "0.5");
        let cloned = original.clone();

        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.available, original.available);
        assert_eq!(cloned.locked, original.locked);
    }
}
