//! Integration tests for Coinbase Exchange API
//!
//! These tests verify the API client functionality using mock responses

#[cfg(test)]
mod tests {
    use super::super::*;
    use reqwest::Client;
    use rest::secrets::SecretValue;
    use secrecy::SecretString;
    
    // Helper function to create a test client
    fn create_test_client() -> PrivateRestClient {
        let api_key = Box::new(SecretValue::new(SecretString::new("test_key".to_string().into())));
        let api_secret = Box::new(SecretValue::new(SecretString::new("dGVzdF9zZWNyZXQ=".to_string().into()))); // "test_secret" base64 encoded
        let api_passphrase = Box::new(SecretValue::new(SecretString::new("test_passphrase".to_string().into())));
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        PrivateRestClient::new(
            api_key,
            api_secret,
            api_passphrase,
            "https://api.exchange.coinbase.com",
            client,
            rate_limiter,
        )
    }

    #[test]
    fn test_client_initialization() {
        let client = create_test_client();
        assert_eq!(client.base_url, "https://api.exchange.coinbase.com");
    }

    #[test]
    fn test_rate_limiter_functionality() {
        let rate_limiter = RateLimiter::new();
        
        // Test that we can check limits for different endpoint types
        tokio_test::block_on(async {
            // These should succeed as we haven't made any requests yet
            assert!(rate_limiter.check_limit(EndpointType::Public).await.is_ok());
            assert!(rate_limiter.check_limit(EndpointType::Private).await.is_ok());
            assert!(rate_limiter.check_limit(EndpointType::PrivateFills).await.is_ok());
            assert!(rate_limiter.check_limit(EndpointType::PrivateLoans).await.is_ok());
        });
    }

    #[test]
    fn test_order_enums() {
        use serde_json;
        
        // Test OrderSide serialization
        let buy_side = OrderSide::Buy;
        let sell_side = OrderSide::Sell;
        
        assert_eq!(serde_json::to_string(&buy_side).unwrap(), "\"buy\"");
        assert_eq!(serde_json::to_string(&sell_side).unwrap(), "\"sell\"");
        
        // Test OrderType serialization
        let limit_order = OrderType::Limit;
        let market_order = OrderType::Market;
        let stop_order = OrderType::Stop;
        
        assert_eq!(serde_json::to_string(&limit_order).unwrap(), "\"limit\"");
        assert_eq!(serde_json::to_string(&market_order).unwrap(), "\"market\"");
        assert_eq!(serde_json::to_string(&stop_order).unwrap(), "\"stop\"");
        
        // Test TimeInForce serialization
        let gtc = TimeInForce::GoodTillCanceled;
        let gtt = TimeInForce::GoodTillTime;
        let ioc = TimeInForce::ImmediateOrCancel;
        let fok = TimeInForce::FillOrKill;
        
        assert_eq!(serde_json::to_string(&gtc).unwrap(), "\"GTC\"");
        assert_eq!(serde_json::to_string(&gtt).unwrap(), "\"GTT\"");
        assert_eq!(serde_json::to_string(&ioc).unwrap(), "\"IOC\"");
        assert_eq!(serde_json::to_string(&fok).unwrap(), "\"FOK\"");
    }

    #[test]
    fn test_self_trade_prevention_enum() {
        use serde_json;
        
        let dc = SelfTradePrevention::DecrementAndCancel;
        let co = SelfTradePrevention::CancelOldest;
        let cn = SelfTradePrevention::CancelNewest;
        let cb = SelfTradePrevention::CancelBoth;
        
        assert_eq!(serde_json::to_string(&dc).unwrap(), "\"dc\"");
        assert_eq!(serde_json::to_string(&co).unwrap(), "\"co\"");
        assert_eq!(serde_json::to_string(&cn).unwrap(), "\"cn\"");
        assert_eq!(serde_json::to_string(&cb).unwrap(), "\"cb\"");
    }

    #[test]
    fn test_error_handling() {
        // Test ApiError variants
        let bad_request = ApiError::BadRequest { msg: "Invalid request".to_string() };
        let unauthorized = ApiError::Unauthorized { msg: "Invalid API key".to_string() };
        let not_found = ApiError::NotFound { msg: "Resource not found".to_string() };
        
        assert!(format!("{}", bad_request).contains("Bad Request"));
        assert!(format!("{}", unauthorized).contains("Unauthorized"));
        assert!(format!("{}", not_found).contains("Not Found"));
    }

    #[test]
    fn test_error_response_mapping() {
        use crate::coinbase::ErrorResponse;
        
        // Test mapping of error responses to specific error types
        let invalid_price_response = ErrorResponse {
            message: "Invalid price specified".to_string(),
        };
        let api_error: ApiError = invalid_price_response.into();
        
        match api_error {
            ApiError::InvalidPrice { msg } => {
                assert_eq!(msg, "Invalid price specified");
            }
            _ => panic!("Expected InvalidPrice error"),
        }
        
        let insufficient_funds_response = ErrorResponse {
            message: "Insufficient funds available".to_string(),
        };
        let api_error: ApiError = insufficient_funds_response.into();
        
        match api_error {
            ApiError::InsufficientFunds { msg } => {
                assert_eq!(msg, "Insufficient funds available");
            }
            _ => panic!("Expected InsufficientFunds error"),
        }
    }

    #[test]
    fn test_default_implementations() {
        // Test default implementations
        let default_tif = TimeInForce::default();
        assert_eq!(default_tif, TimeInForce::GoodTillCanceled);
        
        let default_stp = SelfTradePrevention::default();
        assert_eq!(default_stp, SelfTradePrevention::DecrementAndCancel);
        
        let default_rate_limiter = RateLimiter::default();
        // Just verify it can be created without panicking
        drop(default_rate_limiter);
    }

    #[test]
    fn test_account_balance_comprehensive() {
        let json = r#"
        {
            "id": "71452118-efc7-4cc4-8780-a5e22d4baa53",
            "currency": "BTC",
            "balance": "1.100000000000",
            "hold": "0.100000000000",
            "available": "1.000000000000",
            "profile_id": "75da88c5-05bf-4f54-bc85-5c775bd68254",
            "trading_enabled": true
        }"#;

        let account: AccountBalance = serde_json::from_str(json).unwrap();
        
        // Verify all fields
        assert_eq!(account.id, "71452118-efc7-4cc4-8780-a5e22d4baa53");
        assert_eq!(account.currency, "BTC");
        assert_eq!(account.balance, "1.100000000000");
        assert_eq!(account.hold, "0.100000000000");
        assert_eq!(account.available, "1.000000000000");
        assert_eq!(account.profile_id, "75da88c5-05bf-4f54-bc85-5c775bd68254");
        assert!(account.trading_enabled);
        
        // Test serialization round-trip
        let serialized = serde_json::to_string(&account).unwrap();
        let deserialized: AccountBalance = serde_json::from_str(&serialized).unwrap();
        assert_eq!(account.id, deserialized.id);
        assert_eq!(account.currency, deserialized.currency);
    }
}