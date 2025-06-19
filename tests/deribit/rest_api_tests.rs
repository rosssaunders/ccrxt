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
}