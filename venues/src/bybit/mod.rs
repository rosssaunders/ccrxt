pub mod enums;
mod errors;
pub mod rate_limit;

mod private {
    pub mod rest;
    pub use self::rest::RestClient as PrivateRestClient;
    pub use self::rest::{
        GetWalletBalanceRequest, GetWalletBalanceResponse, WalletBalance, BalanceData,
    };
}

// Re-export public modules
pub use enums::*;
pub use errors::{ApiError, Errors};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

// Export clients
pub use private::PrivateRestClient;
pub use private::{
    GetWalletBalanceRequest, GetWalletBalanceResponse, WalletBalance, BalanceData,
};

/// Type alias for results returned by ByBit API operations
pub type RestResult<T> = Result<T, Errors>;