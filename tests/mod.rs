//! Integration tests for all cryptocurrency exchange venues
//!
//! This module contains integration tests for all supported cryptocurrency exchanges.
//! Tests are organized by venue to maintain clarity and allow for targeted testing.
//!
//! ## Running Tests
//!
//! Run all integration tests:
//! ```bash
//! cargo test
//! ```
//!
//! Run tests for a specific venue:
//! ```bash
//! cargo test binance
//! cargo test deribit
//! # etc.
//! ```
//!
//! Run a specific test:
//! ```bash
//! cargo test binance::coinm::public_integration_tests::test_get_exchange_info
//! ```

// Venue-specific test modules
pub mod binance;
pub mod bingx;
pub mod bitget;
pub mod bitmart;
pub mod bullish;
pub mod bybit;
pub mod coinbaseexchange;
pub mod cryptocom;
pub mod deribit;
pub mod gateio;
pub mod kucoin;
pub mod okx;
