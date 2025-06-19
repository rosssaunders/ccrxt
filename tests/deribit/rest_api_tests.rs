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
}