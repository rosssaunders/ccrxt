use clap::Parser;
use futures::StreamExt;
use venues::binance::spot::websocket::{
    BinanceMessage, BinanceSpotWebSocketClient, DepthLevel, UpdateSpeed,
};
use websockets::WebSocketEvent;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Symbol to monitor (e.g., BTCUSDT, ETHUSDT)
    #[arg(short, long, default_value = "BTCUSDT")]
    symbol: String,

    /// Show only deltas (hide partial depth snapshots)
    #[arg(short, long)]
    deltas_only: bool,

    /// Show full bid/ask arrays instead of just first 3
    #[arg(short, long)]
    full: bool,

    /// Maximum number of updates to show (0 for unlimited)
    #[arg(short, long, default_value = "0")]
    max: usize,

    /// Subscribe to partial book depth (depth5, depth10, depth20)
    #[arg(short, long)]
    partial: Option<String>,

    /// Update speed (fast=100ms, slow=1000ms)
    #[arg(short, long, default_value = "fast")]
    speed: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Binance Depth Stream Monitor");
    println!("============================");
    println!("Symbol: {}", args.symbol);
    println!("Speed: {}", args.speed);
    if args.deltas_only {
        println!("Mode: Deltas only");
    }
    if args.full {
        println!("Display: Full arrays");
    }
    if args.max > 0 {
        println!("Max updates: {}", args.max);
    }
    println!("\nPress Ctrl+C to stop...\n");

    // Handle Ctrl+C gracefully
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        println!("\n⚠️  Received interrupt signal, shutting down...");
        std::process::exit(0);
    });

    // Create and connect
    let mut client = BinanceSpotWebSocketClient::new();
    client.connect().await?;
    println!("✅ Connected to Binance\n");

    // Parse update speed
    let update_speed = match args.speed.as_str() {
        "slow" => UpdateSpeed::Slow,
        _ => UpdateSpeed::Fast,
    };

    // Subscribe to depth updates
    client.subscribe_diff_depth(&args.symbol, update_speed).await?;
    println!("✅ Subscribed to {}@depth@{}ms", args.symbol, if update_speed == UpdateSpeed::Fast { "100" } else { "1000" });

    // Subscribe to partial depth if requested
    if let Some(partial) = args.partial {
        let depth_level = match partial.as_str() {
            "5" | "depth5" => DepthLevel::Five,
            "10" | "depth10" => DepthLevel::Ten,
            "20" | "depth20" => DepthLevel::Twenty,
            _ => DepthLevel::Five,
        };
        client.subscribe_book_depth(&args.symbol, depth_level, update_speed).await?;
        println!("✅ Subscribed to {}@{:?}@{}ms", args.symbol, depth_level, if update_speed == UpdateSpeed::Fast { "100" } else { "1000" });
    }
    
    println!("\n📚 Listening for orderbook updates...\n");

    // Process events
    let mut events = client.event_stream();
    let mut depth_count = 0;
    let mut snapshot_count = 0;

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
                        
                        // Show bid updates
                        if !depth.bids.is_empty() {
                            if args.full {
                                println!("   Bid Updates ({} total):", depth.bids.len());
                                for (i, bid) in depth.bids.iter().enumerate() {
                                    println!("     {}. Price: {} Qty: {}", i+1, bid[0], bid[1]);
                                }
                            } else {
                                println!("   Bid Updates (showing first 3 of {}):", depth.bids.len());
                                for (i, bid) in depth.bids.iter().take(3).enumerate() {
                                    println!("     {}. Price: {} Qty: {}", i+1, bid[0], bid[1]);
                                }
                            }
                        }
                        
                        // Show ask updates
                        if !depth.asks.is_empty() {
                            if args.full {
                                println!("   Ask Updates ({} total):", depth.asks.len());
                                for (i, ask) in depth.asks.iter().enumerate() {
                                    println!("     {}. Price: {} Qty: {}", i+1, ask[0], ask[1]);
                                }
                            } else {
                                println!("   Ask Updates (showing first 3 of {}):", depth.asks.len());
                                for (i, ask) in depth.asks.iter().take(3).enumerate() {
                                    println!("     {}. Price: {} Qty: {}", i+1, ask[0], ask[1]);
                                }
                            }
                        }
                        
                        println!();
                        
                        if args.max > 0 && depth_count >= args.max {
                            println!("✅ Received {} depth updates. Stopping...", depth_count);
                            break;
                        }
                    }
                    BinanceMessage::PartialDepth(depth) => {
                        if !args.deltas_only {
                            snapshot_count += 1;
                            println!("📖 Partial Depth Snapshot #{}", snapshot_count);
                            println!("   Last Update ID: {}", depth.last_update_id);
                            
                            if args.full {
                                println!("   Bids:");
                                for (i, bid) in depth.bids.iter().enumerate() {
                                    println!("     {}. Price: {} Qty: {}", i+1, bid[0], bid[1]);
                                }
                                
                                println!("   Asks:");
                                for (i, ask) in depth.asks.iter().enumerate() {
                                    println!("     {}. Price: {} Qty: {}", i+1, ask[0], ask[1]);
                                }
                            } else {
                                println!("   Top 3 Bids:");
                                for (i, bid) in depth.bids.iter().take(3).enumerate() {
                                    println!("     {}. Price: {} Qty: {}", i+1, bid[0], bid[1]);
                                }
                                
                                println!("   Top 3 Asks:");
                                for (i, ask) in depth.asks.iter().take(3).enumerate() {
                                    println!("     {}. Price: {} Qty: {}", i+1, ask[0], ask[1]);
                                }
                            }
                            println!();
                        }
                    }
                    BinanceMessage::Response(resp) => {
                        println!("📬 Subscription confirmed (ID: {})\n", resp.id);
                    }
                    _ => {}
                }
            }
            WebSocketEvent::Error { error } => {
                eprintln!("⚠️  Error: {}", error);
            }
            WebSocketEvent::Disconnected { reason } => {
                println!("🔌 Disconnected: {:?}", reason);
                break;
            }
            _ => {}
        }
    }

    // Print statistics
    println!("\n📊 Statistics:");
    println!("   Total depth updates: {}", depth_count);
    println!("   Total snapshots: {}", snapshot_count);

    // Disconnect
    client.disconnect().await?;
    println!("\n👋 Disconnected from Binance");

    Ok(())
}