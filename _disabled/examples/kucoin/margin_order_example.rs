//! KuCoin Margin Order API Example
//!
//! This example demonstrates how to use the KuCoin margin order endpoint
//! for placing a margin order (limit or market) in the cross or isolated margin system.
//!
//! Required setup:
//! - Set environment variables:
//!   - KUCOIN_API_KEY: Your KuCoin API key
//!   - KUCOIN_API_SECRET: Your KuCoin API secret
//!   - KUCOIN_PASSPHRASE: Your KuCoin API passphrase
//!
//! Note: This example uses real API endpoints. Ensure you have the necessary
//! credentials and permissions for margin trading on your KuCoin account.
//!
//! Run with:
//!   cargo run --bin margin_order_example --manifest-path venues/examples/kucoin/Cargo.toml

use std::env;

use rest::secrets::SecretString;
use venues::kucoin::private::rest::{
    AddMarginOrderRequest, MarginOrderSide, MarginOrderTimeInForce, MarginOrderType, RestClient,
};

fn uuid() -> String {
    // Simple UUID generator for clientOid (not cryptographically secure)
    use rand::{Rng, distributions::Alphanumeric};
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client with credentials from environment variables
    let api_key = env::var("KUCOIN_API_KEY").expect("KUCOIN_API_KEY not set");
    let api_secret = env::var("KUCOIN_API_SECRET").expect("KUCOIN_API_SECRET not set");
    let passphrase = env::var("KUCOIN_PASSPHRASE").expect("KUCOIN_PASSPHRASE not set");

    // Create REST client for private API using SecretString and boxing
    let client = RestClient::new_with_credentials(
        Box::new(SecretString::new(api_key.into())) as Box<dyn rest::secrets::ExposableSecret>,
        Box::new(SecretString::new(api_secret.into())) as Box<dyn rest::secrets::ExposableSecret>,
        Box::new(SecretString::new(passphrase.into())) as Box<dyn rest::secrets::ExposableSecret>,
    );

    println!("🏪 KuCoin Margin Order API Example");
    println!("====================================\n");

    // 1. Place a margin limit order (simulation)
    println!("📝 Placing margin limit order (simulation)");
    let order_request = AddMarginOrderRequest {
        client_oid: uuid(),
        side: MarginOrderSide::Buy,
        symbol: "BTC-USDT".to_string(),
        r#type: Some(MarginOrderType::Limit),
        stp: None,
        price: Some("30000.0".to_string()),
        size: Some("0.001".to_string()),
        time_in_force: Some(MarginOrderTimeInForce::GoodTillCancelled),
        post_only: Some(true),
        hidden: Some(false),
        iceberg: Some(false),
    };
    println!("   Symbol: {}", order_request.symbol);
    println!("   Side: {:?}", order_request.side);
    println!("   Type: {:?}", order_request.r#type);
    println!("   Price: {:?}", order_request.price);
    println!("   Size: {:?}", order_request.size);
    // Uncomment to actually place the order (requires sufficient margin and permissions)
    /*
    match client.add_margin_order(order_request).await {
        Ok((response, _headers)) => {
            println!("✅ Margin order placed! Order ID: {}", response.order_id);
        }
        Err(e) => println!("❌ Failed to place margin order: {}", e),
    }
    */

    println!("\n🎉 Margin Order API example completed!");
    println!("\nKey concepts demonstrated:");
    println!("• 📝 Placing a margin order (limit or market)");
    println!("• Using clientOid for idempotency");
    println!("• Setting order parameters (side, type, price, size, etc.)");
    println!(
        "\n⚠️  Remember: Margin trading involves risks!\n   Always check market conditions and your account status before placing orders."
    );

    Ok(())
}
