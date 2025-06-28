pub mod enums;
mod errors;

pub mod rate_limit;
pub mod private {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{
        CancelOcoOrderRequest, CancelOrderListItem, CancelOrderListRequest,
        CancelOrderListResponse, CreateOcoOrderResponse, CreateOrderListRequest,
        CreateOrderListResponse, GetOrderHistoryByCurrencyRequest,
        GetOrderHistoryByCurrencyResponse, GetOrderHistoryByCurrencyWithContinuationResponse,
        GetOrderListRequest, GetOrderListResponse, OrderCancellationResult, OrderCreationResult,
        OrderDetails, OrderListItem,
    };
}

pub mod public {
    mod rest;
    pub use self::rest::RestClient;
}

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use private::RestClient as PrivateRestClient;
// Re-export the advanced order trading types
pub use private::{
    CancelOcoOrderRequest, CancelOrderListItem, CancelOrderListRequest, CancelOrderListResponse,
    CreateOcoOrderResponse, CreateOrderListRequest, CreateOrderListResponse,
    GetOrderHistoryByCurrencyRequest, GetOrderHistoryByCurrencyResponse,
    GetOrderHistoryByCurrencyWithContinuationResponse, GetOrderListRequest, GetOrderListResponse,
    OrderCancellationResult, OrderCreationResult, OrderDetails, OrderListItem,
};
pub use public::RestClient as PublicRestClient;
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by Crypto.com API operations
pub type RestResult<T> = Result<T, Errors>;
