//! Tests for Binance Options Private REST API

#[cfg(test)]
mod tests {
    use crate::binance::options::{OptionsOrderType, OrderSide, PrivateRestClient, RateLimiter};
    use crate::binance::options::private::rest::{
        AccountRequest, CancelOrderRequest, NewOrderRequest, PositionRequest, UserTradesRequest,
    };
    use reqwest::Client;
    use std::borrow::Cow;

    // Create a simple test secret implementation
    struct TestSecret(String);

    impl rest::secrets::ExposableSecret for TestSecret {
        fn expose_secret(&self) -> String {
            self.0.clone()
        }
    }

    fn create_test_client() -> PrivateRestClient {
        let api_key = Box::new(TestSecret("test_api_key".to_string()));
        let api_secret = Box::new(TestSecret("test_api_secret".to_string()));
        let base_url: Cow<'static, str> = "https://eapi.binance.com".into();
        let rate_limiter = RateLimiter::new();
        let client = Client::new();

        PrivateRestClient::new(api_key, api_secret, base_url, rate_limiter, client)
    }

    #[test]
    fn test_client_creation() {
        let client = create_test_client();
        // Just verify that the client can be created without panicking
        assert_eq!(client.base_url, "https://eapi.binance.com");
    }

    #[test]
    fn test_account_request_creation() {
        let request = AccountRequest::new();
        assert!(request.recv_window.is_none());

        let request_with_window = AccountRequest::new().recv_window(5000);
        assert_eq!(request_with_window.recv_window, Some(5000));
    }

    #[test]
    fn test_new_order_request_creation() {
        let request = NewOrderRequest::new(
            "BTC-200730-9000-C".to_string(),
            OrderSide::Buy,
            OptionsOrderType::Limit,
            "1.0".to_string(),
        );

        assert_eq!(request.symbol, "BTC-200730-9000-C");
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.order_type, OptionsOrderType::Limit);
        assert_eq!(request.quantity, "1.0");
        assert!(request.price.is_none());

        let request_with_price = request.price("2000.0".to_string());
        assert_eq!(request_with_price.price, Some("2000.0".to_string()));
    }

    #[test]
    fn test_cancel_order_request_creation() {
        let request = CancelOrderRequest::new("BTC-200730-9000-C".to_string());
        assert_eq!(request.symbol, "BTC-200730-9000-C");
        assert!(request.order_id.is_none());
        assert!(request.client_order_id.is_none());

        let request_with_order_id = request.order_id(12345);
        assert_eq!(request_with_order_id.order_id, Some(12345));
    }

    #[test]
    fn test_position_request_creation() {
        let request = PositionRequest::new();
        assert!(request.symbol.is_none());

        let request_with_symbol = request.symbol("BTC-200730-9000-C".to_string());
        assert_eq!(
            request_with_symbol.symbol,
            Some("BTC-200730-9000-C".to_string())
        );
    }

    #[test]
    fn test_user_trades_request_creation() {
        let request = UserTradesRequest::new();
        assert!(request.symbol.is_none());
        assert!(request.from_id.is_none());

        let request_with_params = request
            .symbol("BTC-200730-9000-C".to_string())
            .from_id(12345)
            .limit(100);
        assert_eq!(
            request_with_params.symbol,
            Some("BTC-200730-9000-C".to_string())
        );
        assert_eq!(request_with_params.from_id, Some(12345));
        assert_eq!(request_with_params.limit, Some(100));
    }

    #[test]
    fn test_builder_patterns() {
        // Test that all request builders work as expected
        let account_req = AccountRequest::default().recv_window(5000);
        assert_eq!(account_req.recv_window, Some(5000));

        let position_req = PositionRequest::default()
            .symbol("BTC-200730-9000-C".to_string())
            .recv_window(5000);
        assert_eq!(
            position_req.symbol,
            Some("BTC-200730-9000-C".to_string())
        );
        assert_eq!(position_req.recv_window, Some(5000));

        let trades_req = UserTradesRequest::default()
            .symbol("BTC-200730-9000-C".to_string())
            .start_time(1234567890)
            .end_time(1234567999)
            .limit(50);
        assert_eq!(
            trades_req.symbol,
            Some("BTC-200730-9000-C".to_string())
        );
        assert_eq!(trades_req.start_time, Some(1234567890));
        assert_eq!(trades_req.end_time, Some(1234567999));
        assert_eq!(trades_req.limit, Some(50));
    }
}