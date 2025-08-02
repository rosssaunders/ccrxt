//! Binance USD-M Futures integration tests
//!
//! This module provides comprehensive integration tests for all public endpoints
//! of the Binance USD-M Futures API. Tests cover:
//!
//! - Basic connectivity (ping, server time)
//! - Market data (exchange info, order book, recent trades, klines)
//! - Advanced market data (funding rates, open interest, premium index)
//! - Ticker data (24hr stats, price tickers, book tickers)
//! - Historical data (aggregate trades, historical trades)
//! - Ratio and analytics data (long/short ratios, taker buy/sell volume)
//! - Delivery and index data (delivery price, asset index, constituents)
//! - Basis and volatility data
//!
//! All tests run against the live Binance USD-M API and may be subject to
//! geographic restrictions or rate limits.

pub mod public_api_key_integration_tests;
pub mod public_integration_tests;
