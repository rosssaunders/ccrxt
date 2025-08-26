//! Example demonstrating WebSocket rate limiting for Binance

use std::time::Duration;
use tokio::time::sleep;
use venues::binance::spot::websocket::BinanceSpotWebSocketClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Binance WebSocket Rate Limiting Example\n");

    // Create a new WebSocket client
    let mut client = BinanceSpotWebSocketClient::new();

    // Connect to WebSocket
    println!("Connecting to Binance WebSocket...");
    client.connect().await?;
    println!("Connected successfully!\n");

    // Get initial rate limit stats
    let stats = client.get_rate_limit_stats().await;
    println!("Initial Rate Limit Stats:");
    println!("  Messages in last second: {}/{}", 
        stats.messages_in_last_second, 
        stats.max_messages_per_second
    );
    println!("  Active subscriptions: {}/{}", 
        stats.active_subscriptions, 
        stats.max_subscriptions
    );
    println!("  Connections in window: {}/{}\n", 
        stats.connections_in_window, 
        stats.max_connections_per_window
    );

    // Subscribe to some streams
    println!("Subscribing to BTC/USDT trades...");
    client.subscribe_trades("BTCUSDT").await?;
    
    println!("Subscribing to ETH/USDT trades...");
    client.subscribe_trades("ETHUSDT").await?;

    // Get updated stats
    let stats = client.get_rate_limit_stats().await;
    println!("\nUpdated Rate Limit Stats after subscriptions:");
    println!("  Messages in last second: {}/{}", 
        stats.messages_in_last_second, 
        stats.max_messages_per_second
    );
    println!("  Active subscriptions: {}/{}\n", 
        stats.active_subscriptions, 
        stats.max_subscriptions
    );

    // Try to exceed message rate limit
    println!("Testing message rate limiting (5 msg/sec limit)...");
    let symbols = ["BNBUSDT", "ADAUSDT", "DOGEUSDT", "MATICUSDT", "SOLUSDT", "DOTUSDT", "AVAXUSDT"];
    for (i, symbol) in symbols.iter().enumerate() {
        match client.subscribe_trades(symbol).await {
            Ok(_) => println!("  Message {}: Sent successfully ({})", i + 1, symbol),
            Err(e) => {
                println!("  Message {}: Rate limited! {}", i + 1, e);
                break;
            }
        }
    }

    // Wait for rate limit to reset
    println!("\nWaiting 1 second for rate limit to reset...");
    sleep(Duration::from_secs(1)).await;

    // Check stats again
    let stats = client.get_rate_limit_stats().await;
    println!("Rate Limit Stats after waiting:");
    println!("  Messages in last second: {}/{}", 
        stats.messages_in_last_second, 
        stats.max_messages_per_second
    );

    // Disconnect
    println!("\nDisconnecting...");
    client.disconnect().await?;
    
    // Check that connection state was reset
    let stats = client.get_rate_limit_stats().await;
    println!("Rate Limit Stats after disconnect:");
    println!("  Active subscriptions: {} (should be 0)", stats.active_subscriptions);
    println!("  Messages in last second: {} (should be 0)", stats.messages_in_last_second);
    
    println!("\nExample completed successfully!");
    Ok(())
}