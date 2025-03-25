use futures::StreamExt;
use orderbook::OrderBook;
use std::error::Error;
use std::time::{Duration, Instant};
use venues::bybit::{
    spot::{BybitSpotPublicWebSocket, BybitSpotPublicRest, WebSocketMessage as BybitSpotMessage},
    perp::{BybitPerpPublicWebSocket, BybitPerpPublicRest, WebSocketMessage as BybitPerpMessage},
};
use venues::websockets::WebSocketConnection;
use serde_json::Value;

#[derive(Default)]
struct Metrics {
    updates_processed: u64,
    last_update_latency_ms: u64,
    avg_update_latency_ms: f64,
    max_update_latency_ms: u64,
}

impl Metrics {
    fn update(&mut self, latency_ms: u64) {
        self.last_update_latency_ms = latency_ms;
        self.avg_update_latency_ms = (self.avg_update_latency_ms * self.updates_processed as f64 + latency_ms as f64) / 
                                   (self.updates_processed as f64 + 1.0);
        self.max_update_latency_ms = self.max_update_latency_ms.max(latency_ms);
        self.updates_processed += 1;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Bybit Spot and Perp Orderbook Example");
    println!("=====================================");
    
    // Run spot and perp examples concurrently
    let spot_handle = tokio::spawn(async {
        match run_spot_example().await {
            Ok(_) => println!("Spot example completed successfully"),
            Err(e) => eprintln!("Spot example error: {}", e),
        }
    });
    
    let perp_handle = tokio::spawn(async {
        match run_perp_example().await {
            Ok(_) => println!("Perp example completed successfully"),
            Err(e) => eprintln!("Perp example error: {}", e),
        }
    });
    
    // Wait for both examples to complete
    let _ = tokio::join!(spot_handle, perp_handle);
    
    Ok(())
}

async fn run_spot_example() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("\nStarting Bybit Spot example...");
    
    let symbol = "BTCUSDT";
    let mut metrics = Metrics::default();
    
    // Initialize REST client and WebSocket
    let rest_client = BybitSpotPublicRest::new();
    let mut ws = BybitSpotPublicWebSocket::new();
    
    // Connect to WebSocket
    println!("Connecting to Bybit Spot WebSocket...");
    ws.connect().await?;
    
    // Subscribe to orderbook channel
    let channel = format!("orderbook.50.{}", symbol);
    println!("Subscribing to channel: {}", channel);
    ws.subscribe(vec![channel]).await?;
    
    // Get initial snapshot
    println!("Fetching initial orderbook snapshot...");
    let snapshot = rest_client.get_orderbook_snapshot(symbol, Some(50)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    println!("Received snapshot with {} bids and {} asks", 
             snapshot.result.b.len(), 
             snapshot.result.a.len());
    
    // Initialize orderbook
    let mut orderbook = OrderBook::new(8);  // Using 8 decimal places for price precision
    
    // Process snapshot data
    let bids: Vec<(f64, f64)> = snapshot.result.b.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    let asks: Vec<(f64, f64)> = snapshot.result.a.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    println!("Processed {} bids and {} asks from snapshot", bids.len(), asks.len());
    
    // Apply snapshot to orderbook
    orderbook.apply_snapshot(bids, asks);
    
    // Print initial orderbook state
    print_orderbook("Bybit Spot", &orderbook, 5);
    
    // Process WebSocket messages
    println!("Processing WebSocket messages...");
    let mut message_stream = ws.message_stream();
    let start_time = Instant::now();
    let mut message_count = 0;
    
    // Process messages for 30 seconds
    while start_time.elapsed() < Duration::from_secs(30) {
        if let Some(msg) = message_stream.next().await {
            message_count += 1;
            let update_time = Instant::now();
            
            match msg {
                Ok(BybitSpotMessage::OrderBook(update)) => {
                    println!("Received orderbook update for {}: {} bids, {} asks", 
                             update.data.s, update.data.b.len(), update.data.a.len());
                    
                    // Process update data
                    let bids: Vec<(f64, f64, bool)> = update.data.b.iter()
                        .filter_map(|bid| {
                            let price = bid.0.parse::<f64>().ok()?;
                            let size = bid.1.parse::<f64>().ok()?;
                            Some((price, size, true)) // true for bids
                        })
                        .collect();
                    
                    let asks: Vec<(f64, f64, bool)> = update.data.a.iter()
                        .filter_map(|ask| {
                            let price = ask.0.parse::<f64>().ok()?;
                            let size = ask.1.parse::<f64>().ok()?;
                            Some((price, size, false)) // false for asks
                        })
                        .collect();
                    
                    // Apply updates to orderbook
                    for (price, size, is_bid) in bids.iter().chain(asks.iter()) {
                        orderbook.update(*price, *size, *is_bid);
                    }
                    
                    // Update metrics
                    let latency = update_time.elapsed().as_millis() as u64;
                    metrics.update(latency);
                    
                    // Print updated orderbook every 10 updates
                    if metrics.updates_processed % 10 == 0 {
                        print_orderbook("Bybit Spot", &orderbook, 5);
                        print_metrics(&metrics);
                    }
                },
                Ok(BybitSpotMessage::Ping(ping)) => {
                    println!("Received ping response: {}", ping.ret_msg);
                },
                Ok(BybitSpotMessage::Subscription(sub)) => {
                    println!("Received subscription response: {}", sub.ret_msg);
                },
                Ok(BybitSpotMessage::Raw(value)) => {
                    println!("Received raw message: {}", serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string()));
                },
                Err(e) => {
                    eprintln!("Bybit Spot WebSocket error: {}", e);
                }
            }
        }
    }
    
    println!("Received {} messages in total", message_count);
    
    // Disconnect WebSocket
    println!("Disconnecting from Bybit Spot WebSocket...");
    ws.disconnect().await?;
    
    Ok(())
}

async fn run_perp_example() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("\nStarting Bybit Perp example...");
    
    let symbol = "BTCUSDT";
    let mut metrics = Metrics::default();
    
    // Initialize REST client and WebSocket
    let rest_client = BybitPerpPublicRest::new();
    let mut ws = BybitPerpPublicWebSocket::new();
    
    // Connect to WebSocket
    println!("Connecting to Bybit Perp WebSocket...");
    ws.connect().await?;
    
    // Subscribe to orderbook channel
    let channel = format!("orderbook.50.{}", symbol);
    println!("Subscribing to channel: {}", channel);
    ws.subscribe(vec![channel]).await?;
    
    // Get initial snapshot
    println!("Fetching initial orderbook snapshot...");
    let snapshot = rest_client.get_orderbook_snapshot(symbol, Some(50)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    println!("Received snapshot with {} bids and {} asks", 
             snapshot.result.b.len(), 
             snapshot.result.a.len());
    
    // Initialize orderbook
    let mut orderbook = OrderBook::new(8);  // Using 8 decimal places for price precision
    
    // Process snapshot data
    let bids: Vec<(f64, f64)> = snapshot.result.b.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    let asks: Vec<(f64, f64)> = snapshot.result.a.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    println!("Processed {} bids and {} asks from snapshot", bids.len(), asks.len());
    
    // Apply snapshot to orderbook
    orderbook.apply_snapshot(bids, asks);
    
    // Print initial orderbook state
    print_orderbook("Bybit Perp", &orderbook, 5);
    
    // Process WebSocket messages
    println!("Processing WebSocket messages...");
    let mut message_stream = ws.message_stream();
    let start_time = Instant::now();
    let mut message_count = 0;
    
    // Process messages for 30 seconds
    while start_time.elapsed() < Duration::from_secs(30) {
        if let Some(msg) = message_stream.next().await {
            message_count += 1;
            let update_time = Instant::now();
            
            match msg {
                Ok(BybitPerpMessage::OrderBook(update)) => {
                    println!("Received orderbook update for {}: {} bids, {} asks", 
                             update.data.s, update.data.b.len(), update.data.a.len());
                    
                    // Process update data
                    let bids: Vec<(f64, f64, bool)> = update.data.b.iter()
                        .filter_map(|bid| {
                            let price = bid.0.parse::<f64>().ok()?;
                            let size = bid.1.parse::<f64>().ok()?;
                            Some((price, size, true)) // true for bids
                        })
                        .collect();
                    
                    let asks: Vec<(f64, f64, bool)> = update.data.a.iter()
                        .filter_map(|ask| {
                            let price = ask.0.parse::<f64>().ok()?;
                            let size = ask.1.parse::<f64>().ok()?;
                            Some((price, size, false)) // false for asks
                        })
                        .collect();
                    
                    // Apply updates to orderbook
                    for (price, size, is_bid) in bids.iter().chain(asks.iter()) {
                        orderbook.update(*price, *size, *is_bid);
                    }
                    
                    // Update metrics
                    let latency = update_time.elapsed().as_millis() as u64;
                    metrics.update(latency);
                    
                    // Print updated orderbook every 10 updates
                    if metrics.updates_processed % 10 == 0 {
                        print_orderbook("Bybit Perp", &orderbook, 5);
                        print_metrics(&metrics);
                    }
                },
                Ok(BybitPerpMessage::Ping(ping)) => {
                    println!("Received ping response: {}", ping.ret_msg);
                },
                Ok(BybitPerpMessage::Subscription(sub)) => {
                    println!("Received subscription response: {}", sub.ret_msg);
                },
                Ok(BybitPerpMessage::Raw(value)) => {
                    println!("Received raw message: {}", serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string()));
                },
                Err(e) => {
                    eprintln!("Bybit Perp WebSocket error: {}", e);
                }
            }
        }
    }
    
    println!("Received {} messages in total", message_count);
    
    // Disconnect WebSocket
    println!("Disconnecting from Bybit Perp WebSocket...");
    ws.disconnect().await?;
    
    Ok(())
}

fn print_orderbook(name: &str, orderbook: &OrderBook, depth: usize) {
    println!("\n{} Orderbook:", name);
    println!("===================");
    
    let (bids, asks) = orderbook.get_depth_with_prices(depth);
    
    println!("Asks:");
    for (i, (price, level)) in asks.iter().rev().enumerate() {
        println!("  {}: {} @ {:.8}", i + 1, level.size, price);
    }
    
    if let Some(best_prices) = orderbook.best_bid_ask_prices() {
        let (bid, ask) = best_prices;
        let spread = ask - bid;
        let spread_pct = (spread / bid) * 100.0;
        println!("\nSpread: {:.8} ({:.4}%)", spread, spread_pct);
    }
    
    println!("\nBids:");
    for (i, (price, level)) in bids.iter().enumerate() {
        println!("  {}: {} @ {:.8}", i + 1, level.size, price);
    }
    println!("");
}

fn print_metrics(metrics: &Metrics) {
    println!("Metrics:");
    println!("  Updates processed: {}", metrics.updates_processed);
    println!("  Last update latency: {}ms", metrics.last_update_latency_ms);
    println!("  Avg update latency: {:.2}ms", metrics.avg_update_latency_ms);
    println!("  Max update latency: {}ms", metrics.max_update_latency_ms);
    println!("");
} 