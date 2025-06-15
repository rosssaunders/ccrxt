#[cfg(test)]
mod integration_tests {
    use crate::deribit::{PublicRestClient, AccountTier};

    #[tokio::test]
    async fn test_fork_token_endpoint_compile() {
        // Test that the fork_token endpoint compiles and is accessible
        let client = PublicRestClient::new_default("https://api.deribit.com", AccountTier::Tier3);

        // Test that the method exists by verifying we can get a function reference to it
        // This proves it compiles and is accessible without needing to call it
        let _ = PublicRestClient::fork_token;

        // Verify RestClient itself compiles
        let _ = &client;

        println!("Fork token endpoint is accessible and properly typed");
    }

    #[test]
    fn test_fork_token_json_rpc_structure() {
        use crate::deribit::public::ForkTokenRequest;
        use serde_json;

        // Test that the request structure matches JSON-RPC 2.0 format
        let request = ForkTokenRequest::new(
            "test_refresh_token".to_string(),
            "test_session".to_string(),
            42
        );

        let json_str = serde_json::to_string(&request).expect("Should serialize");
        let json_value: serde_json::Value = serde_json::from_str(&json_str).expect("Should parse");

        // Verify JSON-RPC 2.0 structure
        assert_eq!(json_value["jsonrpc"], "2.0");
        assert_eq!(json_value["method"], "public/fork_token");
        assert_eq!(json_value["id"], 42);
        assert!(json_value["params"].is_object());
        assert_eq!(json_value["params"]["refresh_token"], "test_refresh_token");
        assert_eq!(json_value["params"]["session_name"], "test_session");
    }

    #[test]
    fn test_fork_token_response_parsing() {
        use crate::deribit::public::ForkTokenResponse;
        use serde_json;

        // Test with realistic Deribit API response format
        let response_json = r#"{
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImp0aSI6Ijc2ZTQ4ZGI4LTcwZjMtNGVkNi04ZjYyLWM4ZTJhYTQyNjY4MSIsImN0eSI6Im9wYXF1ZSJ9.eyJhdWQiOiJkZXJpYml0IiwiaXNzIjoiZGVyaWJpdCIsInR5cCI6IkpXVCIsInN1YiI6Ijc2ZTQ4ZGI4LTcwZjMtNGVkNi04ZjYyLWM4ZTJhYTQyNjY4MSIsImp0aSI6Ijc2ZTQ4ZGI4LTcwZjMtNGVkNi04ZjYyLWM4ZTJhYTQyNjY4MSIsInVpZCI6MTIzNDUsInNpZCI6InRlc3Rfc2Vzc2lvbiIsImF1dGhfdXNlcl9pZCI6MTIzNDUsImV4cCI6MTY0MzgwNzIwMCwiaWF0IjoxNjQzODA2MzAwLCJjdHkiOiJvcGFxdWUifQ.signature",
                "expires_in": 604800,
                "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImp0aSI6IjIyYWI5NDJmLTQ3ODItNGNjNy05ZmNmLWVhNTVhMWJlNDM1NyIsImN0eSI6Im9wYXF1ZSJ9.eyJhdWQiOiJkZXJpYml0IiwiaXNzIjoiZGVyaWJpdCIsInR5cCI6IkpXVCIsInN1YiI6IjIyYWI5NDJmLTQ3ODItNGNjNy05ZmNmLWVhNTVhMWJlNDM1NyIsImp0aSI6IjIyYWI5NDJmLTQ3ODItNGNjNy05ZmNmLWVhNTVhMWJlNDM1NyIsInVpZCI6MTIzNDUsImF1dGhfdXNlcl9pZCI6MTIzNDUsImV4cCI6MTY3NTM0MzIwMCwiaWF0IjoxNjQzODA2MzAwLCJjdHkiOiJvcGFxdWUifQ.signature",
                "scope": "session:test_session",
                "sid": "test_session_id",
                "token_type": "bearer"
            }
        }"#;

        let response: ForkTokenResponse = serde_json::from_str(response_json)
            .expect("Should parse realistic Deribit response");

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.access_token.starts_with("eyJ"));
        assert_eq!(response.result.expires_in, 604800);
        assert!(response.result.refresh_token.starts_with("eyJ"));
        assert_eq!(response.result.scope, "session:test_session");
        assert_eq!(response.result.sid, Some("test_session_id".to_string()));
        assert_eq!(response.result.token_type, "bearer");
    }

    #[test] 
    fn test_endpoint_type_classification() {
        use crate::deribit::EndpointType;

        // Test that fork_token is correctly classified
        assert_eq!(
            EndpointType::from_path("public/fork_token"),
            EndpointType::PublicForkToken
        );

        // Test credit cost
        assert_eq!(EndpointType::PublicForkToken.credit_cost(), 500);
    }

    #[tokio::test]
    async fn test_rate_limiting_for_fork_token() {
        use crate::deribit::{RateLimiter, EndpointType, AccountTier};
        use std::sync::Arc;

        let limiter = Arc::new(RateLimiter::new(AccountTier::Tier4));

        // Should be able to make fork_token requests within credit limit
        assert!(limiter.check_limits(EndpointType::PublicForkToken).await.is_ok());
        limiter.record_request(EndpointType::PublicForkToken).await;

        let status = limiter.get_status().await;
        assert_eq!(status.available_credits, 50_000 - 500); // 500 credits consumed
    }

    #[test]
    fn test_error_response_format() {
        use crate::deribit::ErrorResponse;
        use serde_json;

        // Test that we can parse Deribit error responses
        let error_json = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32602,
                "message": "Invalid params",
                "data": {
                    "reason": "refresh_token is required"
                }
            }
        }"#;

        let error_response: ErrorResponse = serde_json::from_str(error_json)
            .expect("Should parse error response");

        assert_eq!(error_response.jsonrpc, "2.0");
        assert_eq!(error_response.id, Some(serde_json::Value::Number(serde_json::Number::from(1))));
        assert_eq!(error_response.error.code, -32602);
        assert_eq!(error_response.error.message, "Invalid params");
        assert!(error_response.error.data.is_some());
    }
}