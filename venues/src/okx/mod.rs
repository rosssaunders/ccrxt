pub mod credentials;
pub mod enums;
mod errors;
pub mod private_client;
pub mod public_client;
mod response;

pub mod rate_limit;
pub mod rate_limiter_trait;

pub mod public {
    mod rest;
    pub use self::rest::*;
}

pub mod private {
    mod rest;
    pub use self::rest::{
        AccountBalance, AccountConfig, AmendOrderRequest, AmendOrderResponse, ApiResponse,
        AttachedAlgoOrder, BalanceDetail, CancelBatchOrdersResponse, CancelOrderRequest,
        CancelOrderResponse, CloseOrderAlgo, ClosePositionRequest, ClosePositionResponse,
        Counterparty, EconomicCalendarEvent, Fill, GetAccountBalanceRequest,
        GetAccountConfigRequest, GetEconomicCalendarRequest, GetFillsRequest,
        GetOrderHistoryRequest, GetOrderRequest, GetPendingOrdersRequest, GetPositionsRequest,
        IpRestriction, OrderDetails, PlaceBatchOrdersRequest, PlaceBatchOrdersResponse,
        PlaceOrderRequest, PlaceOrderResponse, Position, RestClient,
    };
}

pub use credentials::Credentials;
pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use private::{
    AccountBalance, AccountConfig, AmendOrderRequest, AmendOrderResponse, ApiResponse,
    AttachedAlgoOrder, BalanceDetail, CancelBatchOrdersResponse, CancelOrderRequest,
    CancelOrderResponse, CloseOrderAlgo, ClosePositionRequest, ClosePositionResponse, Counterparty,
    EconomicCalendarEvent, Fill, GetAccountBalanceRequest, GetAccountConfigRequest,
    GetEconomicCalendarRequest, GetFillsRequest, GetOrderHistoryRequest, GetOrderRequest,
    GetPendingOrdersRequest, GetPositionsRequest, IpRestriction, OrderDetails,
    PlaceBatchOrdersRequest, PlaceBatchOrdersResponse, PlaceOrderRequest, PlaceOrderResponse,
    Position, RestClient as PrivateRestClient,
};
pub use public::{RestClient as PublicRestClient, *};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};
pub use rate_limiter_trait::OkxRateLimiter;

/// Type alias for results returned by OKX API operations
pub type RestResult<T> = Result<response::ApiResponse<T>, Errors>;
