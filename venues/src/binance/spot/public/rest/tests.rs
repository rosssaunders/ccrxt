use crate::binance::spot::{PublicRestClient, RateLimiter};

#[tokio::test]
async fn test_public_client_creation() {
    let client = reqwest::Client::new();
    let rate_limiter = RateLimiter::new();

    let rest_client = PublicRestClient::new("https://api.binance.com", client, rate_limiter);

    assert_eq!(rest_client.base_url, "https://api.binance.com");
}

#[tokio::test]
async fn test_url_building() {
    let client = reqwest::Client::new();
    let rate_limiter = RateLimiter::new();

    let rest_client = PublicRestClient::new("https://api.binance.com", client, rate_limiter);

    // Test that the client is properly initialized
    assert_eq!(rest_client.base_url, "https://api.binance.com");
}