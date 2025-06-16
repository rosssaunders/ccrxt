pub mod account;
pub mod all_orders;
pub mod batch_order;
pub mod exchange_info;
pub mod order;
pub mod position_risk;
pub mod trades;

pub use account::handle_account_command;
pub use batch_order::handle_batch_order_command;
use clap::Subcommand;
pub use exchange_info::handle_exchange_info_command;
pub use order::handle_order_command;
pub use trades::handle_trades_command;

#[derive(Subcommand)]
pub enum Commands {
    /// Get account information including balances and positions
    Account,

    /// Get recent trades for a symbol
    Trades {
        /// Trading symbol (e.g., BTCUSD_PERP)
        #[arg(short, long)]
        symbol: String,

        /// Maximum number of trades to fetch
        #[arg(short, long, default_value = "100")]
        limit: u32,
    },

    /// Place a batch of orders
    BatchOrder {
        /// Trading symbol (e.g., BTCUSD_PERP)
        #[arg(short, long)]
        symbol: String,

        /// Order side (BUY or SELL)
        #[arg(short, long, value_parser = ["BUY", "SELL"])]
        side: String,

        /// Order type (LIMIT or MARKET)
        #[arg(short, long, value_parser = ["LIMIT", "MARKET"])]
        order_type: String,

        /// Order quantity
        #[arg(short, long)]
        quantity: String,

        /// Order price (required for LIMIT orders)
        #[arg(short, long)]
        price: Option<String>,

        /// Client order ID
        #[arg(long)]
        client_order_id: Option<String>,
    },

    /// Get exchange information (trading rules, symbols, rate limits)
    ExchangeInfo,

    /// Place a new order
    Order {
        /// Trading symbol (e.g., BTCUSD_200925)
        #[arg(short, long)]
        symbol: String,
        /// Order side (BUY or SELL)
        #[arg(short, long)]
        side: String,
        /// Order type (LIMIT or MARKET)
        #[arg(short, long)]
        order_type: String,
        /// Order quantity
        #[arg(short, long)]
        quantity: Option<f64>,
        /// Order price (required for LIMIT)
        #[arg(short, long)]
        price: Option<f64>,
    },

    /// Get all orders for a symbol
    AllOrders {
        /// Trading symbol (e.g., BTCUSD_200925)
        #[arg(short, long)]
        symbol: String,
        /// Limit number of orders
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },

    /// Get position risk (all positions)
    PositionRisk,
}

pub async fn handle_all_orders_command(client: std::sync::Arc<venues::binance::coinm::PrivateRestClient>, symbol: String, limit: u32) -> anyhow::Result<()> {
    crate::commands::all_orders::run_all_orders(&client, symbol, limit).await;
    Ok(())
}

pub async fn handle_position_risk_command(client: std::sync::Arc<venues::binance::coinm::PrivateRestClient>) -> anyhow::Result<()> {
    crate::commands::position_risk::run_position_risk(&client).await;
    Ok(())
}
