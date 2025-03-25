use futures::StreamExt;
use orderbook::OrderBook;
use orderbook::aggregated::{AggregatedOrderBook, Venue as VenueTrait};
use std::time::{Duration, Instant};
use venues::binance::{
    spot::{BinanceSpotPublicWebSocket, BinanceSpotPublicRest, WebSocketMessage as SpotWebSocketMessage},
};
use venues::okx::{OkxPublicWebSocket, OkxPublicRest, WebSocketMessage as OkxWebSocketMessage};
use venues::websockets::WebSocketConnection;
use venues::price_feed::UsdConverter;
use venues::bybit::{
    spot::{BybitSpotPublicWebSocket, BybitSpotPublicRest, WebSocketMessage as BybitSpotMessage},
};
use venues::venue::{BinanceSpot, Okx, BybitSpot};
use std::error::Error;
use std::collections::HashMap;
use std::sync::Arc;
use std::fmt::Display;

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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum VenueType {
    BinanceSpot,
    OKX,
    BybitSpot,
}

impl Display for VenueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VenueType::BinanceSpot => write!(f, "Binance Spot"),
            VenueType::OKX => write!(f, "OKX"),
            VenueType::BybitSpot => write!(f, "Bybit Spot"),
        }
    }
}

struct OrderBookManager {
    orderbooks: HashMap<VenueType, OrderBook>,
    metrics: HashMap<VenueType, VenueMetrics>,
    aggregated_ob: AggregatedOrderBook,
    usd_converter: UsdConverter,
}

impl OrderBookManager {
    fn new(price_precision: u32) -> Self {
        Self {
            orderbooks: HashMap::new(),
            metrics: HashMap::new(),
            aggregated_ob: AggregatedOrderBook::new(price_precision),
            usd_converter: UsdConverter::new(RATE_STALE_AFTER),
        }
    }

    fn add_venue(&mut self, venue: VenueType, price_precision: u32) {
        self.orderbooks.insert(venue.clone(), OrderBook::new(price_precision));
        self.metrics.insert(venue, VenueMetrics::default());
    }

    async fn initialize_snapshots(
        &mut self,
        spot_rest: &BinanceSpotPublicRest,
        okx_rest: &OkxPublicRest,
        bybit_spot_rest: &BybitSpotPublicRest,
        spot_symbol: &str,
        okx_symbol: &str,
        bybit_spot_symbol: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Initialize venues
        self.add_venue(VenueType::BinanceSpot, PRICE_PRECISION);
        self.add_venue(VenueType::OKX, PRICE_PRECISION);
        self.add_venue(VenueType::BybitSpot, PRICE_PRECISION);

        // Get initial snapshots
        let spot_snapshot = spot_rest.get_orderbook_snapshot(spot_symbol, Some(1000)).await
            .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
        let okx_snapshot = okx_rest.get_orderbook_snapshot(okx_symbol, Some(400)).await
            .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
        let bybit_spot_snapshot = bybit_spot_rest.get_orderbook_snapshot(bybit_spot_symbol, Some(50)).await
            .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
        
        // Process and apply snapshots
        if let Some(spot_ob) = self.orderbooks.get_mut(&VenueType::BinanceSpot) {
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
            
            spot_ob.apply_snapshot(spot_bids, spot_asks);
        }

        if let Some(okx_ob) = self.orderbooks.get_mut(&VenueType::OKX) {
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
            
            okx_ob.apply_snapshot(okx_bids, okx_asks);
        }

        if let Some(bybit_spot_ob) = self.orderbooks.get_mut(&VenueType::BybitSpot) {
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
            
            bybit_spot_ob.apply_snapshot(bybit_spot_bids, bybit_spot_asks);
        }
        
        Ok(())
    }

    async fn update_aggregated_orderbook(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Clear all venues from the aggregated orderbook
        for venue in self.orderbooks.keys() {
            match venue {
                VenueType::BinanceSpot => self.aggregated_ob.clear_venue(&BinanceSpot::Spot.to_string()),
                VenueType::OKX => self.aggregated_ob.clear_venue(&Okx::Spot.to_string()),
                VenueType::BybitSpot => self.aggregated_ob.clear_venue(&BybitSpot::Spot.to_string()),
            }
        }
        
        // Update aggregated orderbook with each venue's data
        for (venue, orderbook) in &self.orderbooks {
            let (bids, asks) = orderbook.get_depth(50);
            let (bid_price, ask_price) = orderbook.best_bid_ask_prices().unwrap_or((0.0, 0.0));
            
            // Add bids
            for (i, bid) in bids.iter().enumerate() {
                let price = bid_price - (i as f64 * 0.5);
                match venue {
                    VenueType::BinanceSpot => self.aggregated_ob.update(price, bid.size, true, &BinanceSpot::Spot),
                    VenueType::OKX => self.aggregated_ob.update(price, bid.size, true, &Okx::Spot),
                    VenueType::BybitSpot => self.aggregated_ob.update(price, bid.size, true, &BybitSpot::Spot),
                }
            }
            
            // Add asks
            for (i, ask) in asks.iter().enumerate() {
                let price = ask_price + (i as f64 * 0.5);
                match venue {
                    VenueType::BinanceSpot => self.aggregated_ob.update(price, ask.size, false, &BinanceSpot::Spot),
                    VenueType::OKX => self.aggregated_ob.update(price, ask.size, false, &Okx::Spot),
                    VenueType::BybitSpot => self.aggregated_ob.update(price, ask.size, false, &BybitSpot::Spot),
                }
            }
        }
        
        Ok(())
    }

    fn print_metrics(&self) {
        let metrics_data: Vec<_> = self.metrics.iter()
            .map(|(venue, metrics)| (match venue {
                VenueType::BinanceSpot => "Binance Spot",
                VenueType::OKX => "OKX",
                VenueType::BybitSpot => "Bybit Spot",
            }, metrics))
            .collect();
        print_metrics_table(&metrics_data);
    }

    fn print_aggregated_orderbook(&self, depth: usize) {
        // Get top levels for display
        let (bids, asks) = self.aggregated_ob.get_depth(depth);
        
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
            let sources = level.sources.iter()
                .map(|(source, size)| format!("{}: {:.3}", source, size))
                .collect::<Vec<_>>()
                .join(", ");
            println!("\x1b[31m{:<price_width$.8} | {:<size_width$.3} | {:<sources_width$}\x1b[0m",
                price, level.size, sources);
        }
        
        // Print spread if available
        if let Some((best_bid_price, best_ask_price)) = self.aggregated_ob.best_bid_ask_prices() {
            let spread = best_ask_price - best_bid_price;
            let spread_pct = (spread / best_bid_price) * 100.0;
            println!("{}", "-".repeat(total_width));
            println!("\x1b[33m{:<total_width$}\x1b[0m",
                format!("Spread: {:.8} ({:.4}%)", spread, spread_pct));
            println!("{}", "-".repeat(total_width));
        }
        
        // Print bids
        for (level, price) in bids {
            let sources = level.sources.iter()
                .map(|(source, size)| format!("{}: {:.3}", source, size))
                .collect::<Vec<_>>()
                .join(", ");
            println!("\x1b[32m{:<price_width$.8} | {:<size_width$.3} | {:<sources_width$}\x1b[0m",
                price, level.size, sources);
        }
        
        println!("{}", "=".repeat(total_width));
    }

    fn update_orderbook(&mut self, venue: &VenueType, price: f64, size: f64, is_bid: bool) {
        if let Some(orderbook) = self.orderbooks.get_mut(venue) {
            orderbook.update(price, size, is_bid);
        }
    }

    fn update_metrics(&mut self, venue: &VenueType, latency_ms: u64, best_bid: f64, best_ask: f64) {
        if let Some(metrics) = self.metrics.get_mut(venue) {
            metrics.update_latency(latency_ms);
            metrics.update_prices(best_bid, best_ask);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let spot_rest = BinanceSpotPublicRest::new();
    let okx_rest = OkxPublicRest::new();
    let bybit_spot_rest = BybitSpotPublicRest::new();
    
    let spot_symbol = "BTCUSDT";
    let okx_symbol = "BTC-USDT";
    let bybit_spot_symbol = "BTCUSDT";
    
    let mut orderbook_manager = OrderBookManager::new(PRICE_PRECISION);
    
    // Initialize snapshots
    orderbook_manager.initialize_snapshots(
        &spot_rest,
        &okx_rest,
        &bybit_spot_rest,
        spot_symbol,
        okx_symbol,
        bybit_spot_symbol,
    ).await?;
    
    // Initialize WebSocket clients
    let mut spot_ws = BinanceSpotPublicWebSocket::new();
    let mut okx_ws = OkxPublicWebSocket::new();
    let mut bybit_spot_ws = BybitSpotPublicWebSocket::new();
    
    // Connect to WebSockets
    spot_ws.connect().await?;
    okx_ws.connect().await?;
    bybit_spot_ws.connect().await?;
    
    // Subscribe to orderbook channels
    let spot_channel = format!("{}@depth@100ms", spot_symbol.to_lowercase());
    let okx_channel = format!("books:{}", okx_symbol);
    let bybit_spot_channel = format!("orderbook.50.{}", bybit_spot_symbol);
    
    spot_ws.subscribe(vec![spot_channel.clone()]).await?;
    okx_ws.subscribe(vec![okx_channel.clone()]).await?;
    bybit_spot_ws.subscribe(vec![bybit_spot_channel.clone()]).await?;
    
    // Get message streams
    let mut spot_stream = spot_ws.message_stream();
    let mut okx_stream = okx_ws.message_stream();
    let mut bybit_spot_stream = bybit_spot_ws.message_stream();
    
    // Process updates
    let mut last_print_time = Instant::now();
    let mut reconnect_attempts = 0;
    let mut backoff_ms = INITIAL_BACKOFF_MS;
    
    loop {
        tokio::select! {
            Some(spot_result) = spot_stream.next() => {
                match spot_result {
                    Ok(SpotWebSocketMessage::OrderBook(update)) => {
                        let start_time = Instant::now();
                        
                        // Process bids
                        for bid in &update.bids {
                            if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                                orderbook_manager.update_orderbook(&VenueType::BinanceSpot, price, size, true);
                            }
                        }
                        
                        // Process asks
                        for ask in &update.asks {
                            if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                                orderbook_manager.update_orderbook(&VenueType::BinanceSpot, price, size, false);
                            }
                        }
                        
                        // Update metrics
                        let latency = start_time.elapsed().as_millis() as u64;
                        if let Some((best_bid, best_ask)) = orderbook_manager.orderbooks.get(&VenueType::BinanceSpot)
                            .and_then(|ob| ob.best_bid_ask_prices()) {
                            orderbook_manager.update_metrics(&VenueType::BinanceSpot, latency, best_bid, best_ask);
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
                                    orderbook_manager.update_orderbook(&VenueType::OKX, price, size, true);
                                }
                            }
                            
                            // Process asks
                            for ask in &data.asks {
                                if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                                    orderbook_manager.update_orderbook(&VenueType::OKX, price, size, false);
                                }
                            }
                        }
                        
                        // Update metrics
                        let latency = start_time.elapsed().as_millis() as u64;
                        if let Some((best_bid, best_ask)) = orderbook_manager.orderbooks.get(&VenueType::OKX)
                            .and_then(|ob| ob.best_bid_ask_prices()) {
                            orderbook_manager.update_metrics(&VenueType::OKX, latency, best_bid, best_ask);
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
                                orderbook_manager.update_orderbook(&VenueType::BybitSpot, price, size, true);
                            }
                        }
                        
                        // Process asks
                        for ask in &update.data.a {
                            if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                                orderbook_manager.update_orderbook(&VenueType::BybitSpot, price, size, false);
                            }
                        }
                        
                        // Update metrics
                        let latency = start_time.elapsed().as_millis() as u64;
                        if let Some((best_bid, best_ask)) = orderbook_manager.orderbooks.get(&VenueType::BybitSpot)
                            .and_then(|ob| ob.best_bid_ask_prices()) {
                            orderbook_manager.update_metrics(&VenueType::BybitSpot, latency, best_bid, best_ask);
                        }
                    },
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error processing Bybit Spot message: {}", e);
                        return Err(Box::<dyn Error + Send + Sync>::from(e.to_string()));
                    }
                }
            },
            else => break,
        }
        
        // Update aggregated orderbook and print stats every second
        if last_print_time.elapsed() >= Duration::from_secs(1) {
            orderbook_manager.update_aggregated_orderbook().await?;
            
            // Clear screen
            print!("\x1B[2J\x1B[1;1H");
            
            // Print metrics and orderbook
            orderbook_manager.print_metrics();
            orderbook_manager.print_aggregated_orderbook(10);
            
            last_print_time = Instant::now();
        }
    }
    
    Ok(())
}
