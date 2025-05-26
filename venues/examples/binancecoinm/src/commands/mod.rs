pub mod account;
pub mod trades;
pub mod batch_order;

pub use account::handle_account_command;
pub use trades::handle_trades_command;
pub use batch_order::handle_batch_order_command;

use clap::Subcommand;

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
} 