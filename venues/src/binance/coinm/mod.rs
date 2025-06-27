use std::time::Duration;

mod enums;
mod errors;
mod rate_limit;
mod request;

// Re-export modules for new structure
mod public {
    mod rest;
    pub use self::rest::RestClient as PublicRestClient;
    pub use self::rest::exchange_info::*;
}

mod private {
    mod rest;
    // Re-export RestClient so it can be re-exported by the parent
    pub use self::rest::RestClient as PrivateRestClient;
    pub use self::rest::account::*;
    pub use self::rest::account_trades::*;
    pub use self::rest::all_orders::*;
    pub use self::rest::asset_index::*;
    pub use self::rest::auto_cancel_all_open_orders::*;
    pub use self::rest::batch_order::*;
    pub use self::rest::cancel_all_open_orders::*;
    pub use self::rest::cancel_order::*;
    pub use self::rest::change_initial_leverage::*;
    pub use self::rest::change_margin_type::*;
    pub use self::rest::change_position_mode::*;
    pub use self::rest::download_transaction_history::*;
    pub use self::rest::force_orders::*;
    pub use self::rest::futures_account_balance::*;
    pub use self::rest::get_current_position_mode::*;
    pub use self::rest::income_history::*;
    pub use self::rest::listen_key::*;
    pub use self::rest::modify_isolated_position_margin::*;
    pub use self::rest::modify_multiple_orders::*;
    pub use self::rest::modify_order::*;
    pub use self::rest::multi_asset_mode::*;
    pub use self::rest::notional_brackets::*;
    pub use self::rest::open_orders::*;
    pub use self::rest::order::*;
    pub use self::rest::order_modify_history::*;
    pub use self::rest::position_adl_quantile::*;
    pub use self::rest::position_margin_change_history::*;
    pub use self::rest::position_risk::*;
    pub use self::rest::query_current_open_order::*;
    pub use self::rest::query_order::*;
    pub use self::rest::trading_status::*;
    pub use self::rest::universal_transfer::*;
    pub use self::rest::universal_transfer_history::*;
    pub use self::rest::user_commission_rate::*;
}

// Only expose RestClient at the coinm level, not via private::rest
pub use enums::*;
pub use errors::{ApiError, Errors};
pub use private::*;
pub use public::*;
pub use rate_limit::{RateLimitHeader, RateLimiter};

pub use crate::binance::coinm::errors::ErrorResponse;
pub(crate) use crate::binance::coinm::request::execute_request;

/// Represents the relevant response headers returned by the Binance API for rate limiting and order tracking.
///
/// Each field corresponds to a specific header returned by the API, such as used weights or order counts for various intervals.
/// This structure is now strongly typed for high performance and correctness.
#[derive(Debug, Clone, Default)]
pub struct ResponseHeaders {
    /// Map of parsed rate limit/order count headers to their integer values.
    ///
    /// For example:
    /// - RateLimitHeader { kind: UsedWeight, interval_value: 1, interval_unit: Minute } => 123
    /// - RateLimitHeader { kind: OrderCount, interval_value: 1, interval_unit: Day } => 10
    ///
    /// This map is keyed by strongly-typed header descriptors for maximum performance and correctness.
    pub values: std::collections::HashMap<RateLimitHeader, u32>,
}

#[derive(Debug, Clone)]
pub struct RestResponse<T> {
    pub data: T,
    pub request_duration: Duration,
    pub headers: ResponseHeaders,
}

/// Type alias for results returned by Binance API operations
pub type RestResult<T> = Result<RestResponse<T>, Errors>;

pub mod rest {
    pub mod common;
}
