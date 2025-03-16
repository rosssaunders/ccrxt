use futures::StreamExt;
use orderbook::OrderBook;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use venues::binance::spot::{BinanceSpotPublicWebSocket, BinanceSpotPublicRest, WebSocketMessage};
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
            OrderBookError::SequenceGap(msg) => write!(f, "Sequence gap detected: {}", msg),
            OrderBookError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            OrderBookError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl Error for OrderBookError {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let symbol = "BTCUSDT";
    let mut metrics = Metrics::default();
    let mut orderbook = OrderBook::new(8);  // 8 decimal places precision
    let mut reconnect_attempts = 0;
    let mut backoff_ms = INITIAL_BACKOFF_MS;

    loop {
        match maintain_orderbook(symbol, &mut orderbook, &mut metrics).await {
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

async fn maintain_orderbook(
    symbol: &str,
    orderbook: &mut OrderBook,
    metrics: &mut Metrics,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut ws = BinanceSpotPublicWebSocket::new();
    let rest = BinanceSpotPublicRest::new();
    
    // Connect to WebSocket
    ws.connect().await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Subscribe to depth stream
    ws.subscribe(vec![format!("{}@depth", symbol.to_lowercase())]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Get initial snapshot
    let snapshot = rest.get_orderbook_snapshot(symbol, Some(1000)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Initialize orderbook with snapshot
    orderbook.apply_snapshot(
        snapshot.bids.iter()
            .map(|level| (level.0.parse::<f64>().unwrap(), level.1.parse::<f64>().unwrap()))
            .collect(),
        snapshot.asks.iter()
            .map(|level| (level.0.parse::<f64>().unwrap(), level.1.parse::<f64>().unwrap()))
            .collect(),
    );
    
    let mut message_stream = ws.message_stream();
    let mut last_status = Instant::now();
    
    while let Some(msg) = message_stream.next().await {
        let update_time = Instant::now();
        
        match msg {
            Ok(WebSocketMessage::OrderBook(update)) => {
                // Process the update
                for level in &update.bids {
                    let price = level.0.parse()?;
                    let size = level.1.parse()?;
                    orderbook.update(price, size, true);
                }
                for level in &update.asks {
                    let price = level.0.parse()?;
                    let size = level.1.parse()?;
                    orderbook.update(price, size, false);
                }
                
                metrics.update_latency(update_time.elapsed().as_millis() as u64);
            }
            Ok(WebSocketMessage::Raw(_)) => continue,
            Err(e) => return Err(Box::new(OrderBookError::WebSocketError(e.to_string()))),
        }
        
        // Log status every 5 seconds
        if last_status.elapsed() >= Duration::from_secs(5) {
            log_orderbook_state(orderbook);
            metrics.log_status();
            last_status = Instant::now();
        }
    }
    
    Ok(())
}

fn log_orderbook_state(orderbook: &OrderBook) {
    println!("\nOrderbook State:");
    
    let (bids, asks) = orderbook.get_depth_with_prices(1);
    
    if let Some((bid_price, bid_level)) = bids.first() {
        println!("Best Bid: {} @ {:.8}", bid_price, bid_level.size);
    }
    
    if let Some((ask_price, ask_level)) = asks.first() {
        println!("Best Ask: {} @ {:.8}", ask_price, ask_level.size);
    }
    
    if let Some((bid_price, _)) = bids.first() {
        if let Some((ask_price, _)) = asks.first() {
            println!("Spread: {:.8}", ask_price - bid_price);
            println!("Mid Price: {:.8}", (ask_price + bid_price) / 2.0);
        }
    }
} 