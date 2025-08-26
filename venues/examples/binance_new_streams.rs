//! Example demonstrating the new Binance WebSocket streams

use futures::StreamExt;
use venues::binance::spot::websocket::{
    BinanceMessage, BinanceSpotWebSocketClient, RollingWindowSize,
};
use websockets::WebSocketEvent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Binance New WebSocket Streams Example\n");

    // Create a new WebSocket client
    let mut client = BinanceSpotWebSocketClient::new();

    // Connect to WebSocket
    println!("Connecting to Binance WebSocket...");
    client.connect().await?;
    println!("Connected successfully!\n");

    // Subscribe to the new Average Price stream
    println!("Subscribing to BTC/USDT Average Price stream...");
    client.subscribe_avg_price("BTCUSDT").await?;

    // Subscribe to Rolling Window Statistics (1 hour)
    println!("Subscribing to BTC/USDT Rolling Window Ticker (1h)...");
    client
        .subscribe_rolling_window_ticker("BTCUSDT", RollingWindowSize::OneHour)
        .await?;

    // Subscribe to Rolling Window Statistics (4 hour)
    println!("Subscribing to ETH/USDT Rolling Window Ticker (4h)...");
    client
        .subscribe_rolling_window_ticker("ETHUSDT", RollingWindowSize::FourHour)
        .await?;

    println!("\nListening for messages (press Ctrl+C to stop)...\n");

    // Process events
    let mut event_stream = client.event_stream();
    let mut message_count = 0;

    while let Some(event) = event_stream.next().await {
        match event {
            WebSocketEvent::Message { message } => {
                message_count += 1;
                match message {
                    BinanceMessage::AvgPrice(data) => {
                        println!(
                            "[AvgPrice] {} - Avg: {} (interval: {})",
                            data.symbol, data.avg_price, data.interval
                        );
                    }
                    BinanceMessage::RollingWindowTicker(data) => {
                        println!(
                            "[Rolling {}] {} - Price: {} Change: {}% Vol: {}",
                            data.event_type,
                            data.symbol,
                            data.last_price,
                            data.price_change_percent,
                            data.volume
                        );
                    }
                    _ => {
                        // Other message types
                        println!("[Other] Received message: {:?}", message);
                    }
                }

                // Stop after 10 messages for demo
                if message_count >= 10 {
                    println!("\nReceived 10 messages, disconnecting...");
                    break;
                }
            }
            WebSocketEvent::Disconnected { reason } => {
                println!("Disconnected: {:?}", reason);
                break;
            }
            WebSocketEvent::Error { error } => {
                eprintln!("Error: {}", error);
            }
            _ => {}
        }
    }

    // Disconnect
    client.disconnect().await?;
    println!("Disconnected successfully!");

    Ok(())
}