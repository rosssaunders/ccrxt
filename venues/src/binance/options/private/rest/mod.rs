//! Binance Options Private REST API endpoints
//!
//! This module provides REST endpoints for authenticated Binance Options API operations.

pub mod account;
pub mod client;
pub mod order;
pub mod position;
pub mod user_trades;

#[cfg(test)]
mod tests;

// Export the main client
pub use client::RestClient;

// Re-export endpoint types
pub use account::*;
pub use order::*;
pub use position::*;
pub use user_trades::*;