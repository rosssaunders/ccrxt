//! Integration tests for Gate.io perpetual/futures REST API endpoints
//!
//! These tests verify the functionality of perpetual futures endpoints.
//! Tests run against the live Gate.io API using real market data.

use tokio;
use venues::gateio::perpetual::public::rest::RestClient;

/// Helper function to create a test client for perpetual public endpoints
fn create_perpetual_test_client() -> RestClient {
    RestClient::new(false).expect("Failed to create Gate.io perpetual REST client")
}

#[tokio::test]
async fn test_perpetual_client_creation() {
    let _client = create_perpetual_test_client();
    println!("✓ Perpetual client creation successful");
}

// TODO: Add more perpetual-specific integration tests
// Examples:
// - test_get_futures_contracts
// - test_get_futures_tickers
// - test_get_futures_order_book
// - test_get_futures_trades
// - test_get_futures_funding_rate
