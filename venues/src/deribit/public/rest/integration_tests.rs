#[cfg(test)]
mod integration_tests {
    use crate::deribit::{
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
        // Verify that the new types are properly exported
        use crate::deribit::public::rest::{GetTimeRequest, GetTimeResponse, TestRequest, TestResponse, TestResult};
        
        // Create instances to verify they're accessible
        let _get_time_req = GetTimeRequest {};
        let _test_req = TestRequest::new();
        
        // This test just verifies the types are exported and accessible
        assert!(true, "All new types are properly exported");
    }
}