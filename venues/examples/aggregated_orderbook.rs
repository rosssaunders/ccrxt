use futures::StreamExt;
use orderbook::{OrderBook, Level};
use orderbook::aggregated::{AggregatedOrderBook, AggregatedLevel, VenueSource};
use std::time::{Duration, Instant};
use venues::binance::{
    coinm::{BinanceCoinMPublicWebSocket, BinanceCoinMPublicRest, WebSocketMessage as CoinMWebSocketMessage},
    usdm::{BinanceUsdMPublicWebSocket, BinanceUsdMPublicRest, WebSocketMessage as UsdMWebSocketMessage},
    spot::{BinanceSpotPublicWebSocket, BinanceSpotPublicRest, WebSocketMessage as SpotWebSocketMessage},
};
use venues::okx::{OkxPublicWebSocket, OkxPublicRest, WebSocketMessage as OkxWebSocketMessage};
use venues::websockets::WebSocketConnection;
use venues::price_feed::UsdConverter;
use venues::bybit::{
    spot::{BybitSpotPublicWebSocket, BybitSpotPublicRest, WebSocketMessage as BybitSpotMessage},
    perp::{BybitPerpPublicWebSocket, BybitPerpPublicRest, WebSocketMessage as BybitPerpMessage},
};
use std::error::Error;

const PRICE_PRECISION: u32 = 8;
const MAX_RECONNECT_ATTEMPTS: u32 = 5;
const INITIAL_BACKOFF_MS: u64 = 1000;
const MAX_BACKOFF_MS: u64 = 30000;
const RATE_STALE_AFTER: Duration = Duration::from_secs(60);

// Performance metrics for each venue
struct VenueMetrics {
    updates_processed: u64,
    reconnects: u64,
    last_update_latency_ms: u64,
    avg_update_latency_ms: f64,
    best_bid: f64,
    best_ask: f64,
    last_update_time: std::time::SystemTime,
}

impl Default for VenueMetrics {
    fn default() -> Self {
        Self {
            updates_processed: 0,
            reconnects: 0,
            last_update_latency_ms: 0,
            avg_update_latency_ms: 0.0,
            best_bid: 0.0,
            best_ask: 0.0,
            last_update_time: std::time::SystemTime::now(),
        }
    }
}

impl VenueMetrics {
    fn update_latency(&mut self, latency_ms: u64) {
        self.last_update_latency_ms = latency_ms;
        self.avg_update_latency_ms = (self.avg_update_latency_ms * self.updates_processed as f64 + latency_ms as f64) / 
                                   (self.updates_processed as f64 + 1.0);
        self.updates_processed += 1;
        self.last_update_time = std::time::SystemTime::now();
    }

    fn update_prices(&mut self, best_bid: f64, best_ask: f64) {
        self.best_bid = best_bid;
        self.best_ask = best_ask;
    }
}

// Add new function to print metrics in a table format
fn print_metrics_table(metrics: &[(&str, &VenueMetrics)]) {
    // Calculate column widths
    let venue_width = 15;
    let updates_width = 12;
    let reconnects_width = 10;
    let last_latency_width = 12;
    let avg_latency_width = 12;
    let best_bid_width = 12;
    let best_ask_width = 12;
    let last_update_width = 15;
    let total_width = venue_width + updates_width + reconnects_width + last_latency_width + 
                     avg_latency_width + best_bid_width + best_ask_width + last_update_width + 16; // +16 for separators

    // Print header
    println!("\n{}", "=".repeat(total_width));
    println!("{:^total_width$}", "Exchange Metrics");
    println!("{}", "=".repeat(total_width));
    
    // Print column headers
    println!("{:<venue_width$} | {:>updates_width$} | {:>reconnects_width$} | {:>last_latency_width$} | {:>avg_latency_width$} | {:>best_bid_width$} | {:>best_ask_width$} | {:>last_update_width$}",
        "Venue", "Updates", "Reconnects", "Last Lat", "Avg Lat", "Best Bid", "Best Ask", "Age");
    println!("{}", "-".repeat(total_width));

    // Print each venue's metrics
    let now = std::time::SystemTime::now();
    for (venue, metric) in metrics {
        let age = now.duration_since(metric.last_update_time)
            .map(|d| {
                if d.as_secs() >= 60 {
                    format!("{:.1}m", d.as_secs_f64() / 60.0)
                } else if d.as_secs() > 0 {
                    format!("{:.1}s", d.as_secs_f64())
                } else {
                    format!("{}ms", d.as_millis())
                }
            })
            .unwrap_or_else(|_| "N/A".to_string());

        // Format the bid and ask with colors
        let colored_bid = format!("\x1b[32m{:.2}\x1b[0m", metric.best_bid);  // Green for bids
        let colored_ask = format!("\x1b[31m{:.2}\x1b[0m", metric.best_ask);  // Red for asks

        println!("{:<venue_width$} | {:>updates_width$} | {:>reconnects_width$} | {:>last_latency_width$} | {:>avg_latency_width$} | {:>best_bid_width$} | {:>best_ask_width$} | {:>last_update_width$}",
            venue,
            metric.updates_processed,
            metric.reconnects,
            format!("{}ms", metric.last_update_latency_ms),
            format!("{:.2}ms", metric.avg_update_latency_ms),
            colored_bid,
            colored_ask,
            age);
    }

    println!("{}", "=".repeat(total_width));
    println!(); // Add a blank line after the table
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let coinm_rest = BinanceCoinMPublicRest::new();
    let usdm_rest = BinanceUsdMPublicRest::new();
    let spot_rest = BinanceSpotPublicRest::new();
    let okx_rest = OkxPublicRest::new();
    let bybit_spot_rest = BybitSpotPublicRest::new();
    let bybit_perp_rest = BybitPerpPublicRest::new();
    
    let coinm_symbol = "BTCUSD_PERP";
    let usdm_symbol = "BTCUSDT";
    let spot_symbol = "BTCUSDT";
    let okx_symbol = "BTC-USDT";
    let bybit_spot_symbol = "BTCUSDT";
    let bybit_perp_symbol = "BTCUSDT";
    let usdc_usdt_symbol = "USDCUSDT";
    
    let mut coinm_metrics = VenueMetrics::default();
    let mut usdm_metrics = VenueMetrics::default();
    let mut spot_metrics = VenueMetrics::default();
    let mut okx_metrics = VenueMetrics::default();
    let mut bybit_spot_metrics = VenueMetrics::default();
    let mut bybit_perp_metrics = VenueMetrics::default();
    
    // Create USD converter
    let usd_converter = UsdConverter::new(RATE_STALE_AFTER);
    
    // Create and initialize the aggregated orderbook
    let mut aggregated_ob = AggregatedOrderBook::new(PRICE_PRECISION);
    
    // Initialize individual venue orderbooks
    let mut coinm_ob = OrderBook::new(PRICE_PRECISION);
    let mut usdm_ob = OrderBook::new(PRICE_PRECISION);
    let mut spot_ob = OrderBook::new(PRICE_PRECISION);
    let mut okx_ob = OrderBook::new(PRICE_PRECISION);
    let mut bybit_spot_ob = OrderBook::new(PRICE_PRECISION);
    let mut bybit_perp_ob = OrderBook::new(PRICE_PRECISION);
    
    let mut reconnect_attempts = 0;
    let mut backoff_ms = INITIAL_BACKOFF_MS;

    loop {
        match maintain_aggregated_orderbook(
            &coinm_rest,
            &usdm_rest,
            &spot_rest,
            &okx_rest,
            &bybit_spot_rest,
            &bybit_perp_rest,
            coinm_symbol,
            usdm_symbol,
            spot_symbol,
            okx_symbol,
            bybit_spot_symbol,
            bybit_perp_symbol,
            usdc_usdt_symbol,
            &mut aggregated_ob,
            &mut coinm_ob,
            &mut usdm_ob,
            &mut spot_ob,
            &mut okx_ob,
            &mut bybit_spot_ob,
            &mut bybit_perp_ob,
            &mut coinm_metrics,
            &mut usdm_metrics,
            &mut spot_metrics,
            &mut okx_metrics,
            &mut bybit_spot_metrics,
            &mut bybit_perp_metrics,
            &usd_converter,
        ).await {
            Ok(_) => {
                reconnect_attempts = 0;
                backoff_ms = INITIAL_BACKOFF_MS;
            }
            Err(e) => {
                eprintln!("Error in aggregated orderbook maintenance: {}", e);
                reconnect_attempts += 1;
                coinm_metrics.reconnects += 1;
                usdm_metrics.reconnects += 1;
                spot_metrics.reconnects += 1;
                okx_metrics.reconnects += 1;
                bybit_spot_metrics.reconnects += 1;
                bybit_perp_metrics.reconnects += 1;
                
                if reconnect_attempts > MAX_RECONNECT_ATTEMPTS {
                    return Err(format!("Max reconnect attempts ({}) reached", MAX_RECONNECT_ATTEMPTS).into());
                }
                
                // Exponential backoff with jitter
                let jitter = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.subsec_nanos() % 1000) as u64;
                let backoff = std::cmp::min(backoff_ms + jitter, MAX_BACKOFF_MS);
                eprintln!("Reconnecting in {}ms (attempt {}/{})", backoff, reconnect_attempts, MAX_RECONNECT_ATTEMPTS);
                tokio::time::sleep(Duration::from_millis(backoff)).await;
                backoff_ms = std::cmp::min(backoff_ms * 2, MAX_BACKOFF_MS);
            }
        }
    }
}

async fn maintain_aggregated_orderbook(
    coinm_rest: &BinanceCoinMPublicRest,
    usdm_rest: &BinanceUsdMPublicRest,
    spot_rest: &BinanceSpotPublicRest,
    okx_rest: &OkxPublicRest,
    bybit_spot_rest: &BybitSpotPublicRest,
    bybit_perp_rest: &BybitPerpPublicRest,
    coinm_symbol: &str,
    usdm_symbol: &str,
    spot_symbol: &str,
    okx_symbol: &str,
    bybit_spot_symbol: &str,
    bybit_perp_symbol: &str,
    usdc_usdt_symbol: &str,
    aggregated_ob: &mut AggregatedOrderBook,
    coinm_ob: &mut OrderBook,
    usdm_ob: &mut OrderBook,
    spot_ob: &mut OrderBook,
    okx_ob: &mut OrderBook,
    bybit_spot_ob: &mut OrderBook,
    bybit_perp_ob: &mut OrderBook,
    coinm_metrics: &mut VenueMetrics,
    usdm_metrics: &mut VenueMetrics,
    spot_metrics: &mut VenueMetrics,
    okx_metrics: &mut VenueMetrics,
    bybit_spot_metrics: &mut VenueMetrics,
    bybit_perp_metrics: &mut VenueMetrics,
    usd_converter: &UsdConverter,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize WebSocket clients
    let mut coinm_ws = BinanceCoinMPublicWebSocket::new();
    let mut usdm_ws = BinanceUsdMPublicWebSocket::new();
    let mut spot_ws = BinanceSpotPublicWebSocket::new();
    let mut okx_ws = OkxPublicWebSocket::new();
    let mut bybit_spot_ws = BybitSpotPublicWebSocket::new();
    let mut bybit_perp_ws = BybitPerpPublicWebSocket::new();
    
    // Connect to WebSockets
    coinm_ws.connect().await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    usdm_ws.connect().await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    spot_ws.connect().await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    okx_ws.connect().await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    bybit_spot_ws.connect().await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    bybit_perp_ws.connect().await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Subscribe to orderbook channels
    let coinm_channel = format!("{}@depth@100ms", coinm_symbol.to_lowercase());
    let usdm_channel = format!("{}@depth@100ms", usdm_symbol.to_lowercase());
    let spot_channel = format!("{}@depth@100ms", spot_symbol.to_lowercase());
    let okx_channel = format!("books:{}", okx_symbol);
    let bybit_spot_channel = format!("orderbook.50.{}", bybit_spot_symbol);
    let bybit_perp_channel = format!("orderbook.50.{}", bybit_perp_symbol);
    
    coinm_ws.subscribe(vec![coinm_channel.clone()]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    usdm_ws.subscribe(vec![usdm_channel.clone()]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    spot_ws.subscribe(vec![spot_channel.clone()]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    okx_ws.subscribe(vec![okx_channel.clone()]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    bybit_spot_ws.subscribe(vec![bybit_spot_channel.clone()]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    bybit_perp_ws.subscribe(vec![bybit_perp_channel.clone()]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Get initial snapshots
    let coinm_snapshot = coinm_rest.get_orderbook_snapshot(coinm_symbol, Some(1000)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    let usdm_snapshot = usdm_rest.get_orderbook_snapshot(usdm_symbol, Some(1000)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    let spot_snapshot = spot_rest.get_orderbook_snapshot(spot_symbol, Some(1000)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    let okx_snapshot = okx_rest.get_orderbook_snapshot(okx_symbol, Some(400)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Convert snapshots to orderbook format
    let coinm_bids: Vec<(f64, f64)> = coinm_snapshot.bids.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    let coinm_asks: Vec<(f64, f64)> = coinm_snapshot.asks.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    let usdm_bids: Vec<(f64, f64)> = usdm_snapshot.bids.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    let usdm_asks: Vec<(f64, f64)> = usdm_snapshot.asks.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    let spot_bids: Vec<(f64, f64)> = spot_snapshot.bids.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    let spot_asks: Vec<(f64, f64)> = spot_snapshot.asks.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    let okx_bids: Vec<(f64, f64)> = okx_snapshot.bids.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    let okx_asks: Vec<(f64, f64)> = okx_snapshot.asks.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    // Apply snapshots to individual orderbooks
    coinm_ob.apply_snapshot(coinm_bids, coinm_asks);
    usdm_ob.apply_snapshot(usdm_bids, usdm_asks);
    spot_ob.apply_snapshot(spot_bids, spot_asks);
    okx_ob.apply_snapshot(okx_bids, okx_asks);
    
    // Get initial snapshots for Bybit
    let bybit_spot_snapshot = bybit_spot_rest.get_orderbook_snapshot(bybit_spot_symbol, Some(50)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    let bybit_perp_snapshot = bybit_perp_rest.get_orderbook_snapshot(bybit_perp_symbol, Some(50)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Process Bybit Spot snapshot
    let bybit_spot_bids: Vec<(f64, f64)> = bybit_spot_snapshot.result.b.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    let bybit_spot_asks: Vec<(f64, f64)> = bybit_spot_snapshot.result.a.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    // Process Bybit Perp snapshot
    let bybit_perp_bids: Vec<(f64, f64)> = bybit_perp_snapshot.result.b.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    let bybit_perp_asks: Vec<(f64, f64)> = bybit_perp_snapshot.result.a.iter()
        .filter_map(|level| {
            let price = level.0.parse::<f64>().ok()?;
            let size = level.1.parse::<f64>().ok()?;
            Some((price, size))
        })
        .collect();
    
    // Apply snapshots to Bybit orderbooks
    bybit_spot_ob.apply_snapshot(bybit_spot_bids, bybit_spot_asks);
    bybit_perp_ob.apply_snapshot(bybit_perp_bids, bybit_perp_asks);
    
    // Update aggregated orderbook with initial snapshots
    update_aggregated_orderbook(
        aggregated_ob,
        coinm_ob,
        usdm_ob,
        spot_ob,
        okx_ob,
        bybit_spot_ob,
        bybit_perp_ob,
        usd_converter,
    ).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Get message streams
    let mut coinm_stream = coinm_ws.message_stream();
    let mut usdm_stream = usdm_ws.message_stream();
    let mut spot_stream = spot_ws.message_stream();
    let mut okx_stream = okx_ws.message_stream();
    let mut bybit_spot_stream = bybit_spot_ws.message_stream();
    let mut bybit_perp_stream = bybit_perp_ws.message_stream();
    
    // Process updates
    let mut last_print_time = Instant::now();
    
    loop {
        tokio::select! {
            Some(coinm_result) = coinm_stream.next() => {
                match coinm_result {
                    Ok(CoinMWebSocketMessage::OrderBook(update)) => {
                        let start_time = Instant::now();
                        
                        // Process bids
                        for bid in &update.bids {
                            if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                                coinm_ob.update(price, size, true);
                            }
                        }
                        
                        // Process asks
                        for ask in &update.asks {
                            if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                                coinm_ob.update(price, size, false);
                            }
                        }
                        
                        // Update metrics
                        let latency = start_time.elapsed().as_millis() as u64;
                        coinm_metrics.update_latency(latency);
                        if let Some((best_bid, best_ask)) = coinm_ob.best_bid_ask_prices() {
                            coinm_metrics.update_prices(best_bid, best_ask);
                        }
                    },
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error processing CoinM message: {}", e);
                        return Err(e);
                    }
                }
            },
            Some(usdm_result) = usdm_stream.next() => {
                match usdm_result {
                    Ok(UsdMWebSocketMessage::OrderBook(update)) => {
                        let start_time = Instant::now();
                        
                        // Process bids
                        for bid in &update.bids {
                            if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                                usdm_ob.update(price, size, true);
                            }
                        }
                        
                        // Process asks
                        for ask in &update.asks {
                            if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                                usdm_ob.update(price, size, false);
                            }
                        }
                        
                        // Update metrics
                        let latency = start_time.elapsed().as_millis() as u64;
                        usdm_metrics.update_latency(latency);
                        if let Some((best_bid, best_ask)) = usdm_ob.best_bid_ask_prices() {
                            usdm_metrics.update_prices(best_bid, best_ask);
                        }
                    },
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error processing UsdM message: {}", e);
                        return Err(e);
                    }
                }
            },
            Some(spot_result) = spot_stream.next() => {
                match spot_result {
                    Ok(SpotWebSocketMessage::OrderBook(update)) => {
                        let start_time = Instant::now();
                        
                        // Process bids
                        for bid in &update.bids {
                            if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                                spot_ob.update(price, size, true);
                            }
                        }
                        
                        // Process asks
                        for ask in &update.asks {
                            if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                                spot_ob.update(price, size, false);
                            }
                        }
                        
                        // Update metrics
                        let latency = start_time.elapsed().as_millis() as u64;
                        spot_metrics.update_latency(latency);
                        if let Some((best_bid, best_ask)) = spot_ob.best_bid_ask_prices() {
                            spot_metrics.update_prices(best_bid, best_ask);
                        }
                    },
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error processing Spot message: {}", e);
                        return Err(e);
                    }
                }
            },
            Some(okx_result) = okx_stream.next() => {
                match okx_result {
                    Ok(OkxWebSocketMessage::OrderBook(update)) => {
                        let start_time = Instant::now();
                        
                        for data in &update.data {
                            // Process bids
                            for bid in &data.bids {
                                if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                                    okx_ob.update(price, size, true);
                                }
                            }
                            
                            // Process asks
                            for ask in &data.asks {
                                if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                                    okx_ob.update(price, size, false);
                                }
                            }
                        }
                        
                        // Update metrics
                        let latency = start_time.elapsed().as_millis() as u64;
                        okx_metrics.update_latency(latency);
                        if let Some((best_bid, best_ask)) = okx_ob.best_bid_ask_prices() {
                            okx_metrics.update_prices(best_bid, best_ask);
                        }
                    },
                    Ok(OkxWebSocketMessage::Response(response)) => {
                        println!("Received OKX response: {:?}", response);
                    },
                    Ok(OkxWebSocketMessage::Raw(value)) => {
                        // Remove the debug print statement
                    },
                    Err(e) => {
                        eprintln!("Error processing OKX message: {}", e);
                        return Err(e);
                    }
                }
            },
            Some(bybit_spot_result) = bybit_spot_stream.next() => {
                match bybit_spot_result {
                    Ok(BybitSpotMessage::OrderBook(update)) => {
                        let start_time = Instant::now();
                        
                        // Process bids
                        for bid in &update.data.b {
                            if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                                bybit_spot_ob.update(price, size, true);
                            }
                        }
                        
                        // Process asks
                        for ask in &update.data.a {
                            if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                                bybit_spot_ob.update(price, size, false);
                            }
                        }
                        
                        // Update metrics
                        let latency = start_time.elapsed().as_millis() as u64;
                        bybit_spot_metrics.update_latency(latency);
                        if let Some((best_bid, best_ask)) = bybit_spot_ob.best_bid_ask_prices() {
                            bybit_spot_metrics.update_prices(best_bid, best_ask);
                        }
                    },
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error processing Bybit Spot message: {}", e);
                        return Err(Box::<dyn Error + Send + Sync>::from(e.to_string()));
                    }
                }
            },
            Some(bybit_perp_result) = bybit_perp_stream.next() => {
                match bybit_perp_result {
                    Ok(BybitPerpMessage::OrderBook(update)) => {
                        let start_time = Instant::now();
                        
                        // Process bids
                        for bid in &update.data.b {
                            if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                                bybit_perp_ob.update(price, size, true);
                            }
                        }
                        
                        // Process asks
                        for ask in &update.data.a {
                            if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                                bybit_perp_ob.update(price, size, false);
                            }
                        }
                        
                        // Update metrics
                        let latency = start_time.elapsed().as_millis() as u64;
                        bybit_perp_metrics.update_latency(latency);
                        if let Some((best_bid, best_ask)) = bybit_perp_ob.best_bid_ask_prices() {
                            bybit_perp_metrics.update_prices(best_bid, best_ask);
                        }
                    },
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error processing Bybit Perp message: {}", e);
                        return Err(Box::<dyn Error + Send + Sync>::from(e.to_string()));
                    }
                }
            },
            else => break,
        }
        
        // Update aggregated orderbook and print stats every second
        if last_print_time.elapsed() >= Duration::from_secs(1) {
            update_aggregated_orderbook(
                aggregated_ob,
                coinm_ob,
                usdm_ob,
                spot_ob,
                okx_ob,
                bybit_spot_ob,
                bybit_perp_ob,
                usd_converter,
            ).await
                .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
            
            // Clear screen
            print!("\x1B[2J\x1B[1;1H");
            
            // Print metrics in table format
            let metrics_data = [
                ("Binance CoinM", &coinm_metrics as &VenueMetrics),
                ("Binance UsdM", &usdm_metrics),
                ("Binance Spot", &spot_metrics),
                ("OKX", &okx_metrics),
                ("Bybit Spot", &bybit_spot_metrics),
                ("Bybit Perp", &bybit_perp_metrics),
            ];
            print_metrics_table(&metrics_data);
            
            // Print aggregated orderbook with improved formatting
            print_aggregated_orderbook(aggregated_ob, 10);
            
            last_print_time = Instant::now();
        }
    }
    
    Ok(())
}

async fn update_aggregated_orderbook(
    aggregated_ob: &mut AggregatedOrderBook,
    coinm_ob: &OrderBook,
    usdm_ob: &OrderBook,
    spot_ob: &OrderBook,
    okx_ob: &OrderBook,
    bybit_spot_ob: &OrderBook,
    bybit_perp_ob: &OrderBook,
    usd_converter: &UsdConverter,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Clear venues from the aggregated orderbook
    aggregated_ob.clear_venue(VenueSource::CoinM);
    aggregated_ob.clear_venue(VenueSource::UsdM);
    aggregated_ob.clear_venue(VenueSource::Spot);
    aggregated_ob.clear_venue(VenueSource::OKX);
    aggregated_ob.clear_venue(VenueSource::BybitSpot);
    aggregated_ob.clear_venue(VenueSource::BybitPerp);
    
    
    // Get depth from each venue
    let (coinm_bids, coinm_asks) = coinm_ob.get_depth(50);
    let (usdm_bids, usdm_asks) = usdm_ob.get_depth(50);
    let (spot_bids, spot_asks) = spot_ob.get_depth(50);
    let (okx_bids, okx_asks) = okx_ob.get_depth(50);
    
    // Get price points
    let (coinm_bid_prices, coinm_ask_prices) = coinm_ob.best_bid_ask_prices().unwrap_or((0.0, 0.0));
    let (usdm_bid_prices, usdm_ask_prices) = usdm_ob.best_bid_ask_prices().unwrap_or((0.0, 0.0));
    let (spot_bid_prices, spot_ask_prices) = spot_ob.best_bid_ask_prices().unwrap_or((0.0, 0.0));
    let (okx_bid_prices, okx_ask_prices) = okx_ob.best_bid_ask_prices().unwrap_or((0.0, 0.0));
    
    // Add CoinM levels (USD denominated, needs conversion)
    for (i, bid) in coinm_bids.iter().enumerate() {
        let price = coinm_bid_prices - (i as f64 * 0.5); // Approximate price based on best bid
        let converted_price = if let Some(converted) = usd_converter.convert_usd_to_usdt(price).await {
            converted
        } else {
            price // Fallback to original price if conversion fails
        };
        aggregated_ob.update(converted_price, bid.size, true, VenueSource::CoinM);
    }
    
    for (i, ask) in coinm_asks.iter().enumerate() {
        let price = coinm_ask_prices + (i as f64 * 0.5); // Approximate price based on best ask
        let converted_price = if let Some(converted) = usd_converter.convert_usd_to_usdt(price).await {
            converted
        } else {
            price // Fallback to original price if conversion fails
        };
        aggregated_ob.update(converted_price, ask.size, false, VenueSource::CoinM);
    }
    
    // Add UsdM levels (USDT denominated)
    for (i, bid) in usdm_bids.iter().enumerate() {
        let price = usdm_bid_prices - (i as f64 * 0.5); // Approximate price based on best bid
        aggregated_ob.update(price, bid.size, true, VenueSource::UsdM);
    }
    
    for (i, ask) in usdm_asks.iter().enumerate() {
        let price = usdm_ask_prices + (i as f64 * 0.5); // Approximate price based on best ask
        aggregated_ob.update(price, ask.size, false, VenueSource::UsdM);
    }
    
    // Add Spot levels (USDT denominated)
    for (i, bid) in spot_bids.iter().enumerate() {
        let price = spot_bid_prices - (i as f64 * 0.5); // Approximate price based on best bid
        aggregated_ob.update(price, bid.size, true, VenueSource::Spot);
    }
    
    for (i, ask) in spot_asks.iter().enumerate() {
        let price = spot_ask_prices + (i as f64 * 0.5); // Approximate price based on best ask
        aggregated_ob.update(price, ask.size, false, VenueSource::Spot);
    }
    
    // Add OKX levels (USDT denominated)
    for (i, bid) in okx_bids.iter().enumerate() {
        let price = okx_bid_prices - (i as f64 * 0.5); // Approximate price based on best bid
        aggregated_ob.update(price, bid.size, true, VenueSource::OKX); // Using OKX as the source type
    }
    
    for (i, ask) in okx_asks.iter().enumerate() {
        let price = okx_ask_prices + (i as f64 * 0.5); // Approximate price based on best ask
        aggregated_ob.update(price, ask.size, false, VenueSource::OKX); // Using OKX as the source type
    }
    
    // Get Bybit depth and prices
    let (bybit_spot_bids, bybit_spot_asks) = bybit_spot_ob.get_depth(50);
    let (bybit_perp_bids, bybit_perp_asks) = bybit_perp_ob.get_depth(50);
    
    let (bybit_spot_bid_prices, bybit_spot_ask_prices) = bybit_spot_ob.best_bid_ask_prices().unwrap_or((0.0, 0.0));
    let (bybit_perp_bid_prices, bybit_perp_ask_prices) = bybit_perp_ob.best_bid_ask_prices().unwrap_or((0.0, 0.0));
    
    // Add Bybit Spot levels (USDT denominated)
    for (i, bid) in bybit_spot_bids.iter().enumerate() {
        let price = bybit_spot_bid_prices - (i as f64 * 0.5);
        aggregated_ob.update(price, bid.size, true, VenueSource::BybitSpot);
    }
    
    for (i, ask) in bybit_spot_asks.iter().enumerate() {
        let price = bybit_spot_ask_prices + (i as f64 * 0.5);
        aggregated_ob.update(price, ask.size, false, VenueSource::BybitSpot);
    }
    
    // Add Bybit Perp levels (USDT denominated)
    for (i, bid) in bybit_perp_bids.iter().enumerate() {
        let price = bybit_perp_bid_prices - (i as f64 * 0.5);
        aggregated_ob.update(price, bid.size, true, VenueSource::BybitPerp);
    }
    
    for (i, ask) in bybit_perp_asks.iter().enumerate() {
        let price = bybit_perp_ask_prices + (i as f64 * 0.5);
        aggregated_ob.update(price, ask.size, false, VenueSource::BybitPerp);
    }
    
    Ok(())
}

// Add new function to print the orderbook in a TUI-like format
fn print_aggregated_orderbook(aggregated_ob: &AggregatedOrderBook, depth: usize) {
    // Get top levels for display
    let (bids, asks) = aggregated_ob.get_depth(depth);
    
    // Calculate column widths
    let price_width = 12;
    let size_width = 12;
    let sources_width = 40;
    let total_width = price_width + size_width + sources_width + 4; // +4 for separators
    
    // Print header
    println!("\n{}", "=".repeat(total_width));
    println!("{:^total_width$}", "Aggregated Orderbook");
    println!("{}", "=".repeat(total_width));
    println!("{:<price_width$} | {:<size_width$} | {:<sources_width$}", 
        "Price", "Size", "Sources");
    println!("{}", "-".repeat(total_width));
    
    // Print asks in reverse order (highest to lowest)
    let mut asks_reversed: Vec<_> = asks.into_iter().collect();
    asks_reversed.reverse();
    for (level, price) in asks_reversed {
        println!("\x1b[31m{:<price_width$.8} | {:<size_width$.3} | {:<sources_width$}\x1b[0m",
            price, level.size,
            format!("{:?}", level.sources));
    }
    
    // Print spread if available
    if let Some((best_bid_price, best_ask_price)) = aggregated_ob.best_bid_ask_prices() {
        let spread = best_ask_price - best_bid_price;
        let spread_pct = (spread / best_bid_price) * 100.0;
        println!("{}", "-".repeat(total_width));
        println!("\x1b[33m{:<total_width$}\x1b[0m",
            format!("Spread: {:.8} ({:.4}%)", spread, spread_pct));
        println!("{}", "-".repeat(total_width));
    }
    
    // Print bids
    for (level, price) in bids {
        println!("\x1b[32m{:<price_width$.8} | {:<size_width$.3} | {:<sources_width$}\x1b[0m",
            price, level.size,
            format!("{:?}", level.sources));
    }
    
    println!("{}", "=".repeat(total_width));
}
