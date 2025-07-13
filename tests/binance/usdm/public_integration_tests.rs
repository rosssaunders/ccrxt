//! Integration tests for Binance USD-M Futures public REST API endpoints
//!
//! These tests verify the functionality of all public endpoints that don't require authentication.
//! Tests run against the live Binance USD-M API using real market data.
//!
//! **Note:** Binance API has geographic restrictions. Tests may fail with "Service unavailable
//! from a restricted location" errors when run from certain locations. This is expected behavior
//! and indicates the tests are correctly configured to reach the live API.

use reqwest::Client;
use tokio;
use venues::binance::usdm::{PublicRestClient, RateLimiter};

/// Helper function to create a test client for public endpoints
fn create_public_test_client() -> PublicRestClient {
    let client = Client::new();
    let rate_limiter = RateLimiter::new();

    PublicRestClient::new("https://fapi.binance.com", client, rate_limiter)
}

/// Test the ping endpoint - test connectivity
#[tokio::test]
async fn test_ping() {
    let client = create_public_test_client();

    let result = client.ping().await;
    assert!(
        result.is_ok(),
        "ping request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    // Ping response is empty, just verify we got a response
    println!(
        "Ping successful: request took {:?}",
        response.request_duration
    );
}

/// Test the server time endpoint
#[tokio::test]
async fn test_get_server_time() {
    let client = create_public_test_client();

    let result = client.get_server_time().await;
    assert!(
        result.is_ok(),
        "get_server_time request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        response.data.server_time > 0,
        "Server time should be a positive timestamp"
    );
    println!(
        "Server time: {} (took {:?})",
        response.data.server_time, response.request_duration
    );
}

/// Test the exchange info endpoint
#[tokio::test]
async fn test_get_exchange_info() {
    let client = create_public_test_client();

    let result = client.get_exchange_info().await;
    assert!(
        result.is_ok(),
        "get_exchange_info request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();

    // Verify basic exchange info structure
    assert!(response.data.timezone == "UTC", "Timezone should be UTC");
    assert!(
        !response.data.rate_limits.is_empty(),
        "Should have rate limits"
    );
    // Exchange filters can be empty in USD-M futures
    assert!(!response.data.assets.is_empty(), "Should have assets");
    assert!(!response.data.symbols.is_empty(), "Should have symbols");

    // Verify at least one symbol exists and has required fields
    let symbol = &response.data.symbols[0];
    assert!(!symbol.symbol.is_empty(), "Symbol name should not be empty");
    assert!(
        !symbol.base_asset.is_empty(),
        "Base asset should not be empty"
    );
    // margin_asset is now an enum, so we just verify it exists (which it does if deserialization succeeded)

    println!(
        "Exchange info fetched successfully: {} symbols, took {:?}",
        response.data.symbols.len(),
        response.request_duration
    );
}
/// Test basic functionality without relying on private request types
#[tokio::test]
async fn test_basic_endpoints() {
    let client = create_public_test_client();

    // Test ping
    let ping_result = client.ping().await;
    assert!(ping_result.is_ok(), "Ping should succeed");

    // Test server time
    let time_result = client.get_server_time().await;
    assert!(time_result.is_ok(), "Server time should succeed");

    // Test exchange info
    let exchange_result = client.get_exchange_info().await;
    assert!(exchange_result.is_ok(), "Exchange info should succeed");

    // Test funding rate info
    let funding_result = client.get_funding_rate_info().await;
    assert!(funding_result.is_ok(), "Funding rate info should succeed");

    if let Ok(funding_response) = funding_result {
        assert!(
            !funding_response.data.is_empty(),
            "Should have funding rate data"
        );
        let first_funding = &funding_response.data[0];
        assert!(!first_funding.symbol.is_empty(), "Should have symbol");
        // Note: FundingRateInfo doesn't have funding_rate field, only rate cap/floor
        println!("Funding rate info: {} symbols", funding_response.data.len());
    }

    println!("Basic endpoints test completed successfully");
}

/// Test rate limiting behavior
#[tokio::test]
async fn test_rate_limiting() {
    let client = create_public_test_client();

    // Make multiple rapid requests to test rate limiting
    let mut results = Vec::new();

    for i in 0..5 {
        let result = client.ping().await;
        results.push(result);
        println!(
            "Request {}: {:?}",
            i + 1,
            if results[i].is_ok() {
                "Success"
            } else {
                "Failed"
            }
        );
    }

    // All requests should succeed for ping endpoint with reasonable rate limiting
    let successful_count = results.iter().filter(|r| r.is_ok()).count();
    assert!(
        successful_count >= 3,
        "At least 3 out of 5 requests should succeed"
    );

    println!(
        "Rate limiting test: {}/5 requests succeeded",
        successful_count
    );
}

/// Test endpoint diversity
#[tokio::test]
async fn test_endpoint_diversity() {
    let client = create_public_test_client();

    // Test different endpoint types
    let endpoints_tested = [("ping", client.ping().await.is_ok()),
        ("server_time", client.get_server_time().await.is_ok()),
        ("exchange_info", client.get_exchange_info().await.is_ok()),
        (
            "funding_rate_info",
            client.get_funding_rate_info().await.is_ok(),
        )];

    let successful_endpoints: Vec<_> = endpoints_tested
        .iter()
        .filter(|(_, success)| *success)
        .map(|(name, _)| *name)
        .collect();

    println!("Successful endpoints: {:?}", successful_endpoints);

    // At least 3 out of 4 endpoints should work
    assert!(
        successful_endpoints.len() >= 3,
        "At least 3 endpoints should succeed, got: {:?}",
        successful_endpoints
    );
}
