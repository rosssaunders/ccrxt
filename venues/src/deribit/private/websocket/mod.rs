//! Deribit private websocket endpoint implementations
//!
//! This module provides websocket-specific functionality for Deribit private endpoints,
//! including the unsubscribe_all endpoint that unsubscribes from all active channels.

pub mod unsubscribe_all;

pub use unsubscribe_all::*;