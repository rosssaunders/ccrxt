use reqwest::Client;
use tokio;
use venues::deribit::{AccountTier, GetTradeVolumesRequest, PublicRestClient, RateLimiter};

/// Helper function to create a test client for public endpoints
fn create_public_test_client() -> PublicRestClient {
    let client = Client::new();
    let rate_limiter = RateLimiter::new(AccountTier::Tier1);

    PublicRestClient::new("https://www.deribit.com", client, rate_limiter)
}

/// Test the get_trade_volumes endpoint
#[tokio::test]
async fn test_get_trade_volumes() {
    let client = create_public_test_client();

    let request = GetTradeVolumesRequest {};

    let result = client.get_trade_volumes(request).await;
    assert!(
        result.is_ok(),
        "get_trade_volumes request should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.id > 0);

    println!("Found {} trade volume entries", response.result.len());

    // Validate trade volume data
    for (i, volume) in response.result.iter().take(3).enumerate() {
        assert!(
            volume.futures_volume >= 0.0,
            "futures volume should be non-negative"
        );
        println!(
            "Trade volume {}: currency={:?}, currency_pair={}, futures_volume={}, calls_volume={}, puts_volume={}, spot_volume={}",
            i,
            volume.currency,
            volume.currency_pair,
            volume.futures_volume,
            volume.calls_volume,
            volume.puts_volume,
            volume.spot_volume
        );
    }
}
