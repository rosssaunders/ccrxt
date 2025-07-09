//! Integration tests for Binance Spot private client accessibility
//! 
//! This module contains integration tests that verify the public API access
//! to the Binance Spot private REST client functionality.

use venues::binance::spot::PrivateRestClient;
use venues::binance::spot::RateLimiter;
use rest::secrets::ExposableSecret;
use reqwest::Client;

// Test secret implementation
#[derive(Clone)]
struct TestSecret {
    value: String,
}

impl ExposableSecret for TestSecret {
    fn expose_secret(&self) -> String {
        self.value.clone()
    }
}

impl TestSecret {
    fn new(value: String) -> Self {
        Self { value }
    }
}

#[test]
fn test_public_api_access() {
    // Test that we can create the PrivateRestClient through the public API
    let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
    let api_secret = Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
    let client = Client::new();
    let rate_limiter = RateLimiter::new();

    let _rest_client = PrivateRestClient::new(
        api_key,
        api_secret,
        "https://api.binance.com",
        rate_limiter,
        client,
    );

    // If we get here, the client was created successfully through the public API
    assert!(true);
}