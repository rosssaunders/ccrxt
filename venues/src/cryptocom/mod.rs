mod credentials;
pub mod enums;
mod errors;
pub mod message;
mod private_client;
mod public_client;

pub mod rate_limit;
pub mod rate_limiter_trait;
pub mod private {
    mod rest;
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
    pub mod rest;
    pub use rest::{
        ConversionRateResponse, GetAnnouncementsRequest, GetAnnouncementsResponse, GetBookRequest,
        GetBookResponse, GetCandlestickRequest, GetCandlestickResponse, GetConversionRateRequest,
        GetExpiredSettlementPriceRequest, GetExpiredSettlementPriceResponse, GetInstrumentsRequest,
        GetInstrumentsResponse, GetInsuranceRequest, GetInsuranceResponse,
        GetRiskParametersResponse, GetTickersRequest, GetTickersResponse, GetTradesRequest,
        GetTradesResponse, GetValuationsRequest, GetValuationsResponse,
    };
}

pub use credentials::Credentials;
pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use message::ApiResult;
// Re-export the advanced order trading types
pub use private::{
    CancelOcoOrderRequest, CancelOrderListItem, CancelOrderListRequest, CancelOrderListResponse,
    CreateOcoOrderResponse, CreateOrderListRequest, CreateOrderListResponse,
    GetOrderHistoryByCurrencyRequest, GetOrderHistoryByCurrencyResponse,
    GetOrderHistoryByCurrencyWithContinuationResponse, GetOrderListRequest, GetOrderListResponse,
    OrderCancellationResult, OrderCreationResult, OrderDetails, OrderListItem,
};
pub use private_client::RestClient as PrivateRestClient;
pub use public::{
    ConversionRateResponse, GetAnnouncementsRequest, GetAnnouncementsResponse, GetBookRequest,
    GetBookResponse, GetCandlestickRequest, GetCandlestickResponse, GetConversionRateRequest,
    GetExpiredSettlementPriceRequest, GetExpiredSettlementPriceResponse, GetInstrumentsRequest,
    GetInstrumentsResponse, GetInsuranceRequest, GetInsuranceResponse, GetRiskParametersResponse,
    GetTickersRequest, GetTickersResponse, GetTradesRequest, GetTradesResponse,
    GetValuationsRequest, GetValuationsResponse,
};
pub use public_client::RestClient as PublicRestClient;
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by Crypto.com API operations
pub type RestResult<T> = Result<T, Errors>;
