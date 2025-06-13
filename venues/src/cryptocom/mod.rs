mod errors;
mod integration_tests;
mod examples;
pub mod enums;

pub mod rate_limit;
pub mod private {
    mod rest;
    pub use self::rest::RestClient as RestClient;
    pub use self::rest::{
        CreateOrderListRequest, OrderListItem, CreateOrderListResponse, CreateOcoOrderResponse, OrderCreationResult,
        CancelOrderListRequest, CancelOcoOrderRequest, CancelOrderListItem, CancelOrderListResponse, OrderCancellationResult,
        GetOrderListRequest, GetOrderListResponse, OrderDetails
    };
}

pub mod public {
    mod rest;
    pub use self::rest::RestClient as RestClient;
}

pub use rate_limit::{RateLimiter, EndpointType, RateLimitError, RateLimit};
pub use errors::{Errors, ApiError, ErrorResponse};
pub use enums::*;
pub use private::RestClient as PrivateRestClient;
pub use public::RestClient as PublicRestClient;
// Re-export the advanced order trading types
pub use private::{
    CreateOrderListRequest, OrderListItem, CreateOrderListResponse, CreateOcoOrderResponse, OrderCreationResult,
    CancelOrderListRequest, CancelOcoOrderRequest, CancelOrderListItem, CancelOrderListResponse, OrderCancellationResult,
    GetOrderListRequest, GetOrderListResponse, OrderDetails
};

/// Type alias for results returned by Crypto.com API operations
pub type RestResult<T> = Result<T, Errors>;
