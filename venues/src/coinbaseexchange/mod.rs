pub mod enums;
mod errors;
mod rate_limit;
pub mod rate_limiter_trait;

// Client modules
pub mod private_client;
pub mod public_client;

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by Coinbase API operations
pub type RestResult<T> = Result<T, Errors>;

// Re-export clients
pub use private_client::RestClient as PrivateRestClient;
pub use public_client::RestClient as PublicRestClient;

// Keep the old module structure for backwards compatibility and endpoint access
pub mod public {
    pub mod rest;
}

pub mod private {
    pub mod rest;
}

// Re-export all public REST types for integration tests
pub use public::rest::{
    AuctionInfo, Candle, GetProductBookRequest, GetProductBookResponse, GetProductCandlesRequest,
    GetProductCandlesResponse, GetProductRequest, GetProductResponse, GetProductStatsRequest,
    GetProductStatsResponse, GetProductTickerRequest, GetProductTickerResponse,
    GetProductTradesRequest, GetProductTradesResponse, GetProductVolumeSummaryRequest,
    GetProductVolumeSummaryResponse, GetProductsRequest, GetProductsResponse, OrderBookLevel,
    PaginationInfo, Product, ProductStats, ProductTicker, ProductVolumeSummary, Trade,
};
