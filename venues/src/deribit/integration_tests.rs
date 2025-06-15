//! Integration tests for the Deribit public get_time endpoint
//! 
//! This module demonstrates how to use the get_time endpoint and validates
//! that the implementation follows the expected patterns.

#[cfg(test)]
mod integration_tests {
    use crate::deribit::{
        PublicRestClient, RateLimiter, AccountTier,
        JsonRpcRequest, JsonRpcResponse, 
        EndpointType
    };
    use reqwest::Client;
    use serde_json::json;

    #[tokio::test]
    async fn test_public_rest_client_creation_and_usage() {
        // Demonstrate how a user would create and use the client
        let client = Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let rest_client = PublicRestClient::new(
            "https://test.deribit.com", 
            client, 
            rate_limiter
        );

        // Verify the client was created successfully
        // This test validates method signatures without making actual API calls
        let _ = &rest_client;
        println!("Successfully created Deribit public REST client");
    }

    #[test]
    fn test_json_rpc_request_format_for_get_time() {
        // Test that we can create a proper JSON-RPC request for get_time
        let request = JsonRpcRequest::<()>::new(
            "public/get_time".to_string(),
            None,
            12345,
        );

        // Serialize to JSON and verify format
        let json = serde_json::to_value(&request).unwrap();
        
        assert_eq!(json["jsonrpc"], "2.0");
        assert_eq!(json["method"], "public/get_time");
        assert_eq!(json["id"], 12345);
        assert!(json["params"].is_null());
        
        // Verify the complete JSON structure matches expected format
        let json_string = serde_json::to_string(&request).unwrap();
        assert!(json_string.contains("\"jsonrpc\":\"2.0\""));
        assert!(json_string.contains("\"method\":\"public/get_time\""));
        assert!(json_string.contains("\"id\":12345"));
    }

    #[test]
    fn test_json_rpc_response_parsing_for_get_time() {
        // Test that we can parse a Deribit get_time response
        let response_json = json!({
            "jsonrpc": "2.0",
            "id": 12345,
            "result": 1597026383085u64
        });

        let response: JsonRpcResponse<u64> = serde_json::from_value(response_json).unwrap();
        
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 12345);
        assert_eq!(response.result, Some(1597026383085u64));
        assert!(response.error.is_none());
    }

    #[test]
    fn test_json_rpc_error_response_parsing() {
        // Test that we can parse a Deribit error response
        let error_response_json = json!({
            "jsonrpc": "2.0",
            "id": 12345,
            "error": {
                "code": -32602,
                "message": "Invalid params"
            }
        });

        let response: JsonRpcResponse<u64> = serde_json::from_value(error_response_json).unwrap();
        
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 12345);
        assert!(response.result.is_none());
        
        let error = response.error.unwrap();
        assert_eq!(error.code, -32602);
        assert_eq!(error.message, "Invalid params");
    }

    #[test]
    fn test_endpoint_type_classification() {
        // Verify that get_time is properly classified for rate limiting
        let endpoint_type = EndpointType::from_path("public/get_time");
        
        assert_eq!(endpoint_type, EndpointType::PublicGetTime);
        assert_eq!(endpoint_type.credit_cost(), 0);
    }

    #[tokio::test]
    async fn test_rate_limiting_for_get_time_endpoint() {
        // Test that rate limiting allows get_time requests
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        
        // Should allow multiple get_time requests since they have no specific limits
        for _ in 0..10 {
            let result = rate_limiter.check_limits(EndpointType::PublicGetTime).await;
            assert!(result.is_ok(), "get_time endpoint should not be rate limited");
            
            rate_limiter.record_request(EndpointType::PublicGetTime).await;
        }
    }

    #[test] 
    fn test_response_format_matches_deribit_spec() {
        // Verify our response format matches the expected Deribit API specification
        let expected_response = json!({
            "id": 42,
            "jsonrpc": "2.0",
            "result": 1597026383085u64
        });

        // Parse as our response type
        let response: JsonRpcResponse<u64> = serde_json::from_value(expected_response.clone()).unwrap();
        
        // Serialize back and compare
        let serialized = serde_json::to_value(&response).unwrap();
        
        assert_eq!(serialized["id"], 42);
        assert_eq!(serialized["jsonrpc"], "2.0");
        assert_eq!(serialized["result"], 1597026383085u64);
        assert!(serialized["error"].is_null());
    }
}