//! Binance CoinM integration tests
//!
//! This module provides comprehensive integration tests for all public endpoints
//! of the Binance CoinM (Coin-Margined Futures) API. Tests cover:
//!
//! - Basic connectivity (ping, server time)
//! - Market data (exchange info, order book, trades)
//! - Kline/candlestick data (regular, continuous, index, mark, premium)
//! - Premium index and funding rate information
//! - Ticker data (24hr, price, book ticker)
//! - Open interest data and history
//! - Long/short ratio data (account, position, global)
//! - Volume and basis data
//! - Constituents and funding information
//!
//! All tests run against the live Binance CoinM API and may be subject to
//! geographic restrictions or rate limits.

pub mod public_integration_tests;
