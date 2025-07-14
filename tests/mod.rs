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
//!
//! ## Private Endpoint Testing
//!
//! Private endpoint tests require API credentials and are disabled by default.
//! To enable private tests, set environment variables:
//!
//! ```bash
//! export RUN_PRIVATE_TESTS=true
//! export BINANCE_API_KEY=your_api_key
//! export BINANCE_SECRET_KEY=your_secret_key
//! # ... other venue credentials
//! cargo test
//! ```

// Common testing utilities
pub mod common;

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
