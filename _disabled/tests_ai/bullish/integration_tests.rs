//! Integration tests for Bullish module

#[cfg(test)]
mod tests {
    use crate::bullish::*;
    use reqwest::Client;

    #[test]
    fn test_bullish_module_exports() {
        // Test that all main types are exported and accessible
        let _order_side = OrderSide::Buy;
        let _order_type = OrderType::Limit;
        let _time_in_force = TimeInForce::Gtc;
        let _order_status = OrderStatus::Open;
        
        // Test error types
        let _error = Errors::InvalidApiKey();
        
        // Test rate limiting types
        let _endpoint_type = EndpointType::PrivateTradingAccounts;
        let _rate_limiter = RateLimiter::new();
        
        // Test client creation (without actual API calls)
        let client = Client::new();
        let rate_limiter = RateLimiter::new();
        
        let _public_client = PublicRestClient::new(
            "https://api.exchange.bullish.com",
            client.clone(),
            rate_limiter.clone(),
        );
        
        // Private client would need secrets, so just test the type exists
        let _private_client_type: Option<PrivateRestClient> = None;
        
        // Test trading account types
        let _trading_account_type: Option<TradingAccount> = None;
        let _trading_accounts_response_type: Option<TradingAccountsResponse> = None;
    }

    #[test]
    fn test_bullish_enums_functionality() {
        // Test enum serialization works
        let order_side = OrderSide::Buy;
        let json = serde_json::to_string(&order_side).unwrap();
        assert_eq!(json, "\"BUY\"");
        
        let order_type = OrderType::Limit;
        let json = serde_json::to_string(&order_type).unwrap();
        assert_eq!(json, "\"LMT\"");
    }

    #[test]
    fn test_bullish_rate_limiting() {
        let _rate_limiter = RateLimiter::new();
        let endpoint = EndpointType::PublicMarkets;
        
        // Verify rate limit configuration
        let rate_limit = endpoint.rate_limit();
        assert_eq!(rate_limit.max_requests, 50);
        assert_eq!(rate_limit.window.as_secs(), 1);
    }

    #[test]
    fn test_bullish_error_handling() {
        let api_error = ApiError {
            code: "TEST_ERROR".to_string(),
            message: "Test error message".to_string(),
            details: None,
        };
        
        let error = Errors::ApiError(api_error);
        assert!(error.to_string().contains("TEST_ERROR"));
    }
}