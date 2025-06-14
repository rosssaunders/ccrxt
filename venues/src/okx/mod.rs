pub mod enums;
mod errors;

pub mod rate_limit;
pub mod public {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{
        GetInstrumentsRequest, GetInstrumentsResponse, Instrument,
        GetTimeResponse, TimeData,
        GetMarkPriceRequest, GetMarkPriceResponse, MarkPriceData,
        GetFundingRateRequest, GetFundingRateResponse, FundingRateData,
        GetOpenInterestRequest, GetOpenInterestResponse, OpenInterestData,
        GetPriceLimitRequest, GetPriceLimitResponse, PriceLimitData,
        GetFundingRateHistoryRequest, GetFundingRateHistoryResponse, FundingRateHistoryData,
        GetPositionTiersRequest, GetPositionTiersResponse, PositionTierData,
        GetUnderlyingRequest, GetUnderlyingResponse,
        GetEstimatedPriceRequest, GetEstimatedPriceResponse, EstimatedPriceData,
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
    GetTimeResponse, TimeData,
    GetMarkPriceRequest, GetMarkPriceResponse, MarkPriceData,
    GetFundingRateRequest, GetFundingRateResponse, FundingRateData,
    GetOpenInterestRequest, GetOpenInterestResponse, OpenInterestData,
    GetPriceLimitRequest, GetPriceLimitResponse, PriceLimitData,
    GetFundingRateHistoryRequest, GetFundingRateHistoryResponse, FundingRateHistoryData,
    GetPositionTiersRequest, GetPositionTiersResponse, PositionTierData,
    GetUnderlyingRequest, GetUnderlyingResponse,
    GetEstimatedPriceRequest, GetEstimatedPriceResponse, EstimatedPriceData,
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