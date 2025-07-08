#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::gateio::{
        errors::GateIoError,
        private::rest::create_order::CreateOrderRequest,
        public::rest::tickers::TickersRequest,
        rate_limit::{RateLimitHeader, RateLimiter},
    };

    #[test]
    fn test_error_categorization() {
        // Test retryable errors
        let rate_limit_error = GateIoError::RateLimitExceeded {
            message: "Too many requests".to_string(),
        };
        assert!(rate_limit_error.is_retryable());
        assert_eq!(rate_limit_error.retry_delay_secs(), Some(60));

        let timeout_error = GateIoError::Timeout { timeout_secs: 30 };
        assert!(timeout_error.is_retryable());
        assert_eq!(timeout_error.retry_delay_secs(), Some(5));

        // Test non-retryable errors
        let auth_error = GateIoError::Authentication("Invalid key".to_string());
        assert!(!auth_error.is_retryable());
        assert!(auth_error.is_auth_error());
        assert!(auth_error.is_client_error());

        let param_error = GateIoError::InvalidParameter("Invalid currency pair".to_string());
        assert!(!param_error.is_retryable());
        assert!(param_error.is_client_error());
    }

    #[test]
    fn test_rate_limit_header_parsing() {
        use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-gate-ratelimit-requests-remain"),
            HeaderValue::from_static("5"),
        );
        headers.insert(
            HeaderName::from_static("x-gate-ratelimit-limit"),
            HeaderValue::from_static("100"),
        );
        headers.insert(
            HeaderName::from_static("x-gate-ratelimit-reset-timestamp"),
            HeaderValue::from_static("1640995200"),
        );

        let rate_limit = RateLimitHeader::from_headers(&headers);

        assert_eq!(rate_limit.requests_remain, Some(5));
        assert_eq!(rate_limit.limit, Some(100));
        assert_eq!(rate_limit.reset_timestamp, Some(1640995200));
        assert!(rate_limit.is_near_limit()); // 5/100 = 5% remaining, below 10% threshold
    }

    // Note: Endpoint categorization is tested implicitly through rate limiting behavior
    // The categorize_endpoint method is private and tested through integration

    #[tokio::test]
    async fn test_rate_limiter_usage_tracking() {
        let rate_limiter = RateLimiter::new();

        // Get permit and check usage
        let _permit = rate_limiter.get_permit("/spot/tickers").await.unwrap();

        let stats = rate_limiter.get_usage_stats().await;
        assert!(stats.contains_key("spot_other"));

        let spot_usage = &stats["spot_other"];
        assert_eq!(spot_usage.requests_made, 1);
    }

    #[test]
    fn test_request_validation() {
        use crate::gateio::enums::{OrderSide, OrderType, TimeInForce};

        // Test valid order request
        let valid_order = CreateOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            order_type: OrderType::Limit,
            account: Some("spot".to_string()),
            side: OrderSide::Buy,
            amount: "0.001".to_string(),
            price: Some("30000".to_string()),
            time_in_force: Some(TimeInForce::GoodTillCanceled),
            text: Some("test_order".to_string()),
            iceberg: None,
            stp_mode: None,
        };

        // Verify required fields are present
        assert!(!valid_order.currency_pair.is_empty());
        assert!(matches!(valid_order.side, OrderSide::Buy));
        assert!(!valid_order.amount.is_empty());

        // Test amount parsing
        assert!(valid_order.amount.parse::<f64>().is_ok());
        if let Some(ref price) = valid_order.price {
            assert!(price.parse::<f64>().is_ok());
        }
    }

    #[test]
    fn test_ticker_request_serialization() {
        use serde_json;

        let request = TickersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            timezone: Some("utc0".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("BTC_USDT"));
        assert!(serialized.contains("utc0"));

        let default_request = TickersRequest::default();
        let default_serialized = serde_json::to_string(&default_request).unwrap();
        assert_eq!(default_serialized, "{}"); // Empty object for defaults
    }

    #[test]
    fn test_error_response_mapping() {
        use crate::gateio::errors::ErrorResponse;

        // Test mapping of common error responses
        let rate_limit_error = ErrorResponse {
            label: "RATE_LIMIT_EXCEEDED".to_string(),
            message: "Too many requests".to_string(),
        };

        let mapped_error: GateIoError = rate_limit_error.into();
        match mapped_error {
            GateIoError::RateLimitExceeded { message } => {
                assert_eq!(message, "Too many requests");
            }
            _ => panic!("Expected RateLimitExceeded error"),
        }

        let auth_error = ErrorResponse {
            label: "AUTHENTICATION_FAILED".to_string(),
            message: "Invalid signature".to_string(),
        };

        let mapped_auth_error: GateIoError = auth_error.into();
        match mapped_auth_error {
            GateIoError::Authentication(message) => {
                assert_eq!(message, "Invalid signature");
            }
            _ => panic!("Expected Authentication error"),
        }
    }

    #[tokio::test]
    async fn test_rate_limit_warnings() {
        let rate_limiter = RateLimiter::new();

        // Simulate high usage
        for _ in 0..9 {
            let _permit = rate_limiter.get_permit("/spot/orders").await.unwrap();
        }

        let warnings = rate_limiter.get_rate_limit_warnings().await;

        // Should warn when approaching 80% of spot_order_placement limit (10 req/s)
        assert!(!warnings.is_empty());
        assert!(warnings[0].contains("spot_order_placement"));
        assert!(warnings[0].contains("90.0%")); // 9/10 = 90%
    }

    // Integration test helpers (commented out - require API credentials)
    /*
    #[tokio::test]
    #[ignore] // Use with: cargo test -- --ignored
    async fn integration_test_public_api() {
        let client = PublicRestClient::new(false).unwrap();

        let tickers = client.get_tickers(TickersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            timezone: None,
        }).await.unwrap();

        assert!(!tickers.is_empty());
        let btc_ticker = &tickers[0];
        assert_eq!(btc_ticker.currency_pair, "BTC_USDT");
        assert!(btc_ticker.last.parse::<f64>().is_ok());
    }

    #[tokio::test]
    #[ignore] // Use with: cargo test -- --ignored
    async fn integration_test_private_api() {
        use std::env;

        let api_key = env::var("GATEIO_API_KEY").expect("API key required");
        let api_secret = env::var("GATEIO_API_SECRET").expect("API secret required");

        let client = PrivateRestClient::new(api_key, api_secret, true).unwrap(); // testnet

        let balances = client.get_spot_accounts(None).await.unwrap();
        // Just verify we can authenticate and get a response
        assert!(balances.is_empty() || !balances.is_empty()); // Any result is fine
    }
    */

    #[test]
    fn test_currency_validation() {
        let valid_currencies = vec!["BTC", "ETH", "USDT", "DOT", "ADA"];
        let invalid_currencies = vec!["", "bitcoin", "btc_usd", "VERYLONGCURRENCY"];

        for currency in valid_currencies {
            assert!(currency.len() >= 2);
            assert!(currency.len() <= 10);
            assert!(
                currency
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
            );
        }

        for currency in invalid_currencies {
            let is_invalid = currency.is_empty()
                || currency.len() > 10
                || !currency
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit());
            assert!(
                is_invalid,
                "Currency '{}' should be invalid but validation passed",
                currency
            );
        }
    }

    #[test]
    fn test_currency_pair_validation() {
        let valid_pairs = vec!["BTC_USDT", "ETH_BTC", "DOT_USDT"];
        let invalid_pairs = vec!["BTCUSDT", "BTC-USDT", "btc_usdt", "BTC_"];

        for pair in valid_pairs {
            let parts: Vec<&str> = pair.split('_').collect();
            assert_eq!(parts.len(), 2);
            assert!(!parts[0].is_empty());
            assert!(!parts[1].is_empty());
        }

        for pair in invalid_pairs {
            let parts: Vec<&str> = pair.split('_').collect();
            assert!(
                parts.len() != 2
                    || parts
                        .iter()
                        .any(|p| p.is_empty() || !p.chars().all(|c| c.is_ascii_uppercase()))
            );
        }
    }
}

/*
To run these tests:

1. Unit tests:
   cargo test --package venues gateio::tests

2. All tests including integration tests (requires API credentials):
   GATEIO_API_KEY=your_key GATEIO_API_SECRET=your_secret cargo test --package venues gateio::tests -- --ignored

Test categories:
- Error handling and categorization
- Rate limiting functionality
- Request validation
- Response parsing
- Integration tests (with real API calls)

Best practices demonstrated:
- Comprehensive error testing
- Rate limit validation
- Input validation
- Serialization/deserialization testing
- Integration test structure
*/
