//! Integration tests for Gate.io unified REST API endpoints
//!
//! These tests verify the functionality of unified trading endpoints.
//! Tests run against the live Gate.io API using real market data.

use std::sync::Arc;

use rest::native::NativeHttpClient;
use tokio;
use venues::gateio::{PublicRestClient};

/// Helper function to create a test client for unified public endpoints
fn create_unified_test_client() -> PublicRestClient {
    let http_client = Arc::new(NativeHttpClient::default());
    let rate_limiter = Arc::new(venues::gateio::RateLimiter::default());
    PublicRestClient::new(http_client, rate_limiter, false).expect("Failed to create Gate.io unified REST client")
}

#[tokio::test]
async fn test_unified_client_creation() {
    let _client = create_unified_test_client();
    println!("✓ Unified client creation successful");
}
