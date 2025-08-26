use std::time::Duration;

use futures::StreamExt;
use tokio::time::sleep;
use venues::binance::spot::websocket::{
    BinanceSpotWebSocketClient, DepthLevel, UpdateSpeed,
};
use websockets::WebSocketEvent;

/// Example demonstrating Binance Spot depth (orderbook) WebSocket stream
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Binance Spot Depth Stream Example");
    println!("==================================\n");
    
    // Create WebSocket client
    let mut client = BinanceSpotWebSocketClient::new();
    
    // Connect
    println!("Connecting to Binance WebSocket...");
    client.connect().await?;
    println!("✅ Connected\n");
    
    // Subscribe to BTC/USDT depth updates (5 levels, 100ms updates)
    println!("Subscribing to BTCUSDT depth stream...");
    client.subscribe_book_depth("BTCUSDT", DepthLevel::Five, UpdateSpeed::Fast).await?;
    println!("✅ Subscribed to BTCUSDT@depth5@100ms\n");
    
    // Also subscribe to diff depth for more detailed updates
    client.subscribe_diff_depth("BTCUSDT", UpdateSpeed::Fast).await?;
    println!("✅ Subscribed to BTCUSDT@depth@100ms\n");
    
    println!("🎧 Listening for depth updates...\n");
    
    // Get event stream
    let mut events = client.event_stream();
    let mut message_count = 0;
    let max_messages = 20; // Show first 20 depth updates
    
    // Process events
    while let Some(event) = events.next().await {
        match event {
            WebSocketEvent::Message { message } => {
                // Print raw message for debugging
                let json = serde_json::to_string_pretty(&message).unwrap_or_else(|_| "Failed to serialize".to_string());
                println!("📦 Raw message:\n{}\n", json);
                
                message_count += 1;
                if message_count >= max_messages {
                    println!("\n✅ Received {} depth updates. Disconnecting...", message_count);
                    break;
                }
            }
            WebSocketEvent::Error { error } => {
                println!("⚠️  Error: {}", error);
            }
            WebSocketEvent::Disconnected { reason } => {
                println!("🔌 Disconnected: {:?}", reason);
                break;
            }
            _ => {}
        }
    }
    
    // Disconnect
    client.disconnect().await?;
    println!("Disconnected from Binance WebSocket");
    
    Ok(())
}