pub mod enums;
mod errors;
mod rate_limit;
pub mod rate_limiter_trait;

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by Coinbase API operations
pub type RestResult<T> = Result<T, Errors>;

pub mod public {
    pub mod rest;
    pub use self::rest::RestClient;
}

pub use public::RestClient as PublicRestClient;
// Re-export all public REST types for integration tests
pub use public::rest::{
    AuctionInfo, Candle, GetProductBookRequest, GetProductBookResponse, GetProductCandlesRequest,
    GetProductCandlesResponse, GetProductRequest, GetProductResponse, GetProductStatsRequest,
    GetProductStatsResponse, GetProductTickerRequest, GetProductTickerResponse,
    GetProductTradesRequest, GetProductTradesResponse, GetProductVolumeSummaryRequest,
    GetProductVolumeSummaryResponse, GetProductsRequest, GetProductsResponse, OrderBookLevel,
    PaginationInfo, Product, ProductStats, ProductTicker, ProductVolumeSummary, Trade,
};

pub mod private {
    pub mod rest;
    pub use self::rest::RestClient;
}

pub use private::RestClient as PrivateRestClient;
