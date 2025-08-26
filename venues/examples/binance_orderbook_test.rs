use futures::StreamExt;
use venues::binance::spot::websocket::{
    BinanceMessage, BinanceSpotWebSocketClient, DepthLevel, UpdateSpeed,
};
use websockets::WebSocketEvent;

/// Test Binance orderbook (depth) updates
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Binance Orderbook Updates Test");
    println!("===============================\n");
    
    // Create and connect
    let mut client = BinanceSpotWebSocketClient::new();
    client.connect().await?;
    println!("✅ Connected to Binance\n");
    
    // Subscribe to depth updates
    println!("Subscribing to BTCUSDT depth updates...");
    client.subscribe_diff_depth("BTCUSDT", UpdateSpeed::Fast).await?;
    println!("✅ Subscribed to BTCUSDT@depth@100ms\n");
    
    // Also subscribe to partial book depth for comparison
    client.subscribe_book_depth("BTCUSDT", DepthLevel::Five, UpdateSpeed::Fast).await?;
    println!("✅ Subscribed to BTCUSDT@depth5@100ms\n");
    
    println!("📚 Listening for orderbook updates...\n");
    
    // Process events
    let mut events = client.event_stream();
    let mut depth_count = 0;
    
    // Handle Ctrl+C gracefully
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        println!("\n⚠️  Received interrupt signal, shutting down...");
        std::process::exit(0);
    });
    
    println!("Press Ctrl+C to stop...\n");
    
    while let Some(event) = events.next().await {
        match event {
            WebSocketEvent::Message { message } => {
                match message {
                    BinanceMessage::DepthUpdate(depth) => {
                        depth_count += 1;
                        println!("📚 Depth Update #{} (Delta)", depth_count);
                        println!("   Symbol: {}", depth.symbol);
                        println!("   Event Time: {}", depth.event_time);
                        println!("   Update ID: {} -> {}", depth.first_update_id, depth.final_update_id);
                        
                        // Show first few bid updates
                        if !depth.bids.is_empty() {
                            println!("   Bid Updates (showing first 3):");
                            for (i, bid) in depth.bids.iter().take(3).enumerate() {
                                println!("     {}. Price: {} Qty: {}", i+1, bid[0], bid[1]);
                            }
                        }
                        
                        // Show first few ask updates
                        if !depth.asks.is_empty() {
                            println!("   Ask Updates (showing first 3):");
                            for (i, ask) in depth.asks.iter().take(3).enumerate() {
                                println!("     {}. Price: {} Qty: {}", i+1, ask[0], ask[1]);
                            }
                        }
                        
                        println!("   Total: {} bid updates, {} ask updates\n", 
                                depth.bids.len(), depth.asks.len());
                    }
                    BinanceMessage::PartialDepth(depth) => {
                        println!("📖 Partial Depth Snapshot (depth5)");
                        println!("   Last Update ID: {}", depth.last_update_id);
                        
                        println!("   Top 5 Bids:");
                        for (i, bid) in depth.bids.iter().enumerate() {
                            println!("     {}. Price: {} Qty: {}", i+1, bid[0], bid[1]);
                        }
                        
                        println!("   Top 5 Asks:");
                        for (i, ask) in depth.asks.iter().enumerate() {
                            println!("     {}. Price: {} Qty: {}", i+1, ask[0], ask[1]);
                        }
                        println!();
                    }
                    BinanceMessage::Response(resp) => {
                        println!("📬 Subscription confirmed (ID: {})\n", resp.id);
                    }
                    _ => {}
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
    println!("\n👋 Disconnected from Binance");
    
    Ok(())
}