//! Integration test example for Deribit public/get_combo_ids endpoint
//! 
//! This test shows how users can use the new endpoint to fetch combo IDs.

#[cfg(test)]
mod usage_examples {
    use crate::deribit::{
        AccountTier, Currency, ComboState, GetComboIdsRequest, PublicRestClient, RateLimiter,
    };
    use reqwest::Client;

    #[test]
    fn test_get_combo_ids_request_usage() {
        // Test creating requests for all currencies
        let btc_request = GetComboIdsRequest {
            currency: Currency::BTC,
            state: None, // Get all combos regardless of state
        };

        let eth_active_request = GetComboIdsRequest {
            currency: Currency::ETH,
            state: Some(ComboState::Active), // Only active combos
        };

        let usdc_rfq_request = GetComboIdsRequest {
            currency: Currency::USDC,
            state: Some(ComboState::RFQ), // Only RFQ combos
        };

        // Verify serialization works correctly
        let btc_json = serde_json::to_value(&btc_request).unwrap();
        assert_eq!(btc_json["currency"], "BTC");
        assert!(!btc_json.as_object().unwrap().contains_key("state"));

        let eth_json = serde_json::to_value(&eth_active_request).unwrap();
        assert_eq!(eth_json["currency"], "ETH");
        assert_eq!(eth_json["state"], "active");

        let usdc_json = serde_json::to_value(&usdc_rfq_request).unwrap();
        assert_eq!(usdc_json["currency"], "USDC");
        assert_eq!(usdc_json["state"], "rfq");
    }

    #[test]
    fn test_client_creation_and_usage() {
        // Verify that the PublicRestClient can be created and has the expected methods
        let client = Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let rest_client = PublicRestClient::new(
            "https://www.deribit.com",
            client,
            rate_limiter,
        );

        // We can't make actual API calls in tests, but we can verify the method exists
        // and has the correct signature by attempting to create a future with test data
        let request = GetComboIdsRequest {
            currency: Currency::BTC,
            state: Some(ComboState::Active),
        };

        // This creates the future but doesn't execute it - just verifies the method signature
        let _future = rest_client.get_combo_ids(request);

        // If we reach this point, the method exists and has correct signatures
        assert!(true);
    }

    #[test]
    fn test_response_structure_parsing() {
        // Test parsing a realistic response structure
        let response_json = serde_json::json!({
            "id": 42,
            "jsonrpc": "2.0",
            "result": [
                "BTC-28JUN24-65000-C_BTC-28JUN24-70000-P",
                "BTC-28JUN24-60000-C_BTC-28JUN24-75000-P",
                "ETH-28JUN24-3000-C_ETH-28JUN24-3500-P"
            ]
        });

        let response: crate::deribit::GetComboIdsResponse = 
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 42);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.len(), 3);
        assert!(response.result[0].contains("BTC-28JUN24"));
        assert!(response.result[2].contains("ETH-28JUN24"));
    }

    #[tokio::test]
    async fn test_rate_limiting_for_combo_ids() {
        // Test that the rate limiting system correctly handles the new endpoint
        let rate_limiter = RateLimiter::new(AccountTier::Tier3);

        // The endpoint should use the PublicGetComboIds endpoint type,
        // which consumes 500 credits (same as other non-matching engine endpoints)
        let result = rate_limiter
            .check_limits(crate::deribit::EndpointType::PublicGetComboIds)
            .await;
        assert!(result.is_ok());

        // Record the request
        rate_limiter
            .record_request(crate::deribit::EndpointType::PublicGetComboIds)
            .await;

        // Check that the credit system is working
        let status = rate_limiter.get_status().await;
        // After one request, we should have consumed 500 credits from the default 50,000
        assert_eq!(status.available_credits, 50_000 - 500);
    }

    #[test]
    fn test_all_supported_currencies_and_states() {
        // Verify all combinations work
        let currencies = [
            Currency::BTC,
            Currency::ETH,
            Currency::USDC,
            Currency::USDT,
            Currency::EURR,
        ];

        let states = [
            Some(ComboState::RFQ),
            Some(ComboState::Active),
            Some(ComboState::Inactive),
            None, // All states
        ];

        for currency in currencies {
            for state in &states {
                let request = GetComboIdsRequest {
                    currency: currency.clone(),
                    state: state.clone(),
                };

                // Verify serialization works for all combinations
                let json = serde_json::to_value(&request).unwrap();
                assert!(json["currency"].is_string());
                
                if state.is_some() {
                    assert!(json["state"].is_string());
                } else {
                    assert!(!json.as_object().unwrap().contains_key("state"));
                }
            }
        }
    }
}