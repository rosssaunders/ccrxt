/// Binance Spot WebSocket implementation
/// 
/// This module provides WebSocket connectivity for Binance Spot markets.

pub mod client;
pub mod enums;
pub mod message;
pub mod rate_limit;

// WebSocket endpoint modules
pub mod ws_public_agg_trades;
pub mod ws_public_trade;
pub mod ws_public_orderbook;
pub mod ws_public_klines;
pub mod ws_public_ticker;
pub mod ws_public_all_tickers;
pub mod ws_public_mini_ticker;
pub mod ws_public_all_mini_tickers;
pub mod ws_public_book_ticker;
pub mod ws_public_all_book_tickers;
pub mod ws_public_avg_price;
pub mod ws_public_rolling_window_ticker;

// Re-export key types
pub use client::BinanceSpotWebSocketClient;
pub use enums::{DepthLevel, KlineInterval, StreamType, UpdateSpeed};
pub use message::{BinanceMessage, BinanceRequest, BinanceResponse, ErrorMessage};
pub use rate_limit::{RateLimitError, RateLimitStats, WebSocketRateLimiter};

// Re-export data types from their respective modules
pub use ws_public_agg_trades::AggTradeData;
pub use ws_public_trade::TradeData;
pub use ws_public_klines::{Kline, KlineData};
pub use ws_public_orderbook::{DepthUpdateData, PartialDepthData};
pub use ws_public_ticker::Ticker24hrData;
pub use ws_public_mini_ticker::MiniTicker24hrData;
pub use ws_public_book_ticker::BookTickerData;
pub use ws_public_avg_price::AvgPriceData;
pub use ws_public_rolling_window_ticker::{RollingWindowTickerData, RollingWindowSize};