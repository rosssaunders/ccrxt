//! Deribit private API implementations
//!
//! This module contains implementations for Deribit's private API endpoints,
//! including websocket connections that require authentication.

pub mod websocket;

pub use websocket::*;