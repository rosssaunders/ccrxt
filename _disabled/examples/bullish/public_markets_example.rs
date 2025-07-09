//! Example demonstrating how to get market information from Bullish Exchange
//!
//! This example shows how to:
//! 1. Create a public REST client
//! 2. Fetch all available markets
//! 3. Get detailed information for a specific market
//! 4. Handle API responses

use venues::bullish::public::RestClient;
use venues::bullish::RateLimiter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create HTTP client and rate limiter
    let http_client = reqwest::Client::new();
    let rate_limiter = RateLimiter::new();

    // Create public REST client
    let client = RestClient::new(
        "https://api.exchange.bullish.com",
        http_client,
        rate_limiter,
    );

    println!("🚀 Getting all markets from Bullish Exchange...");

    // Get all markets
    let markets_response = client.get_markets().await?;
    println!("📊 Found {} markets", markets_response.data.len());

    // Display first few markets
    for (i, market) in markets_response.data.iter().take(5).enumerate() {
        println!(
            "{}. {} ({}) - Status: {:?}, Trading: {}",
            i + 1,
            market.symbol,
            market.display_name,
            market.status,
            market.trading_enabled
        );
    }

    // Get detailed information for a specific market (e.g., BTCUSDC)
    if let Some(btc_market) = markets_response.data.iter().find(|m| m.symbol == "BTCUSDC") {
        println!("\n🔍 Getting detailed information for BTCUSDC...");
        let market_detail = client.get_market("BTCUSDC").await?;
        
        println!("Market Details:");
        println!("  Symbol: {}", market_detail.data.symbol);
        println!("  Type: {:?}", market_detail.data.market_type);
        println!("  Min Order Qty: {}", market_detail.data.min_order_qty);
        println!("  Price Increment: {}", market_detail.data.price_increment);
        println!("  Maker Fee: {}", market_detail.data.maker_fee_rate);
        println!("  Taker Fee: {}", market_detail.data.taker_fee_rate);
        
        if let Some(last_price) = &market_detail.data.last_price {
            println!("  Last Price: ${}", last_price);
        }
        
        if let Some(volume_24h) = &market_detail.data.volume_24h {
            println!("  24h Volume: {}", volume_24h);
        }
    }

    Ok(())
}
