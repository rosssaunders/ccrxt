use futures::StreamExt;
use orderbook::OrderBook;
use std::time::{Duration, Instant};
use venues::okx::{OkxPublicWebSocket, OkxPublicRest, WebSocketMessage};
use venues::websockets::WebSocketConnection;
use std::error::Error;

const PRICE_PRECISION: u32 = 8;
const MAX_RECONNECT_ATTEMPTS: u32 = 5;
const INITIAL_BACKOFF_MS: u64 = 1000;
const MAX_BACKOFF_MS: u64 = 30000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let rest_client = OkxPublicRest::new();
    let mut ws_client = OkxPublicWebSocket::new();
    
    let inst_id = "BTC-USDT";
    let channel = format!("books:{}",inst_id);
    
    let mut orderbook = OrderBook::new(PRICE_PRECISION);
    let mut reconnect_attempts = 0;
    let mut backoff_ms = INITIAL_BACKOFF_MS;

    loop {
        match maintain_orderbook(&rest_client, &mut ws_client, &mut orderbook, inst_id, &channel).await {
            Ok(_) => {
                reconnect_attempts = 0;
                backoff_ms = INITIAL_BACKOFF_MS;
            }
            Err(e) => {
                eprintln!("Error in orderbook maintenance: {}", e);
                reconnect_attempts += 1;
                
                if reconnect_attempts > MAX_RECONNECT_ATTEMPTS {
                    return Err(format!("Max reconnect attempts ({}) reached", MAX_RECONNECT_ATTEMPTS).into());
                }
                
                // Exponential backoff with jitter
                let jitter = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.subsec_nanos() % 1000) as u64;
                let backoff = std::cmp::min(backoff_ms + jitter, MAX_BACKOFF_MS);
                eprintln!("Reconnecting in {}ms (attempt {}/{})", backoff, reconnect_attempts, MAX_RECONNECT_ATTEMPTS);
                tokio::time::sleep(Duration::from_millis(backoff)).await;
                backoff_ms = std::cmp::min(backoff_ms * 2, MAX_BACKOFF_MS);
                
                // Ensure disconnected before reconnecting
                let _ = ws_client.disconnect().await;
            }
        }
    }
}

async fn maintain_orderbook(
    rest_client: &OkxPublicRest,
    ws_client: &mut OkxPublicWebSocket,
    orderbook: &mut OrderBook,
    inst_id: &str,
    channel: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Initializing orderbook for {}", inst_id);
    
    // Connect to WebSocket
    if !ws_client.is_connected() {
        ws_client.connect().await
            .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
        println!("Connected to OKX WebSocket");
    }
    
    // Subscribe to orderbook channel
    ws_client.subscribe(vec![channel.to_string()]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    println!("Subscribed to channel: {}", channel);
    
    // Get initial snapshot
    let snapshot = rest_client.get_orderbook_snapshot(inst_id, Some(400)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    println!("Received initial snapshot with {} bids and {} asks", 
             snapshot.bids.len(), snapshot.asks.len());
    
    // Convert snapshot to orderbook format
    let bids: Vec<(f64, f64)> = snapshot.bids.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    let asks: Vec<(f64, f64)> = snapshot.asks.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    // Apply snapshot to orderbook
    orderbook.apply_snapshot(bids, asks);
    
    // Get message stream
    let mut message_stream = ws_client.message_stream();
    
    // Process updates
    let mut last_update_time = Instant::now();
    let mut updates_count = 0;
    
    println!("Starting to process updates...");
    
    while let Some(message_result) = message_stream.next().await {
        match message_result {
            Ok(WebSocketMessage::OrderBook(update)) => {
                for data in &update.data {
                    // Process bids
                    for bid in &data.bids {
                        if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                            orderbook.update(price, size, true);
                        }
                    }
                    
                    // Process asks
                    for ask in &data.asks {
                        if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                            orderbook.update(price, size, false);
                        }
                    }
                }
                
                updates_count += 1;
                
                // Log stats every second
                if last_update_time.elapsed() >= Duration::from_secs(1) {
                    if let Some((best_bid, best_ask)) = orderbook.best_bid_ask_prices() {
                        println!("Processed {} updates. Best bid: {:.8}, Best ask: {:.8}, Spread: {:.2}",
                            updates_count,
                            best_bid,
                            best_ask,
                            (best_ask - best_bid)
                        );
                    }
                    
                    updates_count = 0;
                    last_update_time = Instant::now();
                }
            },
            Ok(WebSocketMessage::Response(response)) => {
                println!("Received response: {:?}", response);
                if let Some(code) = &response.code {
                    if code != "0" {
                        eprintln!("Error response from OKX: {:?}", response);
                    }
                }
            },
            Ok(WebSocketMessage::Raw(value)) => {
                println!("Received raw message: {}", value);
            },
            Err(e) => {
                eprintln!("Error processing message: {}", e);
                return Err(e);
            }
        }
    }
    
    Ok(())
} 