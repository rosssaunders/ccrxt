use clap::Parser;
use futures::StreamExt;
use tracing::{info, Level};
use venues::coinbase::advanced_trade::websocket::CoinbaseAdvancedTradeWebSocket;
use websockets::{BoxResult, WebSocketConnection};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Comma-separated list of products to subscribe to (e.g., "BTC-USD,ETH-USD,SOL-USD")
    #[arg(short, long, default_value = "BTC-USD,ETH-USD,SOL-USD")]
    products: String,

    /// Subscribe to orderbook updates
    #[arg(short = 'b', long, default_value_t = false)]
    orderbook: bool,

    /// Subscribe to ticker updates
    #[arg(short = 'i', long, default_value_t = false)]
    ticker: bool,

    /// Subscribe to market trades
    #[arg(short = 't', long, default_value_t = false)]
    trades: bool,

    /// Subscribe to heartbeats
    #[arg(short = 'r', long, default_value_t = false)]
    heartbeats: bool,
}

#[tokio::main]
async fn main() -> BoxResult<()> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Parse command line arguments
    let args = Args::parse();

    // Convert comma-separated products string to Vec<String>
    let products: Vec<String> = args
        .products
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    info!("Starting Coinbase Advanced Trade market data stream...");
    info!("Products: {:?}", products);

    // Create a new websocket instance
    let mut ws = CoinbaseAdvancedTradeWebSocket::new();

    // Connect to the websocket
    ws.connect().await?;
    info!("Connected to Coinbase Advanced Trade WebSocket");

    // Subscribe to requested channels
    if args.orderbook {
        ws.subscribe_orderbook(products.clone()).await?;
        info!("Subscribed to orderbook updates");
    }
    if args.ticker {
        ws.subscribe_ticker(products.clone()).await?;
        info!("Subscribed to ticker updates");
    }
    if args.trades {
        ws.subscribe_market_trades(products.clone()).await?;
        info!("Subscribed to market trades");
    }
    if args.heartbeats {
        ws.subscribe_heartbeats().await?;
        info!("Subscribed to heartbeats");
    }

    info!("Processing market data...");

    // Get the message stream
    let mut stream = ws.message_stream();

    // Process incoming messages
    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => {
                info!("Received: {:?}", msg);
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }

    Ok(())
}
