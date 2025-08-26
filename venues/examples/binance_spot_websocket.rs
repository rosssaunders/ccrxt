use std::time::Duration;

use futures::StreamExt;
use tokio::time::sleep;
use venues::binance::spot::websocket::{
    BinanceMessage, BinanceSpotWebSocketClient, KlineInterval,
};
use websockets::{DisconnectReason, WebSocketEvent};

/// Example demonstrating Binance Spot WebSocket usage with user-controlled reconnection
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Binance Spot WebSocket Example");
    println!("==============================\n");

    // Run the connection with reconnection logic
    maintain_connection().await
}

/// Maintain connection with exponential backoff reconnection
async fn maintain_connection() -> Result<(), Box<dyn std::error::Error>> {
    let mut backoff = Duration::from_secs(1);
    let max_backoff = Duration::from_secs(60);
    let mut attempt = 0;

    loop {
        println!("Connection attempt #{}", attempt + 1);

        // Create a new WebSocket client
        let mut client = BinanceSpotWebSocketClient::new();

        // Try to connect
        match client.connect().await {
            Ok(_) => {
                println!("✅ Connected to Binance WebSocket");
                backoff = Duration::from_secs(1); // Reset backoff
                attempt = 0;

                // Subscribe to streams
                if let Err(e) = subscribe_to_streams(&mut client).await {
                    println!("❌ Failed to subscribe: {}", e);
                } else {
                    // Handle the connection
                    match handle_connection(&mut client).await {
                        Ok(_) => {
                            println!("Connection closed normally");
                        }
                        Err(e) => {
                            println!("Connection error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to connect: {}", e);
                attempt += 1;

                if attempt >= 5 {
                    println!("Max reconnection attempts reached. Giving up.");
                    return Err("Max reconnection attempts exceeded".into());
                }

                println!("⏳ Retrying in {:?}...", backoff);
                sleep(backoff).await;

                // Exponential backoff
                backoff = (backoff * 2).min(max_backoff);
            }
        }
    }
}

/// Subscribe to various Binance streams
async fn subscribe_to_streams(
    client: &mut BinanceSpotWebSocketClient,
) -> Result<(), Box<dyn std::error::Error>> {
    // Subscribe to BTC/USDT trades
    client.subscribe_trades("BTCUSDT").await?;
    println!("📊 Subscribed to BTCUSDT trades");

    // Subscribe to BTC/USDT 1-minute klines
    client
        .subscribe_klines("BTCUSDT", KlineInterval::OneMinute)
        .await?;
    println!("📈 Subscribed to BTCUSDT 1m klines");

    // Subscribe to ETH/USDT aggregate trades
    client.subscribe_agg_trades("ETHUSDT").await?;
    println!("📊 Subscribed to ETHUSDT aggregate trades");

    Ok(())
}

/// Handle an active WebSocket connection
async fn handle_connection(
    client: &mut BinanceSpotWebSocketClient,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get the event stream
    let mut events = client.event_stream();
    let mut message_count = 0;

    println!("\n🎧 Listening for events...\n");

    // Process events
    while let Some(event) = events.next().await {
        match event {
            WebSocketEvent::Connected => {
                println!("🔌 Event: Connected");
            }
            WebSocketEvent::Disconnected { reason } => {
                println!("🔌 Event: Disconnected - {:?}", reason);

                // Decide whether to reconnect based on the reason
                match reason {
                    DisconnectReason::UserInitiated => {
                        // User initiated disconnect, don't reconnect
                        return Ok(());
                    }
                    DisconnectReason::RemoteClosed { code, reason } => {
                        if code == 1000 {
                            println!("Normal closure: {}", reason);
                            return Ok(());
                        }
                        println!("Remote closed with code {}: {}", code, reason);
                        break;
                    }
                    DisconnectReason::NetworkError { details } => {
                        println!("Network error: {}", details);
                        break;
                    }
                    DisconnectReason::ProtocolError { details } => {
                        println!("Protocol error: {}", details);
                        break;
                    }
                    DisconnectReason::InvalidMessage { details } => {
                        println!("Invalid message: {}", details);
                        // Continue processing, don't disconnect
                    }
                }
            }
            WebSocketEvent::Error { error } => {
                println!("⚠️  Event: Error - {}", error);
                // Errors don't necessarily mean disconnection
            }
            WebSocketEvent::Message { message } => {
                message_count += 1;

                // Handle different message types
                match message {
                    BinanceMessage::Trade(trade) => {
                        println!(
                            "💰 Trade: {} {} @ {} (ID: {})",
                            trade.symbol, trade.quantity, trade.price, trade.trade_id
                        );
                    }
                    BinanceMessage::AggTrade(agg_trade) => {
                        println!(
                            "💰 Agg Trade: {} {} @ {} (ID: {})",
                            agg_trade.symbol, agg_trade.quantity, agg_trade.price, agg_trade.agg_trade_id
                        );
                    }
                    BinanceMessage::Kline(kline) => {
                        let k = &kline.kline;
                        println!(
                            "📊 Kline {}: {} O:{} H:{} L:{} C:{} V:{} Closed:{}",
                            k.interval,
                            kline.symbol,
                            k.open,
                            k.high,
                            k.low,
                            k.close,
                            k.volume,
                            k.is_closed
                        );
                    }
                    BinanceMessage::DepthUpdate(depth) => {
                        println!(
                            "📚 Depth Update: {} Bids:{} Asks:{}",
                            depth.symbol,
                            depth.bids.len(),
                            depth.asks.len()
                        );
                    }
                    BinanceMessage::Response(response) => {
                        println!("📬 Subscription response: ID={}", response.id);
                    }
                    BinanceMessage::Error(error) => {
                        println!("❌ Error from Binance: {} - {}", error.code, error.msg);
                    }
                    _ => {
                        println!("📦 Other message received");
                    }
                }

                // Stop after receiving 10 messages for demo purposes
                if message_count >= 10 {
                    println!("\n✅ Received {} messages. Disconnecting for demo...", message_count);
                    client.disconnect().await?;
                    return Ok(());
                }
            }
            WebSocketEvent::PingReceived { data } => {
                println!("🏓 Ping received with {} bytes", data.len());
            }
            WebSocketEvent::PongReceived { data } => {
                println!("🏓 Pong received with {} bytes", data.len());
            }
        }
    }

    Ok(())
}

