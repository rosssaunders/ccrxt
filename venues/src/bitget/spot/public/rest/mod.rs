pub mod candlestick;
pub mod client;
pub mod coin_info;
pub mod history_candlestick;
pub mod market_trades;
pub mod merge_depth;
pub mod orderbook;
pub mod recent_trades;
pub mod symbol_info;
pub mod ticker;
pub mod vip_fee_rate;

// Re-export client and main types
pub use candlestick::*;
pub use client::RestClient;
pub use coin_info::*;
pub use history_candlestick::*;
pub use market_trades::*;
pub use merge_depth::*;
pub use orderbook::*;
pub use recent_trades::*;
pub use symbol_info::*;
pub use ticker::*;
pub use vip_fee_rate::*;
