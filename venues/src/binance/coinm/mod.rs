use std::time::Duration;

mod enums;
mod errors;
mod private_client;
mod public_client;
mod rate_limit;

// Re-export modules for new structure
mod public {
    mod rest;
    pub use self::rest::*;
}

mod private {
    mod rest;
    // Re-export RestClient so it can be re-exported by the parent
    pub use self::rest::{
        account::*, account_trades::*, all_orders::*, auto_cancel_all_open_orders::*,
        batch_order::*, cancel_all_open_orders::*, cancel_order::*, change_initial_leverage::*,
        change_margin_type::*, change_position_mode::*, create_listen_key::*, delete_listen_key::*,
        extend_listen_key::*, force_orders::*, futures_account_balance::*,
        get_current_position_mode::*, get_transaction_history_download_id::*,
        get_transaction_history_download_link::*, income_history::*,
        modify_isolated_position_margin::*, modify_multiple_orders::*, modify_order::*,
        notional_brackets::*, open_orders::*, order::*, order_modify_history::*,
        position_adl_quantile::*, position_margin_change_history::*, position_risk::*,
        query_current_open_order::*, query_order::*, user_commission_rate::*,
    };
}

// Only expose RestClient at the coinm level, not via private::rest
pub use enums::*;
pub use errors::{ApiError, Errors};
pub use private::*;
pub use public::*;
pub use rate_limit::{RateLimitHeader, RateLimiter};

pub use crate::binance::coinm::errors::ErrorResponse;
pub use private_client::RestClient as PrivateRestClient;
pub use public_client::RestClient as PublicRestClient;

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

/// Type alias for the CoinmClient
pub type CoinmClient = crate::binance::shared::client::PrivateBinanceClient;

use crate::binance::shared::venue_trait::{RateLimits, VenueConfig};

/// Coin-Margined Futures venue configuration
pub struct CoinmConfig;

impl VenueConfig for CoinmConfig {
    fn base_url(&self) -> &str {
        "https://dapi.binance.com"
    }
    fn rate_limits(&self) -> RateLimits {
        RateLimits {
            request_weight_limit: 6000,
            request_weight_window: Duration::from_secs(60),
            raw_requests_limit: 61000,
            raw_requests_window: Duration::from_secs(300), // 5 minutes
            orders_10s_limit: 100,
            orders_minute_limit: 1200,
            orders_day_limit: None,
        }
    }
}
