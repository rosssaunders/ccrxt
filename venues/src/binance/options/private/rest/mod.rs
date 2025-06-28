//! Private REST endpoints module for Binance Options API
//!
//! This module provides access to all private endpoints for the Binance Options API,
//! including account information, order management, position queries, market maker
//! functionality, and block trading.

pub mod client;

// Account endpoints
pub mod account;
pub mod account_funding_flow;
pub mod income_download;
pub mod margin_account;

// Trading endpoints
pub mod batch_cancel;
pub mod batch_orders;
pub mod cancel_order;
pub mod exercise_record;
pub mod history_orders;
pub mod open_orders;
pub mod order;
pub mod position;
pub mod query_order;
pub mod user_trades;

// Market maker endpoints
pub mod countdown_cancel;
pub mod mmp_config;

// Block trade endpoints
pub mod block_trade;

pub use client::RestClient;
