//! Example demonstrating how to use the Bullish public index prices endpoints
//!
//! This example shows how to:
//! - Get all index prices from the Bullish exchange
//! - Get index price for a specific asset symbol
//! - Display the index price information

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

    // Get all index prices
    println!("Fetching all index prices...");
    match public_client.get_index_prices().await {
        Ok(index_prices) => {
            println!("✅ Index Prices ({} assets):", index_prices.len());
            for (i, price) in index_prices.iter().take(5).enumerate() {
                println!(
                    "  {}. {}: ${} (Updated: {})",
                    i + 1,
                    price.asset_symbol,
                    price.price,
                    price.updated_at_datetime
                );
            }
            if index_prices.len() > 5 {
                println!("  ... and {} more", index_prices.len() - 5);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get index prices: {}", e);
        }
    }

    println!();

    // Get index price for BTC specifically
    println!("Fetching BTC index price...");
    match public_client.get_index_price_by_symbol("BTC").await {
        Ok(btc_price) => {
            println!("✅ BTC Index Price:");
            println!("  Symbol: {}", btc_price.asset_symbol);
            println!("  Price: ${}", btc_price.price);
            println!("  Updated: {}", btc_price.updated_at_datetime);
            println!("  Timestamp: {}", btc_price.updated_at_timestamp);
        }
        Err(e) => {
            eprintln!("❌ Failed to get BTC index price: {}", e);
        }
    }

    Ok(())
}
