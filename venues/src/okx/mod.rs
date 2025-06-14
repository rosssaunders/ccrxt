pub mod enums;
mod errors;
#[cfg(test)]
mod integration_tests;

pub mod rate_limit;
pub mod public {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{EconomicEvent, GetEconomicCalendarRequest, GetEconomicCalendarResponse};
    pub use self::rest::{ExchangeRate, ExchangeRateResponse};
    pub use self::rest::{GetHistoryIndexCandlesRequest, GetHistoryIndexCandlesResponse, IndexCandle};
    pub use self::rest::{GetInstrumentsRequest, GetInstrumentsResponse, Instrument};
    pub use self::rest::{GetIndexTickersRequest, GetIndexTickersResponse, IndexTicker};
    pub use self::rest::{GetMarkPriceCandlesRequest, GetMarkPriceCandlesResponse};
    pub use self::rest::{GetMarkPriceCandlesHistoryRequest, GetMarkPriceCandlesHistoryResponse};
    pub use self::rest::{GetIndexCandlesRequest, GetIndexCandlesResponse};
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

mod integration_test;
mod usage_examples;

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
pub use public::{EconomicEvent, GetEconomicCalendarRequest, GetEconomicCalendarResponse};
pub use public::{ExchangeRate, ExchangeRateResponse};
pub use public::{GetHistoryIndexCandlesRequest, GetHistoryIndexCandlesResponse, IndexCandle};
pub use public::{GetInstrumentsRequest, GetInstrumentsResponse, Instrument};
pub use public::{GetIndexTickersRequest, GetIndexTickersResponse, IndexTicker};
pub use public::{GetMarkPriceCandlesRequest, GetMarkPriceCandlesResponse};
pub use public::{GetMarkPriceCandlesHistoryRequest, GetMarkPriceCandlesHistoryResponse};
pub use public::{GetIndexCandlesRequest, GetIndexCandlesResponse};
pub use public::{GetPremiumHistoryRequest, GetPremiumHistoryResponse, PremiumHistory};

pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by OKX API operations
pub type RestResult<T> = Result<T, Errors>;
