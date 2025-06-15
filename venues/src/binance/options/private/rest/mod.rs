//! Binance Options Private REST API endpoints
//!
//! This module provides REST endpoints for authenticated Binance Options API operations.

pub mod account;
pub mod cancel_order;
pub mod client;
pub mod order;
pub mod position;
pub mod user_trades;

// Export the main client
pub use client::RestClient;