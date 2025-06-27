// Public REST endpoints module for Binance Coin-M

pub mod aggregate_trades;
pub mod basis;
pub mod book_ticker;
pub mod client;
pub mod constituents;
pub mod continuous_klines;
pub mod exchange_info;
pub mod funding_info;
pub mod funding_rate;
pub mod global_long_short_account_ratio;
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

pub use client::RestClient;
