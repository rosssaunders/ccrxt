use futures::StreamExt;
use orderbook::OrderBook;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use venues::binance::usdm::{BinanceUsdMPublicWebSocket, BinanceUsdMPublicRest, WebSocketMessage, OrderBookUpdate};
use venues::websockets::WebSocketConnection;
use std::error::Error;

const MAX_RECONNECT_ATTEMPTS: u32 = 5;
const INITIAL_BACKOFF_MS: u64 = 1000;
const MAX_BACKOFF_MS: u64 = 30000;
const MAX_BUFFER_SIZE: usize = 1000;  // Maximum number of updates to buffer
const BUFFER_TIMEOUT_MS: u64 = 100;   // Time to wait for each buffer message
const MAX_BUFFER_TIME_MS: u64 = 5000; // Maximum time to spend buffering

// Performance metrics
#[derive(Default)]
struct Metrics {
    updates_processed: u64,
    gaps_detected: u64,
    reconnects: u64,
    last_update_latency_ms: u64,
    avg_update_latency_ms: f64,
    max_update_latency_ms: u64,
}

impl Metrics {
    fn update_latency(&mut self, latency_ms: u64) {
        self.last_update_latency_ms = latency_ms;
        self.avg_update_latency_ms = (self.avg_update_latency_ms * self.updates_processed as f64 + latency_ms as f64) / 
                                   (self.updates_processed as f64 + 1.0);
        self.max_update_latency_ms = self.max_update_latency_ms.max(latency_ms);
        self.updates_processed += 1;
    }

    fn log_status(&self) {
        println!("Performance Metrics:");
        println!("  Updates Processed: {}", self.updates_processed);
        println!("  Gaps Detected: {}", self.gaps_detected);
        println!("  Reconnects: {}", self.reconnects);
        println!("  Last Update Latency: {}ms", self.last_update_latency_ms);
        println!("  Avg Update Latency: {:.2}ms", self.avg_update_latency_ms);
        println!("  Max Update Latency: {}ms", self.max_update_latency_ms);
    }
}

#[derive(Debug)]
enum OrderBookError {
    WebSocketError(String),
    RestError(String),
    SequenceGap(String),
    Timeout(String),
    ParseError(String),
}

impl std::fmt::Display for OrderBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderBookError::WebSocketError(msg) => write!(f, "WebSocket error: {}", msg),
            OrderBookError::RestError(msg) => write!(f, "REST error: {}", msg),
            OrderBookError::SequenceGap(msg) => write!(f, "Sequence gap: {}", msg),
            OrderBookError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            OrderBookError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for OrderBookError {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let rest_client = BinanceUsdMPublicRest::new();
    let symbol = "BTCUSDT";  // USDM uses USDT pairs
    let mut metrics = Metrics::default();
    
    let mut reconnect_attempts = 0;
    let mut backoff_ms = INITIAL_BACKOFF_MS;

    loop {
        match maintain_orderbook(&rest_client, symbol, &mut metrics).await {
            Ok(_) => {
                reconnect_attempts = 0;
                backoff_ms = INITIAL_BACKOFF_MS;
            }
            Err(e) => {
                eprintln!("Error in orderbook maintenance: {}", e);
                reconnect_attempts += 1;
                metrics.reconnects += 1;
                
                if reconnect_attempts >= MAX_RECONNECT_ATTEMPTS {
                    eprintln!("Max reconnection attempts reached. Final metrics:");
                    metrics.log_status();
                    return Err(e);
                }

                println!("Attempting reconnection in {} ms...", backoff_ms);
                tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                backoff_ms = (backoff_ms * 2).min(MAX_BACKOFF_MS);
            }
        }
    }
}

async fn maintain_orderbook(rest_client: &BinanceUsdMPublicRest, symbol: &str, metrics: &mut Metrics) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut ws = BinanceUsdMPublicWebSocket::new();
    ws.connect().await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Subscribe to depth stream before creating the stream
    ws.subscribe(vec![format!("{}@depth", symbol.to_lowercase())]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Create stream after subscription
    let mut stream = ws.message_stream();

    // Wait for subscription confirmation and buffer updates
    let mut buffered_updates = Vec::new();
    let mut subscription_confirmed = false;
    let start = Instant::now();

    while !subscription_confirmed && start.elapsed() < Duration::from_secs(5) {
        if let Some(msg) = stream.next().await {
            match msg.map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))? {
                WebSocketMessage::Raw(value) => {
                    if value.get("result").is_some() {
                        subscription_confirmed = true;
                    }
                }
                WebSocketMessage::OrderBook(update) => {
                    buffered_updates.push(update);
                }
            }
        }
    }

    if !subscription_confirmed {
        return Err("Failed to confirm subscription".into());
    }

    // Get snapshot
    let snapshot = rest_client.get_orderbook_snapshot(symbol, Some(1000)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    let mut orderbook = OrderBook::new(8);  // Using 8 decimal places for price precision
    
    // Convert snapshot data to the format expected by OrderBook
    let bids: Vec<(f64, f64)> = snapshot.bids.iter()
        .map(|level| {
            Ok((
                level.0.parse().map_err(|e| format!("Failed to parse bid price: {}", e))?,
                level.1.parse().map_err(|e| format!("Failed to parse bid quantity: {}", e))?
            ))
        })
        .collect::<Result<_, String>>()?;
    
    let asks: Vec<(f64, f64)> = snapshot.asks.iter()
        .map(|level| {
            Ok((
                level.0.parse().map_err(|e| format!("Failed to parse ask price: {}", e))?,
                level.1.parse().map_err(|e| format!("Failed to parse ask quantity: {}", e))?
            ))
        })
        .collect::<Result<_, String>>()?;

    orderbook.apply_snapshot(bids, asks);

    // Process buffered updates that are newer than snapshot
    for update in buffered_updates {
        if update.final_update_id >= snapshot.last_update_id {
            apply_update(&mut orderbook, &update)?;
        }
    }

    // Process real-time updates
    let mut last_status = Instant::now();
    
    while let Some(msg) = stream.next().await {
        match msg.map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))? {
            WebSocketMessage::OrderBook(update) => {
                let update_time = Instant::now();
                apply_update(&mut orderbook, &update)?;
                metrics.update_latency(update_time.elapsed().as_millis() as u64);

                // Log status every 5 seconds
                if last_status.elapsed() >= Duration::from_secs(5) {
                    log_orderbook_state(&orderbook, "Periodic update");
                    metrics.log_status();
                    last_status = Instant::now();
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn apply_update(orderbook: &mut OrderBook, update: &OrderBookUpdate) -> Result<(), String> {
    let mut updates = Vec::with_capacity(update.bids.len() + update.asks.len());
    
    // Process bids
    for bid in &update.bids {
        let price = bid.0.parse()
            .map_err(|e| format!("Failed to parse bid price: {}", e))?;
        let quantity = bid.1.parse()
            .map_err(|e| format!("Failed to parse bid quantity: {}", e))?;
        updates.push((price, quantity, true));
    }
    
    // Process asks
    for ask in &update.asks {
        let price = ask.0.parse()
            .map_err(|e| format!("Failed to parse ask price: {}", e))?;
        let quantity = ask.1.parse()
            .map_err(|e| format!("Failed to parse ask quantity: {}", e))?;
        updates.push((price, quantity, false));
    }

    orderbook.batch_update(&updates);
    Ok(())
}

fn log_orderbook_state(orderbook: &OrderBook, context: &str) {
    println!("\nOrderbook State [{}]:", context);
    
    if let (Some((bid_price, _)), Some((ask_price, _))) = 
        (orderbook.best_bid_ask_prices(), orderbook.best_bid_ask_prices()) {
        println!("  Best Bid Price: {:.8}", bid_price);
        println!("  Best Ask Price: {:.8}", ask_price);
        println!("  Spread: {:.8}", ask_price - bid_price);
        println!("  Mid Price: {:.8}", (ask_price + bid_price) / 2.0);
    } else {
        println!("  No valid bid/ask levels found");
    }
} 