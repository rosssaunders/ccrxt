//! Integration tests for Gate.io unified REST API endpoints
//!
//! These tests verify the functionality of unified trading endpoints.
//! Tests run against the live Gate.io API using real market data.

use std::sync::Arc;

use rest::native::NativeHttpClient;
use tokio;
use venues::gateio::unified::public::rest::RestClient;

/// Helper function to create a test client for unified public endpoints
fn create_unified_test_client() -> RestClient {
    let http_client = Arc::new(NativeHttpClient::default());
    RestClient::new(http_client, false).expect("Failed to create Gate.io unified REST client")
}

#[tokio::test]
async fn test_unified_client_creation() {
    let _client = create_unified_test_client();
    println!("✓ Unified client creation successful");
}
