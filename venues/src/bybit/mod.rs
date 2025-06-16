pub mod enums;
pub mod errors;
pub mod rate_limit;

pub mod private {
    mod rest;

    pub use self::rest::RestClient as PrivateRestClient;
    pub use self::rest::{BalanceData, GetWalletBalanceRequest, GetWalletBalanceResponse, WalletBalance};
}

// Re-export public modules
pub use enums::*;
pub use errors::{ApiError, Errors};
// Export clients
pub use private::PrivateRestClient;
pub use private::{BalanceData, GetWalletBalanceRequest, GetWalletBalanceResponse, WalletBalance};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by ByBit API operations
pub type RestResult<T> = Result<T, Errors>;
