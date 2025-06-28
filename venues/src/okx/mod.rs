pub mod enums;
mod errors;

pub mod rate_limit;
pub mod public {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{
        BarSize, GetHistoryMarkPriceCandlesRequest, GetHistoryMarkPriceCandlesResponse,
        MarkPriceCandle,
    };
    pub use self::rest::{
        BasicInterestRate, GetInterestRateLoanQuotaRequest, GetInterestRateLoanQuotaResponse,
        InterestRateLoanQuotaData, RegularInterestRate, VipInterestRate,
    };
    pub use self::rest::{
        ConvertContractCoinData, ConvertContractCoinRequest, ConvertContractCoinResponse,
    };
    pub use self::rest::{
        DeliveryExerciseDetail, DeliveryExerciseHistory, GetDeliveryExerciseHistoryRequest,
        GetDeliveryExerciseHistoryResponse,
    };
    pub use self::rest::{
        DiscountDetail, DiscountRateInterestFreeQuota, GetDiscountRateInterestFreeQuotaRequest,
        GetDiscountRateInterestFreeQuotaResponse,
    };
    pub use self::rest::{EconomicEvent, GetEconomicCalendarRequest, GetEconomicCalendarResponse};
    pub use self::rest::{EstimatedPriceData, GetEstimatedPriceRequest, GetEstimatedPriceResponse};
    pub use self::rest::{ExchangeRate, ExchangeRateResponse};
    pub use self::rest::{FundingRate, GetFundingRateRequest, GetFundingRateResponse};
    pub use self::rest::{
        FundingRateHistory, GetFundingRateHistoryRequest, GetFundingRateHistoryResponse,
    };
    pub use self::rest::{
        GetHistoryIndexCandlesRequest, GetHistoryIndexCandlesResponse, IndexCandle,
    };
    pub use self::rest::{GetIndexCandlesRequest, GetIndexCandlesResponse};
    pub use self::rest::{
        GetIndexComponentsRequest, GetIndexComponentsResponse, IndexComponent, IndexComponentData,
    };
    pub use self::rest::{GetIndexTickersRequest, GetIndexTickersResponse, IndexTicker};
    pub use self::rest::{
        GetInstrumentTickBandsRequest, GetInstrumentTickBandsResponse, InstrumentTickBandData,
        TickBand,
    };
    pub use self::rest::{GetInstrumentsRequest, GetInstrumentsResponse, Instrument};
    pub use self::rest::{
        GetInsuranceFundRequest, GetInsuranceFundResponse, InsuranceFundData, InsuranceFundDetail,
    };
    pub use self::rest::{GetMarkPriceCandlesHistoryRequest, GetMarkPriceCandlesHistoryResponse};
    pub use self::rest::{GetMarkPriceCandlesRequest, GetMarkPriceCandlesResponse};
    pub use self::rest::{GetMarkPriceRequest, GetMarkPriceResponse, MarkPrice};
    pub use self::rest::{GetOpenInterestRequest, GetOpenInterestResponse, OpenInterest};
    pub use self::rest::{GetOptSummaryRequest, GetOptSummaryResponse, OptSummary};
    pub use self::rest::{GetPositionTiersRequest, GetPositionTiersResponse, PositionTier};
    pub use self::rest::{GetPremiumHistoryRequest, GetPremiumHistoryResponse, PremiumHistory};
    pub use self::rest::{GetPriceLimitRequest, GetPriceLimitResponse, PriceLimit};
    pub use self::rest::{
        GetSettlementHistoryRequest, GetSettlementHistoryResponse, SettlementDetail,
        SettlementHistory,
    };
    pub use self::rest::{GetTimeResponse, TimeData};
    pub use self::rest::{GetUnderlyingRequest, GetUnderlyingResponse, UnderlyingData};
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
    BarSize, GetHistoryMarkPriceCandlesRequest, GetHistoryMarkPriceCandlesResponse, MarkPriceCandle,
};
pub use public::{
    ConvertContractCoinData, ConvertContractCoinRequest, ConvertContractCoinResponse,
};
pub use public::{
    DeliveryExerciseDetail, DeliveryExerciseHistory, GetDeliveryExerciseHistoryRequest,
    GetDeliveryExerciseHistoryResponse,
};
pub use public::{EconomicEvent, GetEconomicCalendarRequest, GetEconomicCalendarResponse};
pub use public::{EstimatedPriceData, GetEstimatedPriceRequest, GetEstimatedPriceResponse};
pub use public::{ExchangeRate, ExchangeRateResponse};
pub use public::{FundingRate, GetFundingRateRequest, GetFundingRateResponse};
pub use public::{FundingRateHistory, GetFundingRateHistoryRequest, GetFundingRateHistoryResponse};
pub use public::{GetHistoryIndexCandlesRequest, GetHistoryIndexCandlesResponse, IndexCandle};
pub use public::{GetIndexCandlesRequest, GetIndexCandlesResponse};
pub use public::{
    GetIndexComponentsRequest, GetIndexComponentsResponse, IndexComponent, IndexComponentData,
};
pub use public::{GetIndexTickersRequest, GetIndexTickersResponse, IndexTicker};
pub use public::{
    GetInstrumentTickBandsRequest, GetInstrumentTickBandsResponse, InstrumentTickBandData, TickBand,
};
pub use public::{GetInstrumentsRequest, GetInstrumentsResponse, Instrument};
pub use public::{
    GetInsuranceFundRequest, GetInsuranceFundResponse, InsuranceFundData, InsuranceFundDetail,
};
pub use public::{GetMarkPriceCandlesHistoryRequest, GetMarkPriceCandlesHistoryResponse};
pub use public::{GetMarkPriceCandlesRequest, GetMarkPriceCandlesResponse};
pub use public::{GetMarkPriceRequest, GetMarkPriceResponse, MarkPrice};
pub use public::{GetOpenInterestRequest, GetOpenInterestResponse, OpenInterest};
pub use public::{GetOptSummaryRequest, GetOptSummaryResponse, OptSummary};
pub use public::{GetPremiumHistoryRequest, GetPremiumHistoryResponse, PremiumHistory};
pub use public::{GetPriceLimitRequest, GetPriceLimitResponse, PriceLimit};
pub use public::{GetTimeResponse, TimeData};
pub use public::{GetUnderlyingRequest, GetUnderlyingResponse, UnderlyingData};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by OKX API operations
pub type RestResult<T> = Result<T, Errors>;
