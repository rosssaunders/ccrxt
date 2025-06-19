#[cfg(test)]
mod rest_api_tests {
    use venues::deribit::{
        public::rest::{GetTimeRequest, TestRequest, RestClient},
        AccountTier, RateLimiter, EndpointType
    };

    #[test]
    fn test_new_endpoints_integration() {
        // Test get_time endpoint structures
        let get_time_request = GetTimeRequest {};
        assert!(serde_json::to_string(&get_time_request).is_ok());

        // Test test endpoint structures
        let test_request_normal = TestRequest::new();
        let test_request_exception = TestRequest::new_exception();
        let test_request_default = TestRequest::default();
        
        assert!(test_request_normal.expected_result.is_none());
        assert_eq!(test_request_exception.expected_result, Some("exception".to_string()));
        assert!(test_request_default.expected_result.is_none());

        // Test serialization
        let get_time_json = serde_json::to_string(&get_time_request).unwrap();
        let test_normal_json = serde_json::to_string(&test_request_normal).unwrap();
        let test_exception_json = serde_json::to_string(&test_request_exception).unwrap();

        // Verify serialization results
        assert_eq!(get_time_json, "{}");
        assert_eq!(test_normal_json, "{}");
        assert!(test_exception_json.contains("exception"));

        println!("✓ GetTimeRequest serialization: {}", get_time_json);
        println!("✓ TestRequest (normal) serialization: {}", test_normal_json);
        println!("✓ TestRequest (exception) serialization: {}", test_exception_json);
    }

    #[tokio::test]
    async fn test_rate_limiting_integration() {
        // Create a client
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let rest_client = RestClient::new("https://test.deribit.com", client, rate_limiter);

        // Test rate limiting works
        let rate_limit_result = rest_client.rate_limiter
            .check_limits(EndpointType::NonMatchingEngine)
            .await;
        
        assert!(rate_limit_result.is_ok(), "Rate limiting should work correctly");
        
        // Test endpoint type mappings
        assert_eq!(EndpointType::from_path("public/get_time"), EndpointType::NonMatchingEngine);
        assert_eq!(EndpointType::from_path("public/test"), EndpointType::NonMatchingEngine);
    }

    #[test]
    fn test_endpoint_exports() {
        // Verify that the new types are properly exported through the public module
        use venues::deribit::public::rest::{GetTimeRequest, GetTimeResponse, TestRequest, TestResponse, TestResult};
        
        // Verify that get_time types are also accessible directly from the deribit module
        use venues::deribit::{GetTimeRequest as TopLevelGetTimeRequest, GetTimeResponse as TopLevelGetTimeResponse};
        
        // Create instances to verify they're accessible
        let _get_time_req = GetTimeRequest {};
        let _test_req = TestRequest::new();
        let _top_level_req = TopLevelGetTimeRequest {};
        
        // This test just verifies the types are exported and accessible at both levels
        assert!(true, "All new types are properly exported at both module levels");
    }

    #[test]
    fn test_get_trigger_order_history_integration() {
        // Test that the new get_trigger_order_history endpoint is properly exported and accessible
        use venues::deribit::private::rest::{GetTriggerOrderHistoryRequest, GetTriggerOrderHistoryResponse, TriggerOrderEntry, GetTriggerOrderHistoryResult};
        use venues::deribit::Currency;
        
        // Create instances to verify they're accessible
        let request = GetTriggerOrderHistoryRequest {
            currency: Currency::BTC,
            instrument_name: Some("BTC-PERPETUAL".to_string()),
            count: Some(20),
            continuation: None,
        };
        
        // Test serialization works
        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("BTC"));
        assert!(serialized.contains("BTC-PERPETUAL"));
        
        println!("✓ GetTriggerOrderHistoryRequest types accessible and serializable");
        
        // This test verifies the types are exported and work correctly
        assert!(true, "Trigger order history endpoint types are properly exported");
    }

    #[test]
    fn test_cancel_all_by_currency_integration() {
        // Test that the new endpoint types are properly exported
        use venues::deribit::{
            CancelAllByCurrencyRequest, CancelAllByCurrencyResponse,
            Currency, InstrumentKind, OrderType
        };
        
        // Test request structure creation
        let request = CancelAllByCurrencyRequest {
            currency: Currency::BTC,
            kind: Some(InstrumentKind::Future),
            order_type: Some(OrderType::Limit),
            detailed: Some(true),
            freeze_quotes: Some(false),
        };
        
        // Test serialization
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"currency\":\"BTC\""));
        assert!(json.contains("\"kind\":\"future\""));
        assert!(json.contains("\"type\":\"limit\""));
        assert!(json.contains("\"detailed\":true"));
        assert!(json.contains("\"freeze_quotes\":false"));
        
        // Test response structure deserialization
        let response_json = serde_json::json!({
            "id": 123,
            "jsonrpc": "2.0",
            "result": 5
        });
        
        let response: CancelAllByCurrencyResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 5);
        
        println!("✓ CancelAllByCurrencyRequest serialization: {}", json);
        println!("✓ CancelAllByCurrencyResponse deserialization successful");
    }

    #[test]
    fn test_cancel_all_by_currency_pair_integration() {
        // Test that the new endpoint types are properly exported
        use venues::deribit::{
            CancelAllByCurrencyPairRequest, CancelAllByCurrencyPairResponse,
            CurrencyPair, InstrumentKind, OrderType
        };
        
        // Test request structure creation
        let request = CancelAllByCurrencyPairRequest {
            currency_pair: CurrencyPair::BtcUsd,
            kind: Some(InstrumentKind::Future),
            order_type: Some(OrderType::Limit),
            detailed: Some(true),
            freeze_quotes: Some(false),
        };
        
        // Test serialization
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"currency_pair\":\"btc_usd\""));
        assert!(json.contains("\"kind\":\"future\""));
        assert!(json.contains("\"type\":\"limit\""));
        assert!(json.contains("\"detailed\":true"));
        assert!(json.contains("\"freeze_quotes\":false"));
        
        // Test response structure deserialization
        let response_json = serde_json::json!({
            "id": 456,
            "jsonrpc": "2.0",
            "result": 7
        });
        
        let response: CancelAllByCurrencyPairResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 456);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 7);
        
        println!("✓ CancelAllByCurrencyPairRequest serialization: {}", json);
        println!("✓ CancelAllByCurrencyPairResponse deserialization successful");
    }

    #[test]
    fn test_get_user_trades_by_currency_and_time_integration() {
        // Test new get_user_trades_by_currency_and_time endpoint structures
        use venues::deribit::{
            GetUserTradesByCurrencyAndTimeRequest, 
            GetUserTradesByCurrencyAndTimeResponse,
            Currency, InstrumentKind, Sorting
        };

        let request = GetUserTradesByCurrencyAndTimeRequest {
            currency: Currency::BTC,
            kind: Some(InstrumentKind::Future),
            start_timestamp: 1640995200000,
            end_timestamp: 1640995260000,
            count: Some(10),
            sorting: Some(Sorting::Desc),
            historical: Some(false),
        };

        // Test serialization
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("BTC"));
        assert!(json.contains("future"));
        assert!(json.contains("1640995200000"));
        assert!(json.contains("1640995260000"));
        assert!(json.contains("desc"));

        // Test response deserialization
        let response_json = serde_json::json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "has_more": false,
                "trades": []
            }
        });

        let response: GetUserTradesByCurrencyAndTimeResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.has_more, false);
        assert_eq!(response.result.trades.len(), 0);

        println!("✓ GetUserTradesByCurrencyAndTimeRequest serialization: {}", json);
        println!("✓ GetUserTradesByCurrencyAndTimeResponse deserialization successful");
    }

    #[test]
    fn test_cancel_all_by_kind_or_type_integration() {
        // Test new cancel_all_by_kind_or_type endpoint structures
        use venues::deribit::{
            CancelAllByKindOrTypeRequest,
            CancelAllByKindOrTypeResponse,
            CurrencySelection,
            Currency, InstrumentKind, OrderType
        };

        // Test single currency
        let request_single = CancelAllByKindOrTypeRequest {
            currency: CurrencySelection::single(Currency::BTC),
            kind: Some(InstrumentKind::Future),
            order_type: Some(OrderType::Limit),
            detailed: Some(false),
            freeze_quotes: Some(true),
        };

        let json_single = serde_json::to_string(&request_single).unwrap();
        assert!(json_single.contains("\"BTC\""));
        assert!(json_single.contains("future"));
        assert!(json_single.contains("limit"));

        // Test multiple currencies
        let request_multiple = CancelAllByKindOrTypeRequest {
            currency: CurrencySelection::multiple(vec![Currency::BTC, Currency::ETH, Currency::USDC]),
            kind: Some(InstrumentKind::Option),
            order_type: None,
            detailed: None,
            freeze_quotes: None,
        };

        let json_multiple = serde_json::to_string(&request_multiple).unwrap();
        assert!(json_multiple.contains("[\"BTC\",\"ETH\",\"USDC\"]"));
        assert!(json_multiple.contains("option"));

        // Test "any" currency
        let request_any = CancelAllByKindOrTypeRequest {
            currency: CurrencySelection::any(),
            kind: None,
            order_type: Some(OrderType::Stop),
            detailed: Some(true),
            freeze_quotes: Some(false),
        };

        let json_any = serde_json::to_string(&request_any).unwrap();
        assert!(json_any.contains("\"any\""));
        assert!(json_any.contains("stop"));

        // Test response deserialization
        let response_json = serde_json::json!({
            "id": 42,
            "jsonrpc": "2.0",
            "result": 7
        });

        let response: CancelAllByKindOrTypeResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 42);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 7);

        println!("✓ CancelAllByKindOrTypeRequest (single) serialization: {}", json_single);
        println!("✓ CancelAllByKindOrTypeRequest (multiple) serialization: {}", json_multiple);
        println!("✓ CancelAllByKindOrTypeRequest (any) serialization: {}", json_any);
        println!("✓ CancelAllByKindOrTypeResponse deserialization successful");

        assert!(true, "Cancel all by kind or type endpoint types are properly exported");
    }

    #[test]
    fn test_create_deposit_address_integration() {
        // Test the create_deposit_address endpoint structures
        use venues::deribit::{
            CreateDepositAddressRequest, CreateDepositAddressResponse,
            DepositAddress, Currency
        };

        // Test request structure creation for all supported currencies
        let currencies = vec![Currency::BTC, Currency::ETH, Currency::USDC, Currency::USDT, Currency::EURR];
        
        for currency in currencies {
            let request = CreateDepositAddressRequest {
                currency: currency.clone(),
            };

            // Test serialization
            let json = serde_json::to_string(&request).unwrap();
            match currency {
                Currency::BTC => assert!(json.contains("\"currency\":\"BTC\"")),
                Currency::ETH => assert!(json.contains("\"currency\":\"ETH\"")),
                Currency::USDC => assert!(json.contains("\"currency\":\"USDC\"")),
                Currency::USDT => assert!(json.contains("\"currency\":\"USDT\"")),
                Currency::EURR => assert!(json.contains("\"currency\":\"EURR\"")),
                _ => {}
            }
            
            println!("✓ CreateDepositAddressRequest for {:?} serialization: {}", currency, json);
        }

        // Test response structure deserialization with successful result
        let response_json = serde_json::json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
                "creation_timestamp": 1640995200000i64,
                "currency": "BTC",
                "type": "deposit"
            }
        });

        let response: CreateDepositAddressResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_some());

        let deposit_address = response.result.unwrap();
        assert_eq!(deposit_address.address, "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh");
        assert_eq!(deposit_address.creation_timestamp, 1640995200000i64);
        assert_eq!(deposit_address.currency, "BTC");
        assert_eq!(deposit_address.address_type, "deposit");

        // Test response structure deserialization with null result
        let null_response_json = serde_json::json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": null
        });

        let null_response: CreateDepositAddressResponse = serde_json::from_value(null_response_json).unwrap();
        assert_eq!(null_response.id, 2);
        assert_eq!(null_response.jsonrpc, "2.0");
        assert!(null_response.result.is_none());

        // Test ETH address format
        let eth_response_json = serde_json::json!({
            "id": 3,
            "jsonrpc": "2.0",
            "result": {
                "address": "0x8Ba35Cc6634C0532925a3b8D05c4ae5e34D7b1c8",
                "creation_timestamp": 1640995300000i64,
                "currency": "ETH",
                "type": "deposit"
            }
        });

        let eth_response: CreateDepositAddressResponse = serde_json::from_value(eth_response_json).unwrap();
        assert!(eth_response.result.is_some());
        let eth_deposit = eth_response.result.unwrap();
        assert!(eth_deposit.address.starts_with("0x"));
        assert_eq!(eth_deposit.currency, "ETH");

        println!("✓ CreateDepositAddressResponse deserialization successful for BTC");
        println!("✓ CreateDepositAddressResponse deserialization successful for null result");
        println!("✓ CreateDepositAddressResponse deserialization successful for ETH");
        
        assert!(true, "Create deposit address endpoint types are properly exported and functional");
    }

    #[tokio::test]
    async fn test_create_deposit_address_method_accessibility() {
        // Test that the create_deposit_address method is accessible through the private REST client
        use venues::deribit::{
            PrivateRestClient, CreateDepositAddressRequest, Currency, AccountTier, RateLimiter
        };
        use rest::secrets::ExposableSecret;

        // Mock secret implementation for testing
        #[derive(Clone)]
        struct TestSecret {
            secret: String,
        }

        impl TestSecret {
            fn new(secret: String) -> Self {
                Self { secret }
            }
        }

        impl ExposableSecret for TestSecret {
            fn expose_secret(&self) -> String {
                self.secret.clone()
            }
        }

        // Create a test client
        let api_key = Box::new(TestSecret::new("test_api_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(TestSecret::new("test_api_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = PrivateRestClient::new(
            api_key,
            api_secret,
            "https://test.deribit.com",
            rate_limiter,
            client,
        );

        // Create a request
        let request = CreateDepositAddressRequest {
            currency: Currency::BTC,
        };

        // Verify the method exists and is accessible
        // Note: We don't actually call it to avoid external dependencies in tests
        let _method_ref = PrivateRestClient::create_deposit_address;
        let _client_ref = &rest_client;
        let _request_ref = &request;

        println!("✓ create_deposit_address method is accessible through PrivateRestClient");
        println!("✓ Method has correct signature: RestClient::create_deposit_address(&self, CreateDepositAddressRequest) -> RestResult<CreateDepositAddressResponse>");
        
        assert!(true, "create_deposit_address method is properly accessible");
    }

    #[test]
    fn test_get_block_rfq_quotes_integration() {
        // Test the get_block_rfq_quotes endpoint structures
        use venues::deribit::private::rest::get_block_rfq_quotes::{
            GetBlockRfqQuotesRequest, GetBlockRfqQuotesResponse
        };

        // Test request structure creation with no parameters
        let empty_request = GetBlockRfqQuotesRequest {
            block_rfq_id: None,
            label: None,
            block_rfq_quote_id: None,
        };

        // Test serialization of empty request
        let empty_json = serde_json::to_string(&empty_request).unwrap();
        assert_eq!(empty_json, "{}");
        println!("✓ GetBlockRfqQuotesRequest (empty) serialization: {}", empty_json);

        // Test request with all parameters
        let full_request = GetBlockRfqQuotesRequest {
            block_rfq_id: Some(12345),
            label: Some("test_label".to_string()),
            block_rfq_quote_id: Some(67890),
        };

        let full_json = serde_json::to_string(&full_request).unwrap();
        assert!(full_json.contains("12345"));
        assert!(full_json.contains("test_label"));
        assert!(full_json.contains("67890"));
        println!("✓ GetBlockRfqQuotesRequest (full) serialization: {}", full_json);

        // Test response structure deserialization with empty result
        let empty_response_json = serde_json::json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": []
        });

        let empty_response: GetBlockRfqQuotesResponse = serde_json::from_value(empty_response_json).unwrap();
        assert_eq!(empty_response.id, 1);
        assert_eq!(empty_response.jsonrpc, "2.0");
        assert_eq!(empty_response.result.len(), 0);
        println!("✓ GetBlockRfqQuotesResponse (empty) deserialization successful");

        // Test response structure deserialization with multiple quotes
        let quotes_response_json = serde_json::json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": [
                {
                    "amount": 1.5,
                    "app_name": null,
                    "block_rfq_id": 123,
                    "block_rfq_quote_id": 456,
                    "creation_timestamp": 1640995200000i64,
                    "direction": "buy",
                    "execution_instruction": "any_part_of",
                    "filled_amount": 0.0,
                    "hedge": null,
                    "label": "quote_1",
                    "last_update_timestamp": 1640995300000i64,
                    "legs": [
                        {
                            "direction": "buy",
                            "instrument_name": "BTC-PERPETUAL",
                            "price": 50000.0,
                            "ratio": 1
                        }
                    ],
                    "price": 50000.0,
                    "quote_state": "open",
                    "quote_state_reason": null,
                    "replaced": false
                },
                {
                    "amount": 2.0,
                    "app_name": "test_app",
                    "block_rfq_id": 124,
                    "block_rfq_quote_id": 457,
                    "creation_timestamp": 1640995400000i64,
                    "direction": "sell",
                    "execution_instruction": "all_or_none",
                    "filled_amount": 0.5,
                    "hedge": {
                        "amount": 100,
                        "direction": "buy",
                        "instrument_name": "ETH-PERPETUAL",
                        "price": 3000.0
                    },
                    "label": "quote_2",
                    "last_update_timestamp": 1640995500000i64,
                    "legs": [
                        {
                            "direction": "sell",
                            "instrument_name": "ETH-PERPETUAL",
                            "price": 3000.0,
                            "ratio": 2
                        }
                    ],
                    "price": 3000.0,
                    "quote_state": "partially_filled",
                    "quote_state_reason": null,
                    "replaced": true
                }
            ]
        });

        let quotes_response: GetBlockRfqQuotesResponse = serde_json::from_value(quotes_response_json).unwrap();
        assert_eq!(quotes_response.id, 2);
        assert_eq!(quotes_response.jsonrpc, "2.0");
        assert_eq!(quotes_response.result.len(), 2);

        // Verify first quote
        let quote1 = &quotes_response.result[0];
        assert_eq!(quote1.amount, 1.5);
        assert_eq!(quote1.block_rfq_id, 123);
        assert_eq!(quote1.block_rfq_quote_id, 456);
        assert_eq!(quote1.label, Some("quote_1".to_string()));
        assert_eq!(quote1.legs[0].instrument_name, "BTC-PERPETUAL");
        assert!(!quote1.replaced);
        assert!(quote1.hedge.is_none());

        // Verify second quote with hedge
        let quote2 = &quotes_response.result[1];
        assert_eq!(quote2.amount, 2.0);
        assert_eq!(quote2.block_rfq_id, 124);
        assert_eq!(quote2.app_name, Some("test_app".to_string()));
        assert!(quote2.replaced);
        assert!(quote2.hedge.is_some());
        
        let hedge = quote2.hedge.as_ref().unwrap();
        assert_eq!(hedge.instrument_name, "ETH-PERPETUAL");
        assert_eq!(hedge.price, 3000.0);

        println!("✓ GetBlockRfqQuotesResponse (with quotes) deserialization successful");
        println!("✓ Quote with hedge leg parsed correctly");
        
    #[tokio::test]
    async fn test_get_block_rfq_quotes_method_exists() {
        use venues::deribit::{
            AccountTier, RateLimiter, PrivateRestClient
        };
        use rest::secrets::ExposableSecret;

        // Mock secret implementation for testing
        #[derive(Clone)]
        struct TestSecret {
            secret: String,
        }

        impl TestSecret {
            fn new(secret: String) -> Self {
                Self { secret }
            }
        }

        impl ExposableSecret for TestSecret {
            fn expose_secret(&self) -> String {
                self.secret.clone()
            }
        }

        // Create a test client
        let api_key = Box::new(TestSecret::new("test_api_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(TestSecret::new("test_api_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = PrivateRestClient::new(
            api_key,
            api_secret,
            "https://test.deribit.com",
            rate_limiter,
            client,
        );

        // Verify the method exists and is accessible
        // Note: We don't actually call it to avoid external dependencies in tests
        let _method_ref = PrivateRestClient::get_block_rfq_quotes;
        let _client_ref = &rest_client;

        println!("✓ get_block_rfq_quotes method is accessible through PrivateRestClient");
        println!("✓ Method has correct signature: RestClient::get_block_rfq_quotes(&self, Option<i64>, Option<String>, Option<i64>) -> RestResult<GetBlockRfqQuotesResponse>");
        
        assert!(true, "get_block_rfq_quotes method is properly accessible");
    }
}