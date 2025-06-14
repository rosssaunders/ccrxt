pub mod enums;
mod errors;

pub mod rate_limit;
pub mod public {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{
        GetInstrumentsRequest, GetInstrumentsResponse, Instrument,
        GetSystemTimeResponse, SystemTime,
        GetFundingRateRequest, GetFundingRateResponse, FundingRate,
        GetMarkPriceRequest, GetMarkPriceResponse, MarkPrice,
        GetOpenInterestRequest, GetOpenInterestResponse, OpenInterest,
        GetLimitPriceRequest, GetLimitPriceResponse, LimitPrice,
        GetIndexTickersRequest, GetIndexTickersResponse, IndexTicker,
        GetExchangeRateResponse, ExchangeRate,
    };
}

pub mod private {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{
        OkxApiResponse, PlaceOrderRequest, PlaceOrderResponse, AttachedAlgoOrder,
        CancelOrderRequest, CancelOrderResponse, GetOrderRequest, OrderDetails,
        GetPendingOrdersRequest, GetOrderHistoryRequest, PlaceBatchOrdersRequest,
        PlaceBatchOrdersResponse, CancelBatchOrdersResponse, AmendOrderRequest,
        AmendOrderResponse, ClosePositionRequest, ClosePositionResponse,
        GetAccountBalanceRequest, AccountBalance, BalanceDetail, GetPositionsRequest,
        Position, CloseOrderAlgo, GetFillsRequest, Fill, GetAccountConfigRequest,
        AccountConfig, IpRestriction,
    };
}

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use public::RestClient as PublicRestClient;
pub use public::{
    GetInstrumentsRequest, GetInstrumentsResponse, Instrument,
    GetSystemTimeResponse, SystemTime,
    GetFundingRateRequest, GetFundingRateResponse, FundingRate,
    GetMarkPriceRequest, GetMarkPriceResponse, MarkPrice,
    GetOpenInterestRequest, GetOpenInterestResponse, OpenInterest,
    GetLimitPriceRequest, GetLimitPriceResponse, LimitPrice,
    GetIndexTickersRequest, GetIndexTickersResponse, IndexTicker,
    GetExchangeRateResponse, ExchangeRate,
};
pub use private::RestClient as PrivateRestClient;
pub use private::{
    OkxApiResponse, PlaceOrderRequest, PlaceOrderResponse, AttachedAlgoOrder,
    CancelOrderRequest, CancelOrderResponse, GetOrderRequest, OrderDetails,
    GetPendingOrdersRequest, GetOrderHistoryRequest, PlaceBatchOrdersRequest,
    PlaceBatchOrdersResponse, CancelBatchOrdersResponse, AmendOrderRequest,
    AmendOrderResponse, ClosePositionRequest, ClosePositionResponse,
    GetAccountBalanceRequest, AccountBalance, BalanceDetail, GetPositionsRequest,
    Position, CloseOrderAlgo, GetFillsRequest, Fill, GetAccountConfigRequest,
    AccountConfig, IpRestriction,
};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by OKX API operations
pub type RestResult<T> = Result<T, Errors>;