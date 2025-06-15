//! Binance Options Private API module
//!
//! This module provides private (authenticated) REST API functionality for Binance Options.
//! All endpoints in this module require valid API credentials and proper authentication.

pub mod rest;

pub use rest::RestClient;