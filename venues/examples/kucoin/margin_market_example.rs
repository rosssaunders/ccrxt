//! KuCoin Margin Market API Example
//!
//! This example demonstrates how to use the KuCoin margin market endpoints
//! for querying cross margin symbol configuration.
//!
//! This example does not require credentials.
//!
//! Run with:
//!   cargo run --bin margin_market_example --manifest-path venues/examples/kucoin/Cargo.toml

use venues::kucoin::private::rest::{GetMarginSymbolsRequest, RestClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create REST client (no credentials needed for public endpoints)
    let client = RestClient::new(
        "https://api.kucoin.com",
        venues::kucoin::RateLimiter::new(),
        reqwest::Client::new(),
        Box::new(rest::secrets::SecretValue::new(rest::secrets::SecretString::new(Box::<str>::default()))) as Box<dyn rest::secrets::ExposableSecret>,
        Box::new(rest::secrets::SecretValue::new(rest::secrets::SecretString::new(Box::<str>::default()))) as Box<dyn rest::secrets::ExposableSecret>,
        Box::new(rest::secrets::SecretValue::new(rest::secrets::SecretString::new(Box::<str>::default()))) as Box<dyn rest::secrets::ExposableSecret>,
        false,
    );

    println!("🏪 KuCoin Margin Market API Example");
    println!("====================================\n");

    // 1. Get all cross margin symbols
    println!("📊 Getting all cross margin symbols...");
    let request = GetMarginSymbolsRequest { symbol: None };
    match client.get_margin_symbols(request).await {
        Ok((response, _headers)) => {
            println!("✅ Found {} symbols (timestamp: {})", response.items.len(), response.timestamp);
            for (i, symbol) in response.items.iter().take(5).enumerate() {
                println!("   {}. {} ({}): {}-{} | Trading enabled: {}", i + 1, symbol.symbol, symbol.name, symbol.base_currency, symbol.quote_currency, symbol.enable_trading);
            }
        }
        Err(e) => println!("❌ Failed to get margin symbols: {}", e),
    }

    // 2. Get a specific symbol (e.g., BTC-USDT)
    println!("\n🔍 Getting BTC-USDT margin symbol...");
    let request = GetMarginSymbolsRequest { symbol: Some("BTC-USDT".to_string()) };
    match client.get_margin_symbols(request).await {
        Ok((response, _headers)) => {
            if let Some(symbol) = response.items.first() {
                println!("✅ Symbol: {} ({}), Market: {}, Trading enabled: {}", symbol.symbol, symbol.name, symbol.market, symbol.enable_trading);
            } else {
                println!("⚠️  BTC-USDT not found in margin symbols.");
            }
        }
        Err(e) => println!("❌ Failed to get BTC-USDT margin symbol: {}", e),
    }

    println!("\n🎉 Margin Market API example completed!");
    println!("\nKey concepts demonstrated:");
    println!("• 📊 Querying all cross margin symbols");
    println!("• 🔍 Querying a specific cross margin symbol");

    Ok(())
}
