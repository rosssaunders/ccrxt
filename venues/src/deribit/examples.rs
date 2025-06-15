//! Integration example demonstrating Deribit public API usage
//!
//! This example shows how to use the Deribit public REST client
//! to call the get_combos endpoint.

use crate::deribit::{
    AccountTier, Currency, GetCombosRequest, PublicRestClient, RateLimiter,
};

/// Example demonstrating Deribit public API usage
#[cfg(test)]
mod example {
    use super::*;

    #[test]
    fn example_deribit_public_api_usage() {
        // Note: This is just an example of method calls - it won't make actual HTTP requests in tests

        // Example usage patterns (would be used in real applications):

        // 1. Create a rate limiter for a Tier 4 account (up to 1M USD trading volume)
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        // 2. Create a public REST client
        let client = PublicRestClient::new(
            "https://www.deribit.com/api/v2",
            reqwest::Client::new(),
            rate_limiter,
        );

        // 3. Create request for BTC combos
        let btc_request = GetCombosRequest {
            currency: Currency::Btc,
        };

        // 4. Create request for all currencies
        let any_request = GetCombosRequest {
            currency: Currency::Any,
        };

        // In real usage, you would call these with await:
        // let btc_combos = client.get_combos(btc_request).await?;
        // let all_combos = client.get_combos(any_request).await?;

        // Example of processing the response:
        // for combo in btc_combos.result {
        //     println!("Combo ID: {}", combo.id);
        //     println!("State: {:?}", combo.state);
        //     println!("Creation time: {}", combo.creation_timestamp);
        //     println!("Legs:");
        //     for leg in combo.legs {
        //         println!("  - {} amount: {}", leg.instrument_name, leg.amount);
        //     }
        // }

        // Demonstrate that the structures are properly created
        assert_eq!(client.base_url, "https://www.deribit.com/api/v2");
        assert_eq!(btc_request.currency, Currency::Btc);
        assert_eq!(any_request.currency, Currency::Any);
    }

    #[test]
    fn example_currency_usage() {
        // Example of all supported currencies
        let currencies = vec![
            Currency::Btc,
            Currency::Eth,
            Currency::Usdc,
            Currency::Usdt,
            Currency::Eurr,
            Currency::Any,
        ];

        for currency in currencies {
            let request = GetCombosRequest { currency };
            
            // Each currency can be serialized properly
            let serialized = serde_json::to_value(&request).unwrap();
            assert!(serialized.get("currency").is_some());
            
            // In real usage, you would call the endpoint:
            // let combos = client.get_combos(request).await?;
        }
    }

    #[test]
    fn example_rate_limiting() {
        // Example of different account tiers
        let tiers = vec![
            (AccountTier::Tier1, 30, 100), // 30 req/sec, 100 burst
            (AccountTier::Tier2, 20, 50),  // 20 req/sec, 50 burst
            (AccountTier::Tier3, 10, 30),  // 10 req/sec, 30 burst
            (AccountTier::Tier4, 5, 20),   // 5 req/sec, 20 burst
        ];

        for (tier, sustained_rate, burst_limit) in tiers {
            let _rate_limiter = RateLimiter::new(tier);
            assert_eq!(tier.sustained_rate(), sustained_rate);
            assert_eq!(tier.burst_limit(), burst_limit);

            // In real usage, rate limiting would be checked automatically:
            // rate_limiter.check_limits(EndpointType::PublicGetCombos).await?;
        }
    }
}