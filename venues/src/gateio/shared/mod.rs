//! Shared Gate.io functionality across all product types
//!
//! This module contains common types, errors, and utilities used by all Gate.io
//! product modules (spot, delivery, perpetual, options, unified).

pub mod credentials;
pub mod enums;
pub mod errors;
pub mod rate_limit;

// Re-export commonly used items
pub use credentials::Credentials;
pub use enums::{
    AccountType, CandlestickInterval, OrderSide, OrderStatus, OrderType, StpMode, TimeInForce,
};
pub use errors::{ApiError, ErrorResponse, GateIoError, RestResult};
pub use rate_limit::{RateLimitHeader, RateLimitStatus, RateLimiter, UsageInfo};
