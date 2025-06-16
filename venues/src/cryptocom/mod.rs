pub mod enums;
mod errors;
mod examples;
mod integration_tests;

pub mod rate_limit;
pub mod private {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{
        CancelOcoOrderRequest, CancelOrderListItem, CancelOrderListRequest, CancelOrderListResponse, CreateOcoOrderResponse, CreateOrderListRequest,
        CreateOrderListResponse, GetOrderListRequest, GetOrderListResponse, OrderCancellationResult, OrderCreationResult, OrderDetails, OrderListItem,
    };
}

pub mod public {
    mod rest;
    pub use self::rest::RestClient;
}

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use private::RestClient as PrivateRestClient;
pub use public::RestClient as PublicRestClient;
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};
// Re-export the advanced order trading types
pub use private::{
    CancelOcoOrderRequest, CancelOrderListItem, CancelOrderListRequest, CancelOrderListResponse, CreateOcoOrderResponse, CreateOrderListRequest,
    CreateOrderListResponse, GetOrderListRequest, GetOrderListResponse, OrderCancellationResult, OrderCreationResult, OrderDetails, OrderListItem,
};

/// Type alias for results returned by Crypto.com API operations
pub type RestResult<T> = Result<T, Errors>;
