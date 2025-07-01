//! Spot trading endpoints for Bitget
//!
//! This module contains implementations for all Bitget spot trading endpoints
//! including order placement, cancellation, and trade information retrieval.

mod cancel_order;
mod get_current_orders;
mod get_fills;
mod get_order_history;
mod get_order_info;
mod place_order;

// Re-export all endpoint types and functions
// Removed unused pub use statements for cleaner module interface
