use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::StreamExt;
use orderbook::{OrderBook, Level};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use venues::binance::{
    spot::{BinanceSpotPublicWebSocket, BinanceSpotPublicRest, WebSocketMessage},
};
use venues::websockets::WebSocketConnection;

const PRICE_PRECISION: u32 = 8;
const MAX_RECONNECT_ATTEMPTS: u32 = 5;
const INITIAL_BACKOFF_MS: u64 = 1000;
const MAX_BACKOFF_MS: u64 = 30000;
const REFRESH_RATE: Duration = Duration::from_millis(100);

// Performance metrics for the venue
#[derive(Default)]
struct VenueMetrics {
    updates_processed: u64,
    reconnects: u64,
    last_update_latency_ms: u64,
    avg_update_latency_ms: f64,
    max_update_latency_ms: u64,
}

impl VenueMetrics {
    fn update_latency(&mut self, latency_ms: u64) {
        self.last_update_latency_ms = latency_ms;
        self.avg_update_latency_ms = (self.avg_update_latency_ms * self.updates_processed as f64 + latency_ms as f64) / 
                                   (self.updates_processed as f64 + 1.0);
        self.max_update_latency_ms = self.max_update_latency_ms.max(latency_ms);
        self.updates_processed += 1;
    }
}

// App state
struct App {
    orderbook: OrderBook,
    metrics: VenueMetrics,
    symbol: String,
    depth: usize,
    last_update: Instant,
}

impl App {
    fn new(symbol: &str, depth: usize) -> Self {
        Self {
            orderbook: OrderBook::new(PRICE_PRECISION),
            metrics: VenueMetrics::default(),
            symbol: symbol.to_string(),
            depth,
            last_update: Instant::now(),
        }
    }
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
    let depth = 10;
    let app = App::new(symbol, depth);
    let res = run_app(&mut terminal, app).await;

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

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let rest_client = BinanceSpotPublicRest::new();
    let mut ws = BinanceSpotPublicWebSocket::new();
    
    // Connect to WebSocket
    ws.connect().await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Subscribe to depth stream
    ws.subscribe(vec![format!("{}@depth@100ms", app.symbol.to_lowercase())]).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Get initial snapshot
    let snapshot = rest_client.get_orderbook_snapshot(&app.symbol, Some(1000)).await
        .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
    
    // Initialize orderbook with snapshot
    app.orderbook.apply_snapshot(
        snapshot.bids.iter()
            .filter_map(|level| {
                let price = level.0.parse::<f64>().ok()?;
                let size = level.1.parse::<f64>().ok()?;
                Some((price, size))
            })
            .collect(),
        snapshot.asks.iter()
            .filter_map(|level| {
                let price = level.0.parse::<f64>().ok()?;
                let size = level.1.parse::<f64>().ok()?;
                Some((price, size))
            })
            .collect(),
    );
    
    let mut message_stream = ws.message_stream();
    let mut last_redraw = Instant::now();

    loop {
        // Poll for WebSocket messages (non-blocking)
        if let Ok(Some(msg)) = tokio::time::timeout(Duration::from_millis(1), message_stream.next()).await {
            let update_time = Instant::now();
            
            match msg {
                Ok(WebSocketMessage::OrderBook(update)) => {
                    // Process bids
                    for bid in &update.bids {
                        if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                            app.orderbook.update(price, size, true);
                        }
                    }
                    
                    // Process asks
                    for ask in &update.asks {
                        if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                            app.orderbook.update(price, size, false);
                        }
                    }
                    
                    // Update metrics
                    let latency = update_time.elapsed().as_millis() as u64;
                    app.metrics.update_latency(latency);
                    app.last_update = Instant::now();
                },
                Ok(_) => {},
                Err(e) => {
                    return Err(Box::<dyn Error + Send + Sync>::from(e.to_string()));
                }
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
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame<'_>, app: &App) {
    // Create layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Orderbook
            Constraint::Length(7),  // Metrics
        ].as_ref())
        .split(f.size());

    // Header
    let header_spans = vec![
        Span::styled(
            format!(" {} Orderbook", app.symbol),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        ),
        Span::raw(" | "),
        Span::styled(
            format!("Press 'q' to quit"),
            Style::default().fg(Color::Gray)
        ),
    ];
    
    let header_text = Text::from(Line::from(header_spans));
    
    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).title("Binance Spot"));
    f.render_widget(header, chunks[0]);

    // Orderbook
    let (bids, asks) = app.orderbook.get_depth_with_prices(app.depth);
    
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
    .block(Block::default().borders(Borders::ALL).title("Order Book"))
    .column_spacing(1);
    
    f.render_widget(orderbook_table, chunks[1]);

    // Metrics
    let metrics_lines = vec![
        Line::from(vec![
            Span::raw("Updates Processed: "),
            Span::styled(
                format!("{}", app.metrics.updates_processed),
                Style::default().fg(Color::Cyan)
            ),
        ]),
        Line::from(vec![
            Span::raw("Reconnects: "),
            Span::styled(
                format!("{}", app.metrics.reconnects),
                Style::default().fg(Color::Cyan)
            ),
        ]),
        Line::from(vec![
            Span::raw("Last Update Latency: "),
            Span::styled(
                format!("{}ms", app.metrics.last_update_latency_ms),
                Style::default().fg(Color::Cyan)
            ),
        ]),
        Line::from(vec![
            Span::raw("Avg Update Latency: "),
            Span::styled(
                format!("{:.2}ms", app.metrics.avg_update_latency_ms),
                Style::default().fg(Color::Cyan)
            ),
        ]),
        Line::from(vec![
            Span::raw("Max Update Latency: "),
            Span::styled(
                format!("{}ms", app.metrics.max_update_latency_ms),
                Style::default().fg(Color::Cyan)
            ),
        ]),
    ];
    
    let metrics_text = Text::from(metrics_lines);
    
    let metrics = Paragraph::new(metrics_text)
        .block(Block::default().borders(Borders::ALL).title("Metrics"));
    
    f.render_widget(metrics, chunks[2]);
} 