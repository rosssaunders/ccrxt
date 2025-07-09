//! BitMart WebSocket API example
//!
//! This example demonstrates how to use the BitMart WebSocket API for real-time market data.

use std::time::{SystemTime, UNIX_EPOCH};

use venues::bitmart::{
    PublicWebSocketClient, PrivateWebSocketClient, PublicChannel, DepthLevel, WsMessage,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();

    println!("=== BitMart WebSocket API Example ===");

    // Example 1: Public WebSocket streams
    println!("\n--- Public WebSocket Examples ---");
    
    let public_client = PublicWebSocketClient::new();
    println!("Public WebSocket URL: {}", public_client.url());

    // Create subscription messages for different data types
    let ticker_msg = public_client.subscribe_ticker("BTC_USDT");
    println!("Ticker subscription message: {}", serde_json::to_string_pretty(&ticker_msg)?);

    let depth_msg = public_client.subscribe_depth("BTC_USDT", DepthLevel::Level5);
    println!("Depth subscription message: {}", serde_json::to_string_pretty(&depth_msg)?);

    // Subscribe to multiple tickers
    let multi_ticker_msg = public_client.subscribe_tickers(vec!["BTC_USDT", "ETH_USDT", "LTC_USDT"]);
    println!("Multi-ticker subscription: {}", serde_json::to_string_pretty(&multi_ticker_msg)?);

    // Example 2: Private WebSocket streams
    println!("\n--- Private WebSocket Examples ---");

    // NOTE: These are example credentials - replace with your actual API credentials
    let api_key = std::env::var("BITMART_API_KEY")
        .unwrap_or_else(|_| "your_api_key_here".to_string());
    let api_secret = std::env::var("BITMART_API_SECRET")
        .unwrap_or_else(|_| "your_api_secret_here".to_string());
    let memo = std::env::var("BITMART_MEMO")
        .unwrap_or_else(|_| "your_memo_here".to_string());

    if api_key == "your_api_key_here" {
        println!("⚠️  Private WebSocket example skipped - no credentials provided");
        println!("   Set BITMART_API_KEY, BITMART_API_SECRET, and BITMART_MEMO environment variables to test private streams");
        return Ok(());
    }

    let private_client = PrivateWebSocketClient::new(api_key, api_secret, memo);
    println!("Private WebSocket URL: {}", private_client.url());

    // Create login message
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis() as u64;
    
    let login_msg = private_client.login_message(timestamp)?;
    println!("Login message: {}", serde_json::to_string_pretty(&login_msg)?);

    // Create subscription messages for private channels
    let order_msg = private_client.subscribe_orders();
    println!("Order updates subscription: {}", serde_json::to_string_pretty(&order_msg)?);

    let asset_msg = private_client.subscribe_assets();
    println!("Asset updates subscription: {}", serde_json::to_string_pretty(&asset_msg)?);

    println!("\n--- Example Usage Patterns ---");
    println!("1. Connect to WebSocket endpoint");
    println!("2. For private channels: Send login message and wait for confirmation");
    println!("3. Send subscription messages for desired channels");
    println!("4. Process incoming data messages");
    println!("5. Handle reconnection logic as needed");

    println!("\nExample complete! This demonstrates message creation.");
    println!("For a full WebSocket implementation, you would:");
    println!("  - Establish WebSocket connection using tokio-tungstenite");
    println!("  - Send these messages over the connection");
    println!("  - Parse incoming responses using the WsResponse enum");
    println!("  - Handle different message types (events, data, errors)");

    Ok(())
}
