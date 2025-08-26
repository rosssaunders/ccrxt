//! Shared Gate.io functionality across all product types
//!
//! This module contains common types, errors, and utilities used by all Gate.io
//! product modules (spot, delivery, perpetual, options, unified).

pub mod credentials;
pub mod enums;
pub mod errors;
pub mod private;
pub mod private_client;
pub mod public;
pub mod public_client;
pub mod rate_limit;
pub mod rate_limiter_trait;

// Re-export commonly used items
pub use credentials::Credentials;
pub use enums::{
    AccountType, CandlestickInterval, OrderSide, OrderStatus, OrderType, StpMode, TimeInForce,
};
pub use errors::{ApiError, ErrorResponse, GateIoError};
pub use private_client::*;
pub use public_client::*;
pub use rate_limit::{RateLimitHeader, RateLimitStatus, RateLimiter, UsageInfo};
pub use rate_limiter_trait::GateIoRateLimiter;

/// Result type alias for Gate.io API operations
pub type RestResult<T> = std::result::Result<T, GateIoError>;
