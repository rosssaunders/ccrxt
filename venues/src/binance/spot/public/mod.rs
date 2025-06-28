//! Public API module for Binance Spot
//!
//! This module provides access to public endpoints that do not require authentication.

pub mod rest;

// Re-export the RestClient
pub use rest::RestClient;