//! Example demonstrating how to use the GetRecentTradesRequest without builder methods.

use venues::bitget::public::rest::{GetRecentTradesRequest, RestClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create request using struct syntax instead of builder pattern
    let request = GetRecentTradesRequest {
        symbol: "BTCUSDT".to_string(),
        limit: Some(10),
    };

    println!("Request created: symbol={}, limit={:?}", request.symbol, request.limit);

    // Note: Actual API call would require a properly configured client
    // let client = RestClient::new(
    //     "https://api.bitget.com",
    //     RateLimiter::new(),
    //     reqwest::Client::new()
    // );
    // let response = client.get_recent_trades(&request).await?;

    Ok(())
}
