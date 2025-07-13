//! Public REST endpoints module for Binance Spot API
//!
//! This module provides access to all public endpoints for the Binance Spot API,
//! including market data, trading information, and exchange details.

pub mod client;

// Core endpoints
pub mod exchange_info;
pub mod ping;
pub mod server_time;

// Market data endpoints
pub mod agg_trades;
pub mod avg_price;
pub mod depth;
pub mod historical_trades;
pub mod klines;
pub mod trades;
pub mod ui_klines;

// Ticker endpoints
pub mod ticker;
pub mod ticker_24hr;
pub mod ticker_book;
pub mod ticker_price;
pub mod ticker_trading_day;

// Re-export all request types for integration tests
pub use agg_trades::AggTradesRequest;
pub use avg_price::AvgPriceRequest;
pub use client::RestClient;
pub use depth::DepthRequest;
pub use exchange_info::ExchangeInfoRequest;
pub use historical_trades::HistoricalTradesRequest;
pub use klines::KlinesRequest;
pub use ticker::TickerRequest;
pub use ticker_24hr::Ticker24hrRequest;
pub use ticker_book::TickerBookRequest;
pub use ticker_price::TickerPriceRequest;
pub use ticker_trading_day::TickerTradingDayRequest;
pub use trades::TradesRequest;
pub use ui_klines::UiKlinesRequest;
