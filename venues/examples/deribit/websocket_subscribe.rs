//! Example usage of Deribit WebSocket public/subscribe endpoint
//!
//! This example demonstrates how to connect to Deribit's WebSocket API and
//! subscribe to public channels using the public/subscribe method.

use venues::deribit::websocket::DeribitWebSocketClient;
use websockets::WebSocketConnection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create a new Deribit WebSocket client
    let mut client = DeribitWebSocketClient::new();
    println!("Connecting to Deribit WebSocket API...");

    // Connect to the WebSocket
    if let Err(e) = client.connect().await {
        eprintln!("❌ Failed to connect: {}", e);
        return Ok(());
    }
    println!("✅ Successfully connected to Deribit WebSocket!");

    // Define channels to subscribe to
    let channels = vec![
        "book.BTC-PERPETUAL.100ms".to_string(),  // Order book for BTC perpetual
        "ticker.BTC-PERPETUAL".to_string(),       // Ticker data for BTC perpetual
        "trades.BTC-PERPETUAL".to_string(),       // Trade data for BTC perpetual
    ];

    println!("Subscribing to channels: {:?}", channels);

    // Subscribe to public channels
    match client.public_subscribe(channels).await {
        Ok(subscribed_channels) => {
            println!("✅ Successfully subscribed to channels:");
            for channel in subscribed_channels {
                println!("  - {}", channel);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to subscribe: {}", e);
        }
    }

    // Disconnect when done
    if let Err(e) = client.disconnect().await {
        eprintln!("Warning: Failed to disconnect cleanly: {}", e);
    }
    println!("🔌 Disconnected from Deribit WebSocket");

    Ok(())
}