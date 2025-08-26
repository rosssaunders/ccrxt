//! Public REST endpoints module for Binance Options API
//!
//! This module provides access to all public endpoints for the Binance Options API,
//! including market data, trading information, and exchange details.


// Core endpoints
pub mod exchange_info;
pub mod ping;
pub mod server_time;

// Market data endpoints
pub mod historical_trades;
pub mod klines;
pub mod mark_price;
pub mod order_book;
pub mod recent_trades;
pub mod ticker;

// Option-specific endpoints
pub mod block_trades;
pub mod exercise_history;
pub mod open_interest;
pub mod symbol_price_ticker;

