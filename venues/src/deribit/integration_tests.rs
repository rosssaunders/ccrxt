//! Integration tests for Deribit public API endpoints
//!
//! These tests demonstrate the usage of the Deribit JSON-RPC client
//! and validate the implementation against the expected API format.

#[cfg(test)]
mod integration_tests {
    use crate::deribit::{
        public::JsonRpcClient,
        AccountTier, RateLimiter, EndpointType, JsonRpcRequest, JsonRpcResponse,
        DeribitError, public::StatusResult,
    };

    /// Test that demonstrates creating a Deribit client and calling the status endpoint
    ///
    /// This test verifies that:
    /// 1. The client can be created with proper configuration
    /// 2. The get_status method is accessible and properly typed
    /// 3. Rate limiting is properly integrated
    ///
    /// Note: This test doesn't make actual HTTP requests to avoid depending on
    /// external services in the test suite.
    #[tokio::test]
    async fn test_deribit_client_integration() {
        // Create HTTP client
        let http_client = reqwest::Client::new();

        // Create rate limiter for a Tier 4 account (up to $1M trading volume)
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        // Create Deribit JSON-RPC client
        let deribit_client = JsonRpcClient::new(
            "https://www.deribit.com/api/v2",
            http_client,
            rate_limiter,
        );

        // Verify the client is properly configured
        assert_eq!(deribit_client.base_url, "https://www.deribit.com/api/v2");

        // Verify the get_status method exists and is properly typed
        // We can't call it without a real server, but we can verify the method signature
        let _method_ref = JsonRpcClient::get_status;

        // Verify rate limiting works
        let rate_limit_check = deribit_client
            .rate_limiter
            .check_limits(EndpointType::PublicStatus)
            .await;
        assert!(rate_limit_check.is_ok(), "Rate limiting should allow the first request");

        // Verify rate limiter state
        let status = deribit_client.rate_limiter.get_status().await;
        assert_eq!(status.account_tier, AccountTier::Tier4);
        assert_eq!(status.available_credits, 50_000 - 500); // 500 credits consumed by check_limits
    }

    /// Test demonstrating the expected JSON-RPC request format for public/status
    #[test]
    fn test_status_request_format() {
        // Create a request for the public/status endpoint
        let request: JsonRpcRequest<()> = JsonRpcRequest::new(
            1,
            "public/status".to_string(),
            None,
        );

        // Serialize to JSON to verify the format
        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        // Verify JSON-RPC format
        assert_eq!(json_value["jsonrpc"], "2.0");
        assert_eq!(json_value["id"], 1);
        assert_eq!(json_value["method"], "public/status");
        
        // params should be absent for this endpoint (no parameters)
        assert!(json_value.get("params").is_none());
    }

    /// Test demonstrating the expected JSON-RPC response format for public/status
    #[test]
    fn test_status_response_format() {
        // Mock response JSON that matches Deribit's expected format
        let response_json = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "locked": "false",
                "locked_indices": []
            }
        });

        // Parse the response
        let response: JsonRpcResponse<StatusResult> = 
            serde_json::from_value(response_json).unwrap();

        // Verify response structure
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 1);
        assert!(response.result.is_some());
        assert!(response.error.is_none());

        // Verify result data
        let result = response.result.unwrap();
        assert_eq!(result.locked, "false");
        assert!(result.locked_indices.is_empty());
    }

    /// Test demonstrating error response handling
    #[test]
    fn test_error_response_format() {
        // Mock error response JSON
        let error_response_json = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32601,
                "message": "Method not found"
            }
        });

        // Parse the response
        let response: JsonRpcResponse<StatusResult> = 
            serde_json::from_value(error_response_json).unwrap();

        // Verify error handling
        assert!(response.is_error());
        let result = response.into_result();
        assert!(result.is_err());

        match result.unwrap_err() {
            DeribitError::ApiError { code, message } => {
                assert_eq!(code, -32601);
                assert_eq!(message, "Method not found");
            }
            _ => panic!("Expected ApiError"),
        }
    }

    /// Test all possible lock status values
    #[test]
    fn test_all_lock_status_values() {
        let test_cases = vec![
            ("false", vec![], "No currencies locked"),
            ("partial", vec![1, 3, 5], "Some currencies locked"),
            ("true", vec![0, 1, 2, 3, 4, 5], "All currencies locked"),
        ];

        for (locked_value, indices, description) in test_cases {
            let status_json = serde_json::json!({
                "locked": locked_value,
                "locked_indices": indices
            });

            let status: StatusResult = serde_json::from_value(status_json).unwrap();
            assert_eq!(status.locked, locked_value, "Failed for case: {}", description);
            assert_eq!(status.locked_indices, indices, "Failed for case: {}", description);
        }
    }

    /// Test rate limiting behavior for multiple requests
    #[tokio::test]
    async fn test_rate_limiting_multiple_requests() {
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        // Make multiple status requests (each consumes 500 credits)
        for i in 1..=5 {
            let result = rate_limiter.check_limits(EndpointType::PublicStatus).await;
            assert!(result.is_ok(), "Request {} should succeed", i);
            rate_limiter.record_request(EndpointType::PublicStatus).await;
        }

        // Check remaining credits
        let status = rate_limiter.get_status().await;
        assert_eq!(status.available_credits, 50_000 - (5 * 500), "Should have consumed 2500 credits");
    }
}