// Public REST endpoints module for Binance Coin-M

pub mod aggregate_trades;
pub mod basis;
pub mod book_ticker;
pub mod constituents;
pub mod continuous_klines;
pub mod exchange_info;
pub mod funding_info;
pub mod funding_rate;
pub mod historical_trades;
pub mod index_price_klines;
pub mod klines;
pub mod mark_price_klines;
pub mod open_interest;
pub mod open_interest_hist;
pub mod order_book;
pub mod ping;
pub mod premium_index;
pub mod premium_index_klines;
pub mod recent_trades;
pub mod server_time;
pub mod taker_buy_sell_vol;
pub mod ticker_24hr;
pub mod ticker_price;
pub mod top_long_short_account_ratio;
pub mod top_long_short_position_ratio;

// Re-export all endpoint types for integration tests
pub use aggregate_trades::*;
pub use basis::*;
pub use book_ticker::*;
pub use constituents::*;
pub use continuous_klines::*;
pub use exchange_info::*;
pub use funding_info::*;
pub use funding_rate::*;
pub use historical_trades::*;
pub use index_price_klines::*;
pub use klines::*;
pub use mark_price_klines::*;
pub use open_interest::*;
pub use open_interest_hist::*;
pub use order_book::*;
pub use ping::*;
pub use premium_index::*;
pub use premium_index_klines::*;
pub use recent_trades::*;
pub use server_time::*;
pub use taker_buy_sell_vol::*;
pub use ticker_24hr::*;
pub use ticker_price::*;
pub use top_long_short_account_ratio::*;
pub use top_long_short_position_ratio::*;

pub use crate::binance::coinm::{PublicRestClient as RestClient, RestResult};
