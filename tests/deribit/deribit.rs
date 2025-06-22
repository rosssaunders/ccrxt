//! Integration test example for Deribit public/get_combo_ids endpoint
//! 
//! This test shows how users can use the new endpoint to fetch combo IDs.
//! 
//! Integration tests for Deribit have been moved to `tests/deribit.rs` as per project testing instructions.
//! This file is now deprecated and should not contain integration tests.

#[cfg(test)]
mod usage_examples {
    use crate::deribit::{
        AccountTier, Currency, ComboState, GetComboIdsRequest, GetCombosRequest, PublicRestClient, RateLimiter,
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
    fn test_get_combos_request_usage() {
        // Test creating requests for the new get_combos endpoint
        let btc_request = GetCombosRequest {
            currency: Currency::BTC,
        };

        let eth_request = GetCombosRequest {
            currency: Currency::ETH,
        };

        let any_request = GetCombosRequest {
            currency: Currency::Any, // Get combos for all currencies
        };

        // Verify serialization works correctly
        let btc_json = serde_json::to_value(&btc_request).unwrap();
        assert_eq!(btc_json["currency"], "BTC");

        let eth_json = serde_json::to_value(&eth_request).unwrap();
        assert_eq!(eth_json["currency"], "ETH");

        let any_json = serde_json::to_value(&any_request).unwrap();
        assert_eq!(any_json["currency"], "any");
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
        let combo_ids_request = GetComboIdsRequest {
            currency: Currency::BTC,
            state: Some(ComboState::Active),
        };

        let get_combos_request = GetCombosRequest {
            currency: Currency::BTC,
        };

        // This creates the future but doesn't execute it - just verifies the method signature
        let _combo_ids_future = rest_client.get_combo_ids(combo_ids_request);
        let _get_combos_future = rest_client.get_combos(get_combos_request);

        // If we reach this point, the methods exist and have correct signatures
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

    #[test]
    fn test_get_combos_response_structure_parsing() {
        // Test parsing a realistic get_combos response structure
        let response_json = serde_json::json!({
            "id": 123,
            "jsonrpc": "2.0",
            "result": [
                {
                    "creation_timestamp": 1640995200000i64,
                    "id": "BTC-28JUN24-65000-C_BTC-28JUN24-70000-P",
                    "instrument_id": 123456,
                    "legs": [
                        {
                            "amount": 1,
                            "instrument_name": "BTC-28JUN24-65000-C"
                        },
                        {
                            "amount": -1,
                            "instrument_name": "BTC-28JUN24-70000-P"
                        }
                    ],
                    "state": "active",
                    "state_timestamp": 1640995200000i64
                },
                {
                    "creation_timestamp": 1640995300000i64,
                    "id": "ETH-28JUN24-3000-C_ETH-28JUN24-3500-P",
                    "instrument_id": 789012,
                    "legs": [
                        {
                            "amount": 1,
                            "instrument_name": "ETH-28JUN24-3000-C"
                        },
                        {
                            "amount": -1,
                            "instrument_name": "ETH-28JUN24-3500-P"
                        }
                    ],
                    "state": "rfq",
                    "state_timestamp": 1640995300000i64
                }
            ]
        });

        let response: crate::deribit::GetCombosResponse = 
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.len(), 2);
        
        let first_combo = &response.result[0];
        assert_eq!(first_combo.id, "BTC-28JUN24-65000-C_BTC-28JUN24-70000-P");
        assert_eq!(first_combo.instrument_id, 123456);
        assert_eq!(first_combo.state, "active");
        assert_eq!(first_combo.legs.len(), 2);
        assert_eq!(first_combo.legs[0].amount, 1);
        assert_eq!(first_combo.legs[1].amount, -1);
        
        let second_combo = &response.result[1];
        assert_eq!(second_combo.id, "ETH-28JUN24-3000-C_ETH-28JUN24-3500-P");
        assert_eq!(second_combo.state, "rfq");
    }

    #[tokio::test]
    async fn test_rate_limiting_for_combo_ids() {
        // Test that the rate limiting system correctly handles the new endpoint
        let rate_limiter = RateLimiter::new(AccountTier::Tier3);

        // The endpoint should use the PublicGetComboIds endpoint type,
        // which consumes 500 credits (same as other non-matching engine endpoints)
        let result = rate_limiter
            .check_limits(EndpointType::PublicGetComboIds)
            .await;
        assert!(result.is_ok());

        // Record the request
        rate_limiter
            .record_request(EndpointType::PublicGetComboIds)
            .await;

        // Check that the credit system is working
        let status = rate_limiter.get_status().await;
        // After one request, we should have consumed 500 credits from the default 50,000
        assert_eq!(status.available_credits, 50_000 - 500);
    }

    #[tokio::test]
    async fn test_rate_limiting_for_get_combos() {
        // Test that the rate limiting system correctly handles the new get_combos endpoint
        let rate_limiter = RateLimiter::new(AccountTier::Tier3);

        // The endpoint should use the PublicGetCombos endpoint type,
        // which consumes 500 credits (same as other non-matching engine endpoints)
        let result = rate_limiter
            .check_limits(EndpointType::PublicGetCombos)
            .await;
        assert!(result.is_ok());

        // Record the request
        rate_limiter
            .record_request(EndpointType::PublicGetCombos)
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
            Currency::Any,
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

        // Test get_combos with all currencies
        for currency in currencies {
            let request = GetCombosRequest { currency };
            let json = serde_json::to_value(&request).unwrap();
            assert!(json["currency"].is_string());
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

    #[tokio::test]
    async fn test_private_rest_client_submit_transfer_between_subaccounts_method() {
        // Test that the submit_transfer_between_subaccounts method is accessible via PrivateRestClient
        use crate::deribit::{PrivateRestClient, AccountTier, RateLimiter, SubmitTransferBetweenSubaccountsRequest};
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
        let _ = PrivateRestClient::submit_transfer_between_subaccounts;
        
        // Test creating a request for transfer between subaccounts
        let request = SubmitTransferBetweenSubaccountsRequest {
            currency: "BTC".to_string(),
            amount: 0.001,
            destination: 12345,
            source: Some(67890),
        };

        // Verify request can be serialized
        let json_str = serde_json::to_string(&request).unwrap();
        assert!(json_str.contains("BTC"));
        assert!(json_str.contains("12345"));
        assert!(json_str.contains("67890"));
        
        // Verify the client exists and has the method
        let _ = &rest_client;
        
        println!("PrivateRestClient::submit_transfer_between_subaccounts method is accessible and properly typed");
    }

    #[tokio::test]
    async fn test_private_rest_client_get_user_trades_by_order_method() {
        // Test that the get_user_trades_by_order method is accessible via PrivateRestClient
        use crate::deribit::{PrivateRestClient, AccountTier, RateLimiter, GetUserTradesByOrderRequest, Sorting};
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
        let _ = PrivateRestClient::get_user_trades_by_order;
        
        // Test creating a request for getting user trades by order
        let request = GetUserTradesByOrderRequest {
            order_id: "ETH-12345".to_string(),
            sorting: Some(Sorting::Desc),
            historical: Some(false),
        };

        // Verify request can be serialized
        let json_str = serde_json::to_string(&request).unwrap();
        assert!(json_str.contains("ETH-12345"));
        assert!(json_str.contains("desc"));
        assert!(json_str.contains("false"));
        
        // Verify the client exists and has the method
        let _ = &rest_client;
        
        println!("PrivateRestClient::get_user_trades_by_order method is accessible and properly typed");
    }
}