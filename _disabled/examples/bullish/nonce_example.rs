//! Example demonstrating how to use the Bullish public nonce endpoint
//!
//! This example shows how to:
//! - Get the current nonce range from the Bullish exchange
//! - Display the nonce range information

use reqwest::Client;
use venues::bullish::public::RestClient;
use venues::bullish::RateLimiter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize HTTP client
    let client = Client::new();

    // Create rate limiter
    let rate_limiter = RateLimiter::new();

    // Create public REST client
    let public_client = RestClient::new(
        "https://api.exchange.bullish.com",
        client,
        rate_limiter,
    );

    // Get the current nonce range
    println!("Fetching current nonce range...");
    match public_client.get_nonce().await {
        Ok(nonce) => {
            println!("✅ Nonce Range:");
            println!("  Lower Bound: {}", nonce.lower_bound);
            println!("  Upper Bound: {}", nonce.upper_bound);
            println!("  Range Size: {}", nonce.upper_bound - nonce.lower_bound);
        }
        Err(e) => {
            eprintln!("❌ Failed to get nonce range: {}", e);
        }
    }

    Ok(())
}
