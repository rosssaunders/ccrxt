//! KuCoin Margin Risk Limit API Example
//!
//! This example demonstrates how to use the KuCoin margin risk limit endpoint
//! for querying margin risk limit configuration for cross or isolated margin.
//!
//! This example does not require credentials for public endpoints.
//!
//! Run with:
//!   cargo run --bin margin_risklimit_example --manifest-path venues/examples/kucoin/Cargo.toml

use venues::kucoin::private::rest::{GetMarginRiskLimitRequest, RestClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create REST client (no credentials needed for public endpoints)
    let client = RestClient::new(
        "https://api.kucoin.com",
        venues::kucoin::RateLimiter::new(),
        reqwest::Client::new(),
        Box::new(rest::secrets::SecretValue::new(
            rest::secrets::SecretString::new(Box::<str>::default()),
        )) as Box<dyn rest::secrets::ExposableSecret>,
        Box::new(rest::secrets::SecretValue::new(
            rest::secrets::SecretString::new(Box::<str>::default()),
        )) as Box<dyn rest::secrets::ExposableSecret>,
        Box::new(rest::secrets::SecretValue::new(
            rest::secrets::SecretString::new(Box::<str>::default()),
        )) as Box<dyn rest::secrets::ExposableSecret>,
        false,
    );

    println!("🏪 KuCoin Margin Risk Limit API Example");
    println!("====================================\n");

    // 1. Get all margin risk limits (cross margin)
    println!("📊 Getting all cross margin risk limits...");
    let request = GetMarginRiskLimitRequest {
        is_isolated: Some(false),
        currency: None,
        symbol: None,
    };
    match client.get_margin_risk_limit(request).await {
        Ok((response, _headers)) => {
            println!("✅ Found {} risk limit entries", response.data.len());
            for (i, info) in response.data.iter().take(5).enumerate() {
                println!(
                    "   {}. Currency: {:?}, Borrow Max: {:?}, Enabled: {:?}",
                    i + 1,
                    info.currency,
                    info.borrow_max_amount,
                    info.borrow_enabled
                );
            }
        }
        Err(e) => println!("❌ Failed to get margin risk limits: {}", e),
    }

    // 2. Get isolated margin risk limit for a symbol (e.g., BTC-USDT)
    println!("\n🔍 Getting isolated margin risk limit for BTC-USDT...");
    let request = GetMarginRiskLimitRequest {
        is_isolated: Some(true),
        currency: None,
        symbol: Some("BTC-USDT".to_string()),
    };
    match client.get_margin_risk_limit(request).await {
        Ok((response, _headers)) => {
            if let Some(info) = response.data.first() {
                println!(
                    "✅ Symbol: {:?}, Base Max Borrow: {:?}, Quote Max Borrow: {:?}",
                    info.symbol, info.base_max_borrow_amount, info.quote_max_borrow_amount
                );
            } else {
                println!("⚠️  BTC-USDT not found in margin risk limits.");
            }
        }
        Err(e) => println!("❌ Failed to get isolated margin risk limit: {}", e),
    }

    println!("\n🎉 Margin Risk Limit API example completed!");
    println!("\nKey concepts demonstrated:");
    println!("• 📊 Querying all cross margin risk limits");
    println!("• 🔍 Querying isolated margin risk limit for a symbol");

    Ok(())
}
