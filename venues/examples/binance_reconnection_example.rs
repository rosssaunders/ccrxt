use futures::StreamExt;
use std::time::Duration;
use tokio::time::sleep;
use venues::binance::spot::websocket::{
    BinanceMessage, BinanceSpotWebSocketClient, UpdateSpeed,
};
use websockets::{DisconnectReason, WebSocketEvent};

/// Example showing how to handle WebSocket disconnections and reconnections
/// 
/// IMPORTANT: The library does NOT automatically reconnect. This example shows
/// how users can implement their own reconnection logic based on their requirements.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Binance WebSocket Reconnection Example");
    println!("======================================\n");
    
    let mut client = BinanceSpotWebSocketClient::new();
    let mut reconnect_attempts = 0;
    const MAX_RECONNECT_ATTEMPTS: u32 = 5;
    
    loop {
        // Connect to WebSocket
        match client.connect().await {
            Ok(_) => {
                println!("✅ Connected to Binance WebSocket");
                reconnect_attempts = 0; // Reset counter on successful connection
                
                // Subscribe to streams
                if let Err(e) = client.subscribe_diff_depth("BTCUSDT", UpdateSpeed::Fast).await {
                    eprintln!("⚠️  Failed to subscribe: {}", e);
                    continue;
                }
                println!("📊 Subscribed to BTCUSDT depth updates\n");
                
                // Process events
                let mut events = client.event_stream();
                let mut message_count = 0;
                
                while let Some(event) = events.next().await {
                    match event {
                        WebSocketEvent::Connected => {
                            println!("🔌 Connected event received");
                        }
                        
                        WebSocketEvent::Disconnected { reason } => {
                            println!("🔌 Disconnected: {:?}", reason);
                            
                            // Decide whether to reconnect based on the reason
                            match reason {
                                DisconnectReason::UserInitiated => {
                                    println!("User initiated disconnect, exiting...");
                                    return Ok(());
                                }
                                DisconnectReason::RemoteClosed { code, reason } => {
                                    println!("Server closed connection (code: {}, reason: {})", code, reason);
                                    
                                    // Some close codes might indicate we shouldn't reconnect
                                    if code == 1008 || code == 1003 {
                                        println!("Policy violation or unsupported data, not reconnecting");
                                        return Ok(());
                                    }
                                }
                                DisconnectReason::NetworkError { details } => {
                                    println!("Network error: {}", details);
                                }
                                DisconnectReason::ProtocolError { details } => {
                                    println!("Protocol error: {}", details);
                                }
                                DisconnectReason::InvalidMessage { details } => {
                                    println!("Invalid message: {}", details);
                                }
                            }
                            
                            // Break inner loop to trigger reconnection
                            break;
                        }
                        
                        WebSocketEvent::Error { error } => {
                            eprintln!("⚠️  Error: {}", error);
                            // Errors don't necessarily mean disconnection
                            // User can decide whether to continue or reconnect
                        }
                        
                        WebSocketEvent::Message { message } => {
                            message_count += 1;
                            
                            // Just count messages for this example
                            if message_count % 10 == 0 {
                                println!("📦 Received {} messages", message_count);
                            }
                            
                            // Handle specific message types if needed
                            match message {
                                BinanceMessage::DepthUpdate(_) => {
                                    // Process depth update
                                }
                                BinanceMessage::Error(err) => {
                                    eprintln!("⚠️  Binance error: {} - {}", err.code, err.msg);
                                }
                                _ => {}
                            }
                        }
                        
                        WebSocketEvent::PingReceived { .. } => {
                            // Binance handles ping/pong automatically
                        }
                        
                        WebSocketEvent::PongReceived { .. } => {
                            // Binance handles ping/pong automatically
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to connect: {}", e);
            }
        }
        
        // Reconnection logic with exponential backoff
        reconnect_attempts += 1;
        if reconnect_attempts > MAX_RECONNECT_ATTEMPTS {
            eprintln!("❌ Max reconnection attempts reached, giving up");
            break;
        }
        
        let backoff_seconds = 2u64.pow(reconnect_attempts.min(6)); // Cap at 64 seconds
        println!(
            "⏳ Waiting {} seconds before reconnection attempt {}/{}...",
            backoff_seconds, reconnect_attempts, MAX_RECONNECT_ATTEMPTS
        );
        sleep(Duration::from_secs(backoff_seconds)).await;
        
        // Disconnect before reconnecting (cleanup)
        let _ = client.disconnect().await;
    }
    
    Ok(())
}

/// Alternative: Simple reconnection handler as a separate function
async fn handle_with_reconnection(
    symbol: &str,
    mut process_message: impl FnMut(BinanceMessage),
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BinanceSpotWebSocketClient::new();
    
    loop {
        // Connect
        client.connect().await?;
        client.subscribe_trades(symbol).await?;
        
        // Process events
        let mut events = client.event_stream();
        while let Some(event) = events.next().await {
            match event {
                WebSocketEvent::Message { message } => {
                    process_message(message);
                }
                WebSocketEvent::Disconnected { reason } => {
                    match reason {
                        DisconnectReason::UserInitiated => return Ok(()),
                        _ => {
                            // Reconnect after a delay
                            sleep(Duration::from_secs(5)).await;
                            break; // Break inner loop to reconnect
                        }
                    }
                }
                _ => {}
            }
        }
        
        // Cleanup before reconnecting
        let _ = client.disconnect().await;
    }
}