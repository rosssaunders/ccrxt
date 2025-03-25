use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::StreamExt;
use orderbook::{OrderBook, Level};
use parking_lot::Mutex;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs},
    Terminal,
};
use std::{
    collections::HashMap,
    error::Error,
    io,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::mpsc;
use venues::binance::{
    spot::{BinanceSpotPublicWebSocket, BinanceSpotPublicRest, WebSocketMessage as BinanceSpotMessage},
    usdm::{BinanceUsdMPublicWebSocket, BinanceUsdMPublicRest, WebSocketMessage as BinanceUsdMMessage},
    coinm::{BinanceCoinMPublicWebSocket, BinanceCoinMPublicRest, WebSocketMessage as BinanceCoinMMessage},
};
use venues::bybit::{
    spot::{BybitSpotPublicWebSocket, BybitSpotPublicRest, WebSocketMessage as BybitSpotMessage},
    perp::{BybitPerpPublicWebSocket, BybitPerpPublicRest, WebSocketMessage as BybitPerpMessage},
};
use venues::okx::{OkxPublicWebSocket, OkxPublicRest, WebSocketMessage as OkxMessage};
use venues::websockets::WebSocketConnection;

const PRICE_PRECISION: u32 = 8;
const REFRESH_RATE: Duration = Duration::from_millis(100);
const DEPTH: usize = 10;

// Exchange types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Exchange {
    BinanceSpot,
    BinanceUsdM,
    BinanceCoinM,
    Okx,
    BybitSpot,
    BybitPerp,
}

impl Exchange {
    fn name(&self) -> &'static str {
        match self {
            Exchange::BinanceSpot => "Binance Spot",
            Exchange::BinanceUsdM => "Binance USDM",
            Exchange::BinanceCoinM => "Binance COINM",
            Exchange::Okx => "OKX",
            Exchange::BybitSpot => "Bybit Spot",
            Exchange::BybitPerp => "Bybit Perp",
        }
    }
    
    fn all() -> Vec<Exchange> {
        vec![
            Exchange::BinanceSpot, 
            Exchange::BinanceUsdM, 
            Exchange::BinanceCoinM, 
            Exchange::Okx,
            Exchange::BybitSpot,
            Exchange::BybitPerp,
        ]
    }
}

// Performance metrics for each venue
#[derive(Default, Clone)]
struct VenueMetrics {
    updates_processed: u64,
    reconnects: u64,
    last_update_latency_ms: u64,
    avg_update_latency_ms: f64,
    max_update_latency_ms: u64,
    last_update_time: Option<Instant>,
}

impl VenueMetrics {
    fn update_latency(&mut self, latency_ms: u64) {
        self.last_update_latency_ms = latency_ms;
        self.avg_update_latency_ms = (self.avg_update_latency_ms * self.updates_processed as f64 + latency_ms as f64) / 
                                   (self.updates_processed as f64 + 1.0);
        self.max_update_latency_ms = self.max_update_latency_ms.max(latency_ms);
        self.updates_processed += 1;
        self.last_update_time = Some(Instant::now());
    }
}

// App state
struct App {
    orderbooks: HashMap<Exchange, OrderBook>,
    metrics: HashMap<Exchange, VenueMetrics>,
    symbol: String,
    current_tab: usize,
    exchanges: Vec<Exchange>,
    running: bool,
}

impl App {
    fn new(symbol: &str) -> Self {
        let exchanges = Exchange::all();
        let mut orderbooks = HashMap::new();
        let mut metrics = HashMap::new();
        
        for &exchange in &exchanges {
            orderbooks.insert(exchange, OrderBook::new(PRICE_PRECISION));
            metrics.insert(exchange, VenueMetrics::default());
        }
        
        Self {
            orderbooks,
            metrics,
            symbol: symbol.to_string(),
            current_tab: 0,
            exchanges,
            running: true,
        }
    }
    
    fn next_tab(&mut self) {
        self.current_tab = (self.current_tab + 1) % self.exchanges.len();
    }
    
    fn previous_tab(&mut self) {
        if self.current_tab > 0 {
            self.current_tab -= 1;
        } else {
            self.current_tab = self.exchanges.len() - 1;
        }
    }
    
    fn current_exchange(&self) -> Exchange {
        self.exchanges[self.current_tab]
    }
}

// Message types for the UI thread
enum UiMessage {
    OrderBookUpdate(Exchange),
    Metrics(Exchange, VenueMetrics),
    Error(String),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Create app and run it
    let symbol = "BTCUSDT";
    let app = Arc::new(Mutex::new(App::new(symbol)));
    
    // Create channels for communication
    let (tx, rx) = mpsc::channel(100);
    
    // Create a separate task group for data collection
    let mut handles = Vec::new();
    
    // Spawn Binance Spot task
    {
        let app_clone = Arc::clone(&app);
        let symbol_clone = symbol.to_string();
        let tx_clone = tx.clone();
        
        let handle = tokio::task::spawn(async move {
            match run_binance_spot(&app_clone, &symbol_clone, tx_clone).await {
                Ok(_) => {},
                Err(e) => eprintln!("Binance Spot error: {}", e),
            }
        });
        
        handles.push(handle);
    }
    
    // Spawn Binance USDM task
    {
        let app_clone = Arc::clone(&app);
        let symbol_clone = symbol.to_string();
        let tx_clone = tx.clone();
        
        let handle = tokio::task::spawn(async move {
            match run_binance_usdm(&app_clone, &symbol_clone, tx_clone).await {
                Ok(_) => {},
                Err(e) => eprintln!("Binance USDM error: {}", e),
            }
        });
        
        handles.push(handle);
    }
    
    // Spawn Binance COINM task
    {
        let app_clone = Arc::clone(&app);
        let tx_clone = tx.clone();
        
        let handle = tokio::task::spawn(async move {
            // CoinM uses a different symbol format
            let coinm_symbol = "BTCUSD_PERP";
            match run_binance_coinm(&app_clone, coinm_symbol, tx_clone).await {
                Ok(_) => {},
                Err(e) => eprintln!("Binance COINM error: {}", e),
            }
        });
        
        handles.push(handle);
    }
    
    // Spawn OKX task
    {
        let app_clone = Arc::clone(&app);
        let tx_clone = tx.clone();
        
        let handle = tokio::task::spawn(async move {
            // OKX uses a slightly different symbol format
            let okx_symbol = "BTC-USDT";
            match run_okx(&app_clone, okx_symbol, tx_clone).await {
                Ok(_) => {},
                Err(e) => eprintln!("OKX error: {}", e),
            }
        });
        
        handles.push(handle);
    }
    
    // Spawn Bybit Spot task
    {
        let app_clone = Arc::clone(&app);
        let symbol_clone = symbol.to_string();
        let tx_clone = tx.clone();
        
        let handle = tokio::task::spawn(async move {
            match run_bybit_spot(&app_clone, &symbol_clone, tx_clone).await {
                Ok(_) => {},
                Err(e) => eprintln!("Bybit Spot error: {}", e),
            }
        });
        
        handles.push(handle);
    }
    
    // Spawn Bybit Perp task
    {
        let app_clone = Arc::clone(&app);
        let symbol_clone = symbol.to_string();
        let tx_clone = tx.clone();
        
        let handle = tokio::task::spawn(async move {
            match run_bybit_perp(&app_clone, &symbol_clone, tx_clone).await {
                Ok(_) => {},
                Err(e) => eprintln!("Bybit Perp error: {}", e),
            }
        });
        
        handles.push(handle);
    }
    
    // Run the UI
    let res = run_ui(&mut terminal, app, rx);
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    
    // Abort all tasks (we don't need to wait for them)
    for handle in handles {
        handle.abort();
    }
    
    Ok(())
}

async fn run_binance_spot(
    app: &Arc<Mutex<App>>, 
    symbol: &str,
    tx: mpsc::Sender<UiMessage>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let rest_client = BinanceSpotPublicRest::new();
    let mut ws = BinanceSpotPublicWebSocket::new();
    
    // Connect to WebSocket
    ws.connect().await.map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Subscribe to depth stream
    ws.subscribe(vec![format!("{}@depth@100ms", symbol.to_lowercase())]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Get initial snapshot
    let snapshot = rest_client.get_orderbook_snapshot(symbol, Some(1000)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Process snapshot data outside of the mutex lock
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
    
    // Initialize orderbook with snapshot - minimize time holding the lock
    {
        let mut app_guard = app.lock();
        if let Some(orderbook) = app_guard.orderbooks.get_mut(&Exchange::BinanceSpot) {
            orderbook.apply_snapshot(bids, asks);
        }
    } // Lock is released here
    
    // Notify UI after releasing the lock
    tx.send(UiMessage::OrderBookUpdate(Exchange::BinanceSpot)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    let mut message_stream = ws.message_stream();
    
    // Check if we should continue running
    while {
        let running = app.lock().running;
        running
    } {
        if let Some(msg) = message_stream.next().await {
            let update_time = Instant::now();
            
            match msg {
                Ok(BinanceSpotMessage::OrderBook(update)) => {
                    // Process update data outside of the mutex lock
                    let bids: Vec<(f64, f64, bool)> = update.bids.iter()
                        .filter_map(|bid| {
                            let price = bid.0.parse::<f64>().ok()?;
                            let size = bid.1.parse::<f64>().ok()?;
                            Some((price, size, true)) // true for bids
                        })
                        .collect();
                    
                    let asks: Vec<(f64, f64, bool)> = update.asks.iter()
                        .filter_map(|ask| {
                            let price = ask.0.parse::<f64>().ok()?;
                            let size = ask.1.parse::<f64>().ok()?;
                            Some((price, size, false)) // false for asks
                        })
                        .collect();
                    
                    // Apply updates with minimal lock time
                    let metrics_clone = {
                        let mut app_guard = app.lock();
                        if let Some(orderbook) = app_guard.orderbooks.get_mut(&Exchange::BinanceSpot) {
                            // Apply all updates at once
                            for (price, size, is_bid) in bids.iter().chain(asks.iter()) {
                                orderbook.update(*price, *size, *is_bid);
                            }
                            
                            // Update metrics
                            let latency = update_time.elapsed().as_millis() as u64;
                            if let Some(metrics) = app_guard.metrics.get_mut(&Exchange::BinanceSpot) {
                                metrics.update_latency(latency);
                                Some(metrics.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }; // Lock is released here
                    
                    // Send metrics update if available
                    if let Some(metrics) = metrics_clone {
                        tx.send(UiMessage::Metrics(Exchange::BinanceSpot, metrics)).await
                            .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                    }
                    
                    // Notify UI of orderbook update
                    tx.send(UiMessage::OrderBookUpdate(Exchange::BinanceSpot)).await
                        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                },
                Ok(_) => {},
                Err(e) => {
                    tx.send(UiMessage::Error(format!("Binance Spot error: {}", e))).await
                        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                    return Err(Box::<dyn Error + Send + Sync>::from(e.to_string()));
                }
            }
        }
    }
    
    Ok(())
}

async fn run_binance_usdm(
    app: &Arc<Mutex<App>>, 
    symbol: &str,
    tx: mpsc::Sender<UiMessage>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let rest_client = BinanceUsdMPublicRest::new();
    let mut ws = BinanceUsdMPublicWebSocket::new();
    
    // Connect to WebSocket
    ws.connect().await.map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Subscribe to depth stream
    let channel = format!("{}@depth@100ms", symbol.to_lowercase());
    ws.subscribe(vec![channel]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Get initial snapshot
    let snapshot = rest_client.get_orderbook_snapshot(symbol, Some(1000)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Process snapshot data outside of the mutex lock
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
    
    // Initialize orderbook with snapshot - minimize time holding the lock
    {
        let mut app_guard = app.lock();
        if let Some(orderbook) = app_guard.orderbooks.get_mut(&Exchange::BinanceUsdM) {
            orderbook.apply_snapshot(bids, asks);
        }
    } // Lock is released here
    
    // Notify UI after releasing the lock
    tx.send(UiMessage::OrderBookUpdate(Exchange::BinanceUsdM)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    let mut message_stream = ws.message_stream();
    
    // Check if we should continue running
    while {
        let running = app.lock().running;
        running
    } {
        if let Some(msg) = message_stream.next().await {
            let update_time = Instant::now();
            
            match msg {
                Ok(BinanceUsdMMessage::OrderBook(update)) => {
                    // Process update data outside of the mutex lock
                    let bids: Vec<(f64, f64, bool)> = update.bids.iter()
                        .filter_map(|bid| {
                            let price = bid.0.parse::<f64>().ok()?;
                            let size = bid.1.parse::<f64>().ok()?;
                            Some((price, size, true)) // true for bids
                        })
                        .collect();
                    
                    let asks: Vec<(f64, f64, bool)> = update.asks.iter()
                        .filter_map(|ask| {
                            let price = ask.0.parse::<f64>().ok()?;
                            let size = ask.1.parse::<f64>().ok()?;
                            Some((price, size, false)) // false for asks
                        })
                        .collect();
                    
                    // Apply updates with minimal lock time
                    let metrics_clone = {
                        let mut app_guard = app.lock();
                        if let Some(orderbook) = app_guard.orderbooks.get_mut(&Exchange::BinanceUsdM) {
                            // Apply all updates at once
                            for (price, size, is_bid) in bids.iter().chain(asks.iter()) {
                                orderbook.update(*price, *size, *is_bid);
                            }
                            
                            // Update metrics
                            let latency = update_time.elapsed().as_millis() as u64;
                            if let Some(metrics) = app_guard.metrics.get_mut(&Exchange::BinanceUsdM) {
                                metrics.update_latency(latency);
                                Some(metrics.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }; // Lock is released here
                    
                    // Send metrics update if available
                    if let Some(metrics) = metrics_clone {
                        tx.send(UiMessage::Metrics(Exchange::BinanceUsdM, metrics)).await
                            .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                    }
                    
                    // Notify UI of orderbook update
                    tx.send(UiMessage::OrderBookUpdate(Exchange::BinanceUsdM)).await
                        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                },
                Ok(_) => {},
                Err(e) => {
                    tx.send(UiMessage::Error(format!("Binance USDM error: {}", e))).await
                        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                    return Err(Box::<dyn Error + Send + Sync>::from(e.to_string()));
                }
            }
        }
    }
    
    Ok(())
}

async fn run_binance_coinm(
    app: &Arc<Mutex<App>>, 
    symbol: &str,
    tx: mpsc::Sender<UiMessage>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let rest_client = BinanceCoinMPublicRest::new();
    let mut ws = BinanceCoinMPublicWebSocket::new();
    
    // Connect to WebSocket
    ws.connect().await.map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Subscribe to depth stream
    ws.subscribe(vec![format!("{}@depth@100ms", symbol.to_lowercase())]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Get initial snapshot
    let snapshot = rest_client.get_orderbook_snapshot(symbol, Some(1000)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Process snapshot data outside of the mutex lock
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
    
    // Initialize orderbook with snapshot - minimize time holding the lock
    {
        let mut app_guard = app.lock();
        if let Some(orderbook) = app_guard.orderbooks.get_mut(&Exchange::BinanceCoinM) {
            orderbook.apply_snapshot(bids, asks);
        }
    } // Lock is released here
    
    // Notify UI after releasing the lock
    tx.send(UiMessage::OrderBookUpdate(Exchange::BinanceCoinM)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    let mut message_stream = ws.message_stream();
    
    // Check if we should continue running
    while {
        let running = app.lock().running;
        running
    } {
        if let Some(msg) = message_stream.next().await {
            let update_time = Instant::now();
            
            match msg {
                Ok(BinanceCoinMMessage::OrderBook(update)) => {
                    // Process update data outside of the mutex lock
                    let bids: Vec<(f64, f64, bool)> = update.bids.iter()
                        .filter_map(|bid| {
                            let price = bid.0.parse::<f64>().ok()?;
                            let size = bid.1.parse::<f64>().ok()?;
                            Some((price, size, true)) // true for bids
                        })
                        .collect();
                    
                    let asks: Vec<(f64, f64, bool)> = update.asks.iter()
                        .filter_map(|ask| {
                            let price = ask.0.parse::<f64>().ok()?;
                            let size = ask.1.parse::<f64>().ok()?;
                            Some((price, size, false)) // false for asks
                        })
                        .collect();
                    
                    // Apply updates with minimal lock time
                    let metrics_clone = {
                        let mut app_guard = app.lock();
                        if let Some(orderbook) = app_guard.orderbooks.get_mut(&Exchange::BinanceCoinM) {
                            // Apply all updates at once
                            for (price, size, is_bid) in bids.iter().chain(asks.iter()) {
                                orderbook.update(*price, *size, *is_bid);
                            }
                            
                            // Update metrics
                            let latency = update_time.elapsed().as_millis() as u64;
                            if let Some(metrics) = app_guard.metrics.get_mut(&Exchange::BinanceCoinM) {
                                metrics.update_latency(latency);
                                Some(metrics.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }; // Lock is released here
                    
                    // Send metrics update if available
                    if let Some(metrics) = metrics_clone {
                        tx.send(UiMessage::Metrics(Exchange::BinanceCoinM, metrics)).await
                            .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                    }
                    
                    // Notify UI of orderbook update
                    tx.send(UiMessage::OrderBookUpdate(Exchange::BinanceCoinM)).await
                        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                },
                Ok(_) => {},
                Err(e) => {
                    tx.send(UiMessage::Error(format!("Binance COINM error: {}", e))).await
                        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                    return Err(Box::<dyn Error + Send + Sync>::from(e.to_string()));
                }
            }
        }
    }
    
    Ok(())
}

async fn run_okx(
    app: &Arc<Mutex<App>>, 
    symbol: &str,
    tx: mpsc::Sender<UiMessage>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let rest_client = OkxPublicRest::new();
    let mut ws = OkxPublicWebSocket::new();
    
    // Connect to WebSocket
    ws.connect().await.map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Subscribe to depth stream
    ws.subscribe(vec![format!("books:{}", symbol)]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Get initial snapshot
    let snapshot = rest_client.get_orderbook_snapshot(symbol, Some(400)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Process snapshot data outside of the mutex lock
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
    
    // Initialize orderbook with snapshot - minimize time holding the lock
    {
        let mut app_guard = app.lock();
        if let Some(orderbook) = app_guard.orderbooks.get_mut(&Exchange::Okx) {
            orderbook.apply_snapshot(bids, asks);
        }
    } // Lock is released here
    
    // Notify UI after releasing the lock
    tx.send(UiMessage::OrderBookUpdate(Exchange::Okx)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    let mut message_stream = ws.message_stream();
    
    // Check if we should continue running
    while {
        let running = app.lock().running;
        running
    } {
        if let Some(msg) = message_stream.next().await {
            let update_time = Instant::now();
            
            match msg {
                Ok(OkxMessage::OrderBook(update)) => {
                    // Process update data outside of the mutex lock
                    let mut all_updates = Vec::new();
                    
                    for data in &update.data {
                        // Process bids
                        for bid in &data.bids {
                            if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                                all_updates.push((price, size, true)); // true for bids
                            }
                        }
                        
                        // Process asks
                        for ask in &data.asks {
                            if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                                all_updates.push((price, size, false)); // false for asks
                            }
                        }
                    }
                    
                    // Apply updates with minimal lock time
                    let metrics_clone = {
                        let mut app_guard = app.lock();
                        if let Some(orderbook) = app_guard.orderbooks.get_mut(&Exchange::Okx) {
                            // Apply all updates at once
                            for (price, size, is_bid) in &all_updates {
                                orderbook.update(*price, *size, *is_bid);
                            }
                            
                            // Update metrics
                            let latency = update_time.elapsed().as_millis() as u64;
                            if let Some(metrics) = app_guard.metrics.get_mut(&Exchange::Okx) {
                                metrics.update_latency(latency);
                                Some(metrics.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }; // Lock is released here
                    
                    // Send metrics update if available
                    if let Some(metrics) = metrics_clone {
                        tx.send(UiMessage::Metrics(Exchange::Okx, metrics)).await
                            .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                    }
                    
                    // Notify UI of orderbook update
                    tx.send(UiMessage::OrderBookUpdate(Exchange::Okx)).await
                        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                },
                Ok(_) => {},
                Err(e) => {
                    tx.send(UiMessage::Error(format!("OKX error: {}", e))).await
                        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                    return Err(Box::<dyn Error + Send + Sync>::from(e.to_string()));
                }
            }
        }
    }
    
    Ok(())
}

async fn run_bybit_spot(
    app: &Arc<Mutex<App>>, 
    symbol: &str,
    tx: mpsc::Sender<UiMessage>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let rest_client = BybitSpotPublicRest::new();
    let mut ws = BybitSpotPublicWebSocket::new();
    
    // Connect to WebSocket
    ws.connect().await.map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Subscribe to depth stream
    ws.subscribe(vec![format!("orderbook.50.{}", symbol)]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Get initial snapshot
    let snapshot = rest_client.get_orderbook_snapshot(symbol, Some(50)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Process snapshot data outside of the mutex lock
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
    
    // Initialize orderbook with snapshot - minimize time holding the lock
    {
        let mut app_guard = app.lock();
        if let Some(orderbook) = app_guard.orderbooks.get_mut(&Exchange::BybitSpot) {
            orderbook.apply_snapshot(bids, asks);
        }
    } // Lock is released here
    
    // Notify UI after releasing the lock
    tx.send(UiMessage::OrderBookUpdate(Exchange::BybitSpot)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    let mut message_stream = ws.message_stream();
    let mut reconnect_count = 0;
    
    // Check if we should continue running
    while {
        let running = app.lock().running;
        running
    } {
        if let Some(msg) = message_stream.next().await {
            let update_time = Instant::now();
            
            match msg {
                Ok(BybitSpotMessage::OrderBook(update)) => {
                    // Process update data outside of the mutex lock
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
                    
                    // Apply updates with minimal lock time
                    let metrics_clone = {
                        let mut app_guard = app.lock();
                        if let Some(orderbook) = app_guard.orderbooks.get_mut(&Exchange::BybitSpot) {
                            // Apply all updates at once
                            for (price, size, is_bid) in bids.iter().chain(asks.iter()) {
                                orderbook.update(*price, *size, *is_bid);
                            }
                            
                            // Update metrics
                            let latency = update_time.elapsed().as_millis() as u64;
                            if let Some(metrics) = app_guard.metrics.get_mut(&Exchange::BybitSpot) {
                                metrics.update_latency(latency);
                                metrics.reconnects = reconnect_count;
                                Some(metrics.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }; // Lock is released here
                    
                    // Send metrics update if available
                    if let Some(metrics) = metrics_clone {
                        tx.send(UiMessage::Metrics(Exchange::BybitSpot, metrics)).await
                            .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                    }
                    
                    // Notify UI of orderbook update
                    tx.send(UiMessage::OrderBookUpdate(Exchange::BybitSpot)).await
                        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                },
                Ok(_) => {
                    // Ignore other message types (ping, subscription responses, etc.)
                },
                Err(e) => {
                    tx.send(UiMessage::Error(format!("Bybit Spot error: {}", e))).await
                        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                    
                    // Try to reconnect
                    reconnect_count += 1;
                    
                    // Disconnect and reconnect
                    let _ = ws.disconnect().await;
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    
                    match ws.connect().await {
                        Ok(_) => {
                            // Resubscribe
                            match ws.subscribe(vec![format!("orderbook.50.{}", symbol)]).await {
                                Ok(_) => {
                                    // Get a new message stream
                                    message_stream = ws.message_stream();
                                    continue;
                                },
                                Err(e) => {
                                    return Err(Box::<dyn Error + Send + Sync>::from(format!("Failed to resubscribe: {}", e)));
                                }
                            }
                        },
                        Err(e) => {
                            return Err(Box::<dyn Error + Send + Sync>::from(format!("Failed to reconnect: {}", e)));
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

async fn run_bybit_perp(
    app: &Arc<Mutex<App>>, 
    symbol: &str,
    tx: mpsc::Sender<UiMessage>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let rest_client = BybitPerpPublicRest::new();
    let mut ws = BybitPerpPublicWebSocket::new();
    
    // Connect to WebSocket
    ws.connect().await.map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Subscribe to depth stream
    ws.subscribe(vec![format!("orderbook.50.{}", symbol)]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Get initial snapshot
    let snapshot = rest_client.get_orderbook_snapshot(symbol, Some(50)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Process snapshot data outside of the mutex lock
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
    
    // Initialize orderbook with snapshot - minimize time holding the lock
    {
        let mut app_guard = app.lock();
        if let Some(orderbook) = app_guard.orderbooks.get_mut(&Exchange::BybitPerp) {
            orderbook.apply_snapshot(bids, asks);
        }
    } // Lock is released here
    
    // Notify UI after releasing the lock
    tx.send(UiMessage::OrderBookUpdate(Exchange::BybitPerp)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    let mut message_stream = ws.message_stream();
    let mut reconnect_count = 0;
    
    // Check if we should continue running
    while {
        let running = app.lock().running;
        running
    } {
        if let Some(msg) = message_stream.next().await {
            let update_time = Instant::now();
            
            match msg {
                Ok(BybitPerpMessage::OrderBook(update)) => {
                    // Process update data outside of the mutex lock
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
                    
                    // Apply updates with minimal lock time
                    let metrics_clone = {
                        let mut app_guard = app.lock();
                        if let Some(orderbook) = app_guard.orderbooks.get_mut(&Exchange::BybitPerp) {
                            // Apply all updates at once
                            for (price, size, is_bid) in bids.iter().chain(asks.iter()) {
                                orderbook.update(*price, *size, *is_bid);
                            }
                            
                            // Update metrics
                            let latency = update_time.elapsed().as_millis() as u64;
                            if let Some(metrics) = app_guard.metrics.get_mut(&Exchange::BybitPerp) {
                                metrics.update_latency(latency);
                                metrics.reconnects = reconnect_count;
                                Some(metrics.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }; // Lock is released here
                    
                    // Send metrics update if available
                    if let Some(metrics) = metrics_clone {
                        tx.send(UiMessage::Metrics(Exchange::BybitPerp, metrics)).await
                            .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                    }
                    
                    // Notify UI of orderbook update
                    tx.send(UiMessage::OrderBookUpdate(Exchange::BybitPerp)).await
                        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                },
                Ok(_) => {
                    // Ignore other message types (ping, subscription responses, etc.)
                },
                Err(e) => {
                    tx.send(UiMessage::Error(format!("Bybit Perp error: {}", e))).await
                        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
                    
                    // Try to reconnect
                    reconnect_count += 1;
                    
                    // Disconnect and reconnect
                    let _ = ws.disconnect().await;
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    
                    match ws.connect().await {
                        Ok(_) => {
                            // Resubscribe
                            match ws.subscribe(vec![format!("orderbook.50.{}", symbol)]).await {
                                Ok(_) => {
                                    // Get a new message stream
                                    message_stream = ws.message_stream();
                                    continue;
                                },
                                Err(e) => {
                                    return Err(Box::<dyn Error + Send + Sync>::from(format!("Failed to resubscribe: {}", e)));
                                }
                            }
                        },
                        Err(e) => {
                            return Err(Box::<dyn Error + Send + Sync>::from(format!("Failed to reconnect: {}", e)));
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn run_ui<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: Arc<Mutex<App>>,
    mut rx: mpsc::Receiver<UiMessage>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut last_redraw = Instant::now();

    loop {
        // Process UI messages
        while let Ok(message) = rx.try_recv() {
            match message {
                UiMessage::OrderBookUpdate(_) => {
                    // No need to do anything, we'll redraw on the next tick
                },
                UiMessage::Metrics(exchange, metrics) => {
                    let mut app_guard = app.lock();
                    if let Some(m) = app_guard.metrics.get_mut(&exchange) {
                        *m = metrics;
                    }
                },
                UiMessage::Error(err) => {
                    eprintln!("Error: {}", err);
                },
            }
        }

        // Draw UI at the specified refresh rate
        if last_redraw.elapsed() >= REFRESH_RATE {
            terminal.draw(|f| ui(f, &app))?;
            last_redraw = Instant::now();
        }

        // Check for key events
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                let mut app_guard = app.lock();
                match key.code {
                    KeyCode::Char('q') => {
                        app_guard.running = false;
                        return Ok(());
                    },
                    KeyCode::Right | KeyCode::Tab => {
                        app_guard.next_tab();
                    },
                    KeyCode::Left | KeyCode::BackTab => {
                        app_guard.previous_tab();
                    },
                    _ => {},
                }
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame<'_>, app: &Arc<Mutex<App>>) {
    let app_guard = app.lock();
    
    // Create layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Tabs
            Constraint::Min(10),    // Orderbook
            Constraint::Length(7),  // Metrics
        ].as_ref())
        .split(f.size());

    // Create tabs
    let tab_titles: Vec<Line> = app_guard.exchanges.iter()
        .map(|e| Line::from(e.name()))
        .collect();
    
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Exchanges"))
        .select(app_guard.current_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    
    f.render_widget(tabs, chunks[0]);

    // Get current exchange
    let current_exchange = app_guard.current_exchange();
    let orderbook = app_guard.orderbooks.get(&current_exchange).unwrap();
    
    // Orderbook
    let (bids, asks) = orderbook.get_depth_with_prices(DEPTH);
    
    // Reverse asks to show highest ask at the top
    let mut asks_reversed = Vec::new();
    for (price, level) in asks.iter() {
        asks_reversed.push((*price, level.clone()));
    }
    asks_reversed.reverse();
    
    // Create table rows for asks
    let ask_rows: Vec<Row> = asks_reversed.iter()
        .map(|(price, level)| {
            Row::new(vec![
                Cell::from(format!("{:.8}", price))
                    .style(Style::default().fg(Color::Red)),
                Cell::from(format!("{:.8}", level.size)),
            ])
        })
        .collect();
    
    // Create table rows for bids
    let bid_rows: Vec<Row> = bids.iter()
        .map(|(price, level)| {
            Row::new(vec![
                Cell::from(format!("{:.8}", price))
                    .style(Style::default().fg(Color::Green)),
                Cell::from(format!("{:.8}", level.size)),
            ])
        })
        .collect();
    
    // Calculate spread
    let spread_text = if let (Some((bid_price, _)), Some((ask_price, _))) = (bids.first(), asks.first()) {
        format!("Spread: {:.8} ({:.4}%)", 
            ask_price - bid_price, 
            (ask_price - bid_price) * 100.0 / bid_price)
    } else {
        "Spread: N/A".to_string()
    };
    
    // Create spread row
    let spread_row = Row::new(vec![
        Cell::from(spread_text)
            .style(Style::default().fg(Color::Yellow)),
        Cell::from(""),
    ]);
    
    // Combine all rows
    let mut all_rows = ask_rows;
    all_rows.push(spread_row);
    all_rows.extend(bid_rows);
    
    // Create orderbook table
    let orderbook_table = Table::new(all_rows, vec![
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .header(Row::new(vec![
        Cell::from("Price").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Size").style(Style::default().add_modifier(Modifier::BOLD)),
    ]))
    .block(Block::default().borders(Borders::ALL).title(format!("{} Order Book", current_exchange.name())))
    .column_spacing(1);
    
    f.render_widget(orderbook_table, chunks[1]);

    // Metrics
    if let Some(metrics) = app_guard.metrics.get(&current_exchange) {
        let metrics_lines = vec![
            Line::from(vec![
                Span::raw("Updates Processed: "),
                Span::styled(
                    format!("{}", metrics.updates_processed),
                    Style::default().fg(Color::Cyan)
                ),
            ]),
            Line::from(vec![
                Span::raw("Reconnects: "),
                Span::styled(
                    format!("{}", metrics.reconnects),
                    Style::default().fg(Color::Cyan)
                ),
            ]),
            Line::from(vec![
                Span::raw("Last Update Latency: "),
                Span::styled(
                    format!("{}ms", metrics.last_update_latency_ms),
                    Style::default().fg(Color::Cyan)
                ),
            ]),
            Line::from(vec![
                Span::raw("Avg Update Latency: "),
                Span::styled(
                    format!("{:.2}ms", metrics.avg_update_latency_ms),
                    Style::default().fg(Color::Cyan)
                ),
            ]),
            Line::from(vec![
                Span::raw("Max Update Latency: "),
                Span::styled(
                    format!("{}ms", metrics.max_update_latency_ms),
                    Style::default().fg(Color::Cyan)
                ),
            ]),
        ];
        
        let metrics_text = Text::from(metrics_lines);
        
        let metrics_widget = Paragraph::new(metrics_text)
            .block(Block::default().borders(Borders::ALL).title("Metrics"));
        
        f.render_widget(metrics_widget, chunks[2]);
    }
} 