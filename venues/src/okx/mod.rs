pub mod enums;
mod errors;

pub mod rate_limit;
pub mod public {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{
        GetHistoryMarkPriceCandlesRequest, GetHistoryMarkPriceCandlesResponse,
        HistoryMarkPriceCandle,
    };
    pub use self::rest::{GetInstrumentsRequest, GetInstrumentsResponse, Instrument};
    pub use self::rest::{GetMarkPriceCandlesRequest, GetMarkPriceCandlesResponse, MarkPriceCandle};
    pub use self::rest::{GetMarkPriceRequest, GetMarkPriceResponse, MarkPrice};
    pub use self::rest::{GetPremiumHistoryRequest, GetPremiumHistoryResponse, PremiumHistory};
}

pub mod private {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{
        AccountBalance, AccountConfig, AmendOrderRequest, AmendOrderResponse, AttachedAlgoOrder,
        BalanceDetail, CancelBatchOrdersResponse, CancelOrderRequest, CancelOrderResponse,
        CloseOrderAlgo, ClosePositionRequest, ClosePositionResponse, Fill,
        GetAccountBalanceRequest, GetAccountConfigRequest, GetFillsRequest, GetOrderHistoryRequest,
        GetOrderRequest, GetPendingOrdersRequest, GetPositionsRequest, IpRestriction,
        OkxApiResponse, OrderDetails, PlaceBatchOrdersRequest, PlaceBatchOrdersResponse,
        PlaceOrderRequest, PlaceOrderResponse, Position,
    };
}

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use private::RestClient as PrivateRestClient;
pub use private::{
    AccountBalance, AccountConfig, AmendOrderRequest, AmendOrderResponse, AttachedAlgoOrder,
    BalanceDetail, CancelBatchOrdersResponse, CancelOrderRequest, CancelOrderResponse,
    CloseOrderAlgo, ClosePositionRequest, ClosePositionResponse, Fill, GetAccountBalanceRequest,
    GetAccountConfigRequest, GetFillsRequest, GetOrderHistoryRequest, GetOrderRequest,
    GetPendingOrdersRequest, GetPositionsRequest, IpRestriction, OkxApiResponse, OrderDetails,
    PlaceBatchOrdersRequest, PlaceBatchOrdersResponse, PlaceOrderRequest, PlaceOrderResponse,
    Position,
};
pub use public::RestClient as PublicRestClient;
pub use public::{
    GetHistoryMarkPriceCandlesRequest, GetHistoryMarkPriceCandlesResponse, HistoryMarkPriceCandle,
};
pub use public::{GetInstrumentsRequest, GetInstrumentsResponse, Instrument};
pub use public::{GetMarkPriceCandlesRequest, GetMarkPriceCandlesResponse, MarkPriceCandle};
pub use public::{GetMarkPriceRequest, GetMarkPriceResponse, MarkPrice};
pub use public::{GetPremiumHistoryRequest, GetPremiumHistoryResponse, PremiumHistory};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by OKX API operations
pub type RestResult<T> = Result<T, Errors>;
