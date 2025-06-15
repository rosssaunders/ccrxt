//! Integration test example for Deribit public/get_combo_ids endpoint
//! 
//! This test shows how users can use the new endpoint to fetch combo IDs.
//! 
//! Integration tests for Deribit have been moved to `tests/deribit.rs` as per project testing instructions.
//! This file is now deprecated and should not contain integration tests.

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

    #[test]
    fn test_set_clearance_originator_usage_example() {
        // Test creating a set clearance originator request
        use crate::deribit::{DepositId, Originator, SetClearanceOriginatorRequest};

        let deposit_id = DepositId {
            currency: "BTC".to_string(),
            user_id: 12345,
            address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            tx_hash: "1a2b3c4d5e6f7890abcdef1234567890abcdef1234567890abcdef1234567890".to_string(),
        };

        // Example: Personal originator
        let personal_originator = Originator {
            is_personal: true,
            company_name: "".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            address: "123 Main St, New York, NY 10001".to_string(),
        };

        let personal_request = SetClearanceOriginatorRequest {
            deposit_id: deposit_id.clone(),
            originator: personal_originator,
        };

        // Example: Corporate originator
        let corporate_originator = Originator {
            is_personal: false,
            company_name: "Acme Corp".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            address: "456 Business Ave, San Francisco, CA 94105".to_string(),
        };

        let corporate_request = SetClearanceOriginatorRequest {
            deposit_id,
            originator: corporate_originator,
        };

        // Verify serialization works correctly for both types
        let personal_json = serde_json::to_value(&personal_request).unwrap();
        assert_eq!(personal_json["originator"]["is_personal"], true);
        assert_eq!(personal_json["originator"]["first_name"], "John");
        assert_eq!(personal_json["originator"]["last_name"], "Doe");

        let corporate_json = serde_json::to_value(&corporate_request).unwrap();
        assert_eq!(corporate_json["originator"]["is_personal"], false);
        assert_eq!(corporate_json["originator"]["company_name"], "Acme Corp");
    }

    #[test]
    fn test_withdraw_endpoint_integration() {
        // Test that all withdraw-related types are accessible and properly structured
        use crate::deribit::{
            Currency, WithdrawalPriority, WithdrawalState,
            WithdrawRequest, WithdrawResponse, WithdrawalData
        };

        // Test creating a withdraw request
        let request = WithdrawRequest {
            currency: Currency::BTC,
            address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            amount: 0.001,
            priority: Some(WithdrawalPriority::High),
        };

        // Test enum values and serialization
        assert_eq!(WithdrawalPriority::default(), WithdrawalPriority::High);
        assert_eq!(format!("{}", WithdrawalPriority::VeryHigh), "very_high");
        assert_eq!(format!("{}", WithdrawalState::Unconfirmed), "unconfirmed");
        
        // Test request serialization
        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value["currency"], "BTC");
        assert_eq!(json_value["priority"], "high");
        assert_eq!(json_value["amount"], 0.001);

        // Test request without priority (should not include priority field)
        let minimal_request = WithdrawRequest {
            currency: Currency::ETH,
            address: "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c".to_string(),
            amount: 0.5,
            priority: None,
        };

        let minimal_json = serde_json::to_value(&minimal_request).unwrap();
        assert_eq!(minimal_json["currency"], "ETH");
        assert!(!minimal_json.as_object().unwrap().contains_key("priority"));

        // Test all withdrawal priorities
        let priorities = [
            WithdrawalPriority::Insane,
            WithdrawalPriority::ExtremeHigh,
            WithdrawalPriority::VeryHigh,
            WithdrawalPriority::High,
            WithdrawalPriority::Mid,
            WithdrawalPriority::Low,
            WithdrawalPriority::VeryLow,
        ];

        for priority in priorities {
            let priority_json = serde_json::to_value(&priority).unwrap();
            assert!(priority_json.is_string());
            
            // Test deserialization round-trip
            let deserialized: WithdrawalPriority = serde_json::from_value(priority_json).unwrap();
            assert_eq!(deserialized, priority);
        }

        // Test all withdrawal states
        let states = [
            WithdrawalState::Unconfirmed,
            WithdrawalState::Confirmed,
            WithdrawalState::Cancelled,
            WithdrawalState::Completed,
            WithdrawalState::Interrupted,
            WithdrawalState::Rejected,
        ];

        for state in states {
            let state_json = serde_json::to_value(&state).unwrap();
            assert!(state_json.is_string());
            
            // Test deserialization round-trip
            let deserialized: WithdrawalState = serde_json::from_value(state_json).unwrap();
            assert_eq!(deserialized, state);
        }
    }

    #[tokio::test]
    async fn test_private_rest_client_withdraw_method() {
        // Test that the withdraw method is accessible via PrivateRestClient
        use crate::deribit::{PrivateRestClient, AccountTier, RateLimiter};
        use rest::secrets::ExposableSecret;

        // Test secret implementation
        #[derive(Clone)]
        struct PlainTextSecret {
            secret: String,
        }

        impl PlainTextSecret {
            fn new(secret: String) -> Self {
                Self { secret }
            }
        }

        impl ExposableSecret for PlainTextSecret {
            fn expose_secret(&self) -> String {
                self.secret.clone()
            }
        }

        let api_key = Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = PrivateRestClient::new(
            api_key,
            api_secret,
            "https://test.deribit.com",
            rate_limiter,
            client,
        );

        // Test that we can get a function reference to the method
        let _ = PrivateRestClient::withdraw;
        
        // Verify the client exists and has the withdraw method
        let _ = &rest_client;
        
        println!("PrivateRestClient::withdraw method is accessible and properly typed");
    }
}