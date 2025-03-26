use futures::StreamExt;
use orderbook::OrderBook;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use venues::binance::spot::{BinanceSpotPublicWebSocket, BinanceSpotPublicRest, WebSocketMessage, OrderBookUpdate};
use venues::websockets::{WebSocketConnection, BoxError};
use mapper::{BinanceSpotDecoder, OrderBookDecoder};

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

impl std::error::Error for OrderBookError {}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let rest_client = BinanceSpotPublicRest::new();
    let symbol = "BTCUSDT";
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

async fn maintain_orderbook(
    rest_client: &BinanceSpotPublicRest,
    symbol: &str,
    metrics: &mut Metrics,
) -> Result<(), BoxError> {
    let mut ws = BinanceSpotPublicWebSocket::new();
    let start_time = Instant::now();
    let decoder = BinanceSpotDecoder;
    
    // 1. Connect and subscribe
    println!("Connecting to WebSocket...");
    ws.connect().await.map_err(|e| OrderBookError::WebSocketError(e.to_string()))?;
    
    println!("Subscribing to depth stream...");
    ws.subscribe_depth(symbol).await
        .map_err(|e| OrderBookError::WebSocketError(e.to_string()))?;
    
    let mut stream = ws.message_stream();
    
    // 2. Wait for subscription confirmation with timeout
    println!("Waiting for subscription confirmation...");
    let mut subscription_confirmed = false;
    let mut buffered_updates = VecDeque::with_capacity(MAX_BUFFER_SIZE);
    
    while !subscription_confirmed {
        if let Ok(Some(msg)) = tokio::time::timeout(
            Duration::from_secs(5),
            stream.next()
        ).await {
            match msg {
                Ok(WebSocketMessage::Raw(value)) => {
                    if value.get("result").is_some() {
                        subscription_confirmed = true;
                        println!("Subscription confirmed in {}ms", start_time.elapsed().as_millis());
                    }
                }
                Ok(WebSocketMessage::OrderBook(update)) => {
                    if buffered_updates.len() < MAX_BUFFER_SIZE {
                        buffered_updates.push_back(update);
                    }
                }
                Err(e) => return Err(BoxError::from(e.to_string())),
            }
        } else {
            return Err(BoxError::from(OrderBookError::Timeout("Subscription confirmation timeout".into())));
        }
    }

    // 3. Get snapshot
    println!("Getting orderbook snapshot...");
    let snapshot = rest_client.get_orderbook_snapshot(symbol, Some(1000))
        .await
        .map_err(|e| OrderBookError::RestError(e.to_string()))?;
    println!("Got snapshot with last_update_id: {} in {}ms", 
             snapshot.last_update_id, start_time.elapsed().as_millis());
    
    let mut message_buffer = VecDeque::with_capacity(MAX_BUFFER_SIZE);
    message_buffer.extend(buffered_updates);
    
    // 4. Buffer additional messages with timeout
    println!("Buffering initial messages...");
    let buffer_start = Instant::now();
    while buffer_start.elapsed() < Duration::from_millis(MAX_BUFFER_TIME_MS) && 
          message_buffer.len() < MAX_BUFFER_SIZE {
        if let Ok(Some(msg)) = tokio::time::timeout(
            Duration::from_millis(BUFFER_TIMEOUT_MS),
            stream.next()
        ).await {
            match msg {
                Ok(WebSocketMessage::OrderBook(update)) => {
                    message_buffer.push_back(update);
                }
                Ok(WebSocketMessage::Raw(value)) => {
                    if value.get("result").is_none() {
                        println!("Unexpected raw message: {:?}", value);
                    }
                }
                Err(e) => return Err(BoxError::from(e.to_string())),
            }
        }
    }
    println!("Buffered {} messages in {}ms", 
             message_buffer.len(), buffer_start.elapsed().as_millis());

    // 5. Initialize orderbook
    let mut orderbook = OrderBook::new(8);
    let bids: Vec<(f64, f64)> = snapshot.bids.iter()
        .map(|level| {
            Ok((
                level.0.parse().map_err(|e| OrderBookError::ParseError(format!("Failed to parse bid price: {}", e)))?,
                level.1.parse().map_err(|e| OrderBookError::ParseError(format!("Failed to parse bid quantity: {}", e)))?
            ))
        })
        .collect::<Result<_, BoxError>>()?;
    let asks: Vec<(f64, f64)> = snapshot.asks.iter()
        .map(|level| {
            Ok((
                level.0.parse().map_err(|e| OrderBookError::ParseError(format!("Failed to parse ask price: {}", e)))?,
                level.1.parse().map_err(|e| OrderBookError::ParseError(format!("Failed to parse ask quantity: {}", e)))?
            ))
        })
        .collect::<Result<_, BoxError>>()?;
    
    orderbook.apply_snapshot(bids, asks);
    log_orderbook_state(&orderbook, "After snapshot");

    let mut last_update_id = snapshot.last_update_id;
    let mut first_message_processed = false;

    // 6. Process buffered messages
    println!("Processing {} buffered messages...", message_buffer.len());
    while let Some(update) = message_buffer.pop_front() {
        let update_time = Instant::now();
        println!("Processing update - U: {}, u: {}, last_update_id: {}", 
                update.first_update_id, update.final_update_id, last_update_id);
                
        if !first_message_processed {
            if update.final_update_id <= last_update_id {
                println!("Skipping old update: final_id({}) <= snapshot_id({})", 
                        update.final_update_id, last_update_id);
                continue;
            }

            if update.first_update_id > last_update_id + 1 || update.final_update_id < last_update_id + 1 {
                metrics.gaps_detected += 1;
                return Err(BoxError::from(OrderBookError::SequenceGap(format!(
                    "First update out of sequence: U({}) > lastUpdateId+1({}) || u({}) < lastUpdateId+1({})",
                    update.first_update_id, last_update_id + 1, update.final_update_id, last_update_id + 1
                ))));
            }

            first_message_processed = true;
            println!("First message processed successfully");
        } else {
            if update.first_update_id != last_update_id + 1 {
                metrics.gaps_detected += 1;
                return Err(BoxError::from(OrderBookError::SequenceGap(format!(
                    "Gap in sequence: first_update_id({}) != last_update_id+1({})",
                    update.first_update_id, last_update_id + 1
                ))));
            }
        }

        apply_update(&mut orderbook, &update, &decoder, &mut last_update_id)?;
        metrics.update_latency(update_time.elapsed().as_millis() as u64);
        
        if metrics.updates_processed % 100 == 0 {
            log_orderbook_state(&orderbook, "Periodic update");
            metrics.log_status();
        }
    }

    // 7. Process real-time messages
    println!("Processing real-time updates...");
    let mut last_status = Instant::now();
    
    while let Some(msg) = stream.next().await {
        match msg {
            Ok(WebSocketMessage::OrderBook(update)) => {
                let update_time = Instant::now();
                
                if update.first_update_id != last_update_id + 1 {
                    metrics.gaps_detected += 1;
                    return Err(BoxError::from(OrderBookError::SequenceGap(format!(
                        "Gap in sequence: first_update_id({}) != last_update_id+1({})",
                        update.first_update_id, last_update_id + 1
                    ))));
                }

                apply_update(&mut orderbook, &update, &decoder, &mut last_update_id)?;
                metrics.update_latency(update_time.elapsed().as_millis() as u64);

                // Log status every 5 seconds
                if last_status.elapsed() >= Duration::from_secs(5) {
                    log_orderbook_state(&orderbook, "Periodic update");
                    metrics.log_status();
                    last_status = Instant::now();
                }
            }
            Ok(WebSocketMessage::Raw(value)) => {
                if value.get("result").is_none() {
                    println!("Unexpected raw message: {:?}", value);
                }
            }
            Err(e) => return Err(BoxError::from(e.to_string())),
        }
    }

    println!("WebSocket connection closed, reinitializing...");
    ws.disconnect().await.map_err(|e| OrderBookError::WebSocketError(e.to_string()))?;
    Ok(())
}

fn log_orderbook_state(orderbook: &OrderBook, context: &str) {
    println!("Orderbook State [{}]:", context);
    
    if let (Some((bid_price, ask_price)), Some((best_bid, best_ask))) = 
        (orderbook.best_bid_ask_prices(), orderbook.best_bid_ask()) {
        println!("  Best Bid Price: {:.8}", bid_price);
        println!("  Best Bid Size: {}", best_bid.size);
        println!("  Best Ask Price: {:.8}", ask_price);
        println!("  Best Ask Size: {}", best_ask.size);
        println!("  Spread: {:.8}", ask_price - bid_price);
    } else {
        println!("  No valid bid/ask levels found");
    }
}

fn apply_update(
    orderbook: &mut OrderBook, 
    update: &OrderBookUpdate, 
    decoder: &BinanceSpotDecoder,
    last_update_id: &mut u64
) -> Result<(), BoxError> {
    let updates = decoder.decode_update(update);
    orderbook.batch_update(&updates);
    *last_update_id = update.final_update_id;
    Ok(())
} 