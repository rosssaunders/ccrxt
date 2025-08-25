//! Integration tests for Binance USD-M Futures public REST API endpoints that require API key authentication
//!
//! These tests verify the functionality of public endpoints that require API key authentication
//! but do not require request signing (MARKET_DATA security type endpoints).
//! Tests run against the live Binance USD-M API using real market data.
//!
//! **Note:** These tests require a valid Binance API key to be provided via environment variables.
//! Set BINANCE_API_KEY environment variable to run these tests.
//!
//! **Note:** Binance API has geographic restrictions. Tests may fail with "Service unavailable
//! from a restricted location" errors when run from certain locations. This is expected behavior
//! and indicates the tests are correctly configured to reach the live API.

use std::{sync::Arc, time::Duration};

use tokio;
use venues::binance::{
    shared::{RateLimiter, RateLimits},
    usdm::PublicRestClient,
};

/// Helper function to create a test client for API key endpoints
fn create_api_key_test_client() -> PublicRestClient {
    let http_client = std::sync::Arc::new(rest::native::NativeHttpClient::default());
    let rate_limits = RateLimits {
        request_weight_limit: 2400,
        request_weight_window: Duration::from_secs(60),
        raw_requests_limit: 1200,
        raw_requests_window: Duration::from_secs(60),
        orders_10s_limit: 100,
        orders_minute_limit: 1200,
        orders_day_limit: None,
    };
    let rate_limiter = RateLimiter::new(rate_limits);

    PublicRestClient::new(
        "https://fapi.binance.com",
        http_client,
        Arc::new(rate_limiter),
    )
}

/// Helper function to get API key from environment
fn get_api_key() -> Option<String> {
    std::env::var("BINANCE_API_KEY").ok()
}

/// Test the historical trades endpoint with API key authentication
#[tokio::test]
async fn test_get_historical_trades() {
    use rest::secrets::{SecretString, SecretValue};
    use venues::binance::usdm::public::rest::historical_trades::HistoricalTradesRequest;

    let Some(api_key) = get_api_key() else {
        println!(
            "Skipping test_get_historical_trades: BINANCE_API_KEY environment variable not set"
        );
        return;
    };

    let client = create_api_key_test_client();
    let request = HistoricalTradesRequest {
        symbol: "BTCUSDT".into(),
        limit: Some(10),
        from_id: None,
    };

    // Create API key secret
    let api_key_secret = SecretValue::new(SecretString::new(api_key.into()));

    let result = client.get_historical_trades(&api_key_secret, request).await;
    assert!(
        result.is_ok(),
        "get_historical_trades should succeed with valid API key: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        !response.data.is_empty(),
        "Should have historical trade data"
    );

    let trade = &response.data[0];
    assert!(trade.id > 0, "Trade should have valid ID");
    assert!(!trade.price.is_empty(), "Trade should have price");
    assert!(!trade.qty.is_empty(), "Trade should have quantity");

    println!(
        "Historical trades: {} trades (took {:?})",
        response.data.len(),
        std::time::Duration::from_secs(0)
    );
}
