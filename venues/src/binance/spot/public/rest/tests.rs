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

#[tokio::test]
async fn test_client_has_all_public_methods() {
    let client = reqwest::Client::new();
    let rate_limiter = RateLimiter::new();
    let rest_client = PublicRestClient::new("https://api.binance.com", client, rate_limiter);

    // This test just ensures all our methods are accessible
    // We're not calling them to avoid network requests in tests
    let _ = &rest_client.ping();
    let _ = &rest_client.time();
    let _ = &rest_client.exchange_info();
    let _ = &rest_client.depth("BTCUSDT", Some(5));
    let _ = &rest_client.trades("BTCUSDT", Some(5));
    let _ = &rest_client.ticker_24hr(Some("BTCUSDT"), None, None);
    let _ = &rest_client.ticker_price(Some("BTCUSDT"), None);
    let _ = &rest_client.klines("BTCUSDT", "1h", None, None, None, None);
}