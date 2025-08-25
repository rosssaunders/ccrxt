pub mod enums;
mod errors;
mod response;

pub mod rate_limit;
pub mod rate_limiter_trait;

pub mod public {
    mod rest;
    pub use self::rest::{
        BarSize, BasicInterestRate, ConvertContractCoinData, ConvertContractCoinRequest,
        ConvertContractCoinResponse, DeliveryExerciseDetail, DeliveryExerciseHistory,
        DiscountDetail, DiscountRateInterestFreeQuota, EstimatedPriceData, EstimatedSettlementInfo,
        ExchangeRate, ExchangeRateResponse, FundingRate, FundingRateHistory,
        GetDeliveryExerciseHistoryRequest, GetDeliveryExerciseHistoryResponse,
        GetDiscountRateInterestFreeQuotaRequest, GetDiscountRateInterestFreeQuotaResponse,
        GetEstimatedPriceRequest, GetEstimatedPriceResponse, GetEstimatedSettlementInfoRequest,
        GetEstimatedSettlementInfoResponse, GetFundingRateHistoryRequest,
        GetFundingRateHistoryResponse, GetFundingRateRequest, GetFundingRateResponse,
        GetHistoryIndexCandlesRequest, GetHistoryIndexCandlesResponse,
        GetHistoryMarkPriceCandlesRequest, GetHistoryMarkPriceCandlesResponse,
        GetIndexCandlesRequest, GetIndexCandlesResponse, GetIndexComponentsRequest,
        GetIndexComponentsResponse, GetIndexTickersRequest, GetIndexTickersResponse,
        GetInstrumentTickBandsRequest, GetInstrumentTickBandsResponse, GetInstrumentsRequest,
        GetInstrumentsResponse, GetInsuranceFundRequest, GetInsuranceFundResponse,
        GetInterestRateLoanQuotaRequest, GetInterestRateLoanQuotaResponse,
        GetMarkPriceCandlesHistoryRequest, GetMarkPriceCandlesHistoryResponse,
        GetMarkPriceCandlesRequest, GetMarkPriceCandlesResponse, GetMarkPriceRequest,
        GetMarkPriceResponse, GetOpenInterestRequest, GetOpenInterestResponse,
        GetOptSummaryRequest, GetOptSummaryResponse, GetPositionTiersRequest,
        GetPositionTiersResponse, GetPremiumHistoryRequest, GetPremiumHistoryResponse,
        GetPriceLimitRequest, GetPriceLimitResponse, GetSettlementHistoryRequest,
        GetSettlementHistoryResponse, GetTimeResponse, GetUnderlyingRequest, GetUnderlyingResponse,
        IndexCandle, IndexComponent, IndexComponentData, IndexTicker, Instrument,
        InstrumentTickBandData, InsuranceFundData, InsuranceFundDetail, InterestRateLoanQuotaData,
        MarkPrice, MarkPriceCandle, OpenInterest, OptSummary, PositionTier, PremiumHistory,
        PriceLimit, RegularInterestRate, RestClient, SettlementDetail, SettlementHistory, TickBand,
        TickBandInstrumentType, TimeData, VipInterestRate,
    };
}

pub mod private {
    mod rest;
    pub use self::rest::{
        AccountBalance, AccountConfig, AmendOrderRequest, AmendOrderResponse, AttachedAlgoOrder,
        BalanceDetail, CancelBatchOrdersResponse, CancelOrderRequest, CancelOrderResponse,
        CloseOrderAlgo, ClosePositionRequest, ClosePositionResponse, Fill,
        GetAccountBalanceRequest, GetAccountConfigRequest, GetFillsRequest, GetOrderHistoryRequest,
        GetOrderRequest, GetPendingOrdersRequest, GetPositionsRequest, IpRestriction,
        OkxApiResponse, OrderDetails, PlaceBatchOrdersRequest, PlaceBatchOrdersResponse,
        PlaceOrderRequest, PlaceOrderResponse, Position, RestClient,
    };
}

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use private::{
    AccountBalance, AccountConfig, AmendOrderRequest, AmendOrderResponse, AttachedAlgoOrder,
    BalanceDetail, CancelBatchOrdersResponse, CancelOrderRequest, CancelOrderResponse,
    CloseOrderAlgo, ClosePositionRequest, ClosePositionResponse, Fill, GetAccountBalanceRequest,
    GetAccountConfigRequest, GetFillsRequest, GetOrderHistoryRequest, GetOrderRequest,
    GetPendingOrdersRequest, GetPositionsRequest, IpRestriction, OkxApiResponse, OrderDetails,
    PlaceBatchOrdersRequest, PlaceBatchOrdersResponse, PlaceOrderRequest, PlaceOrderResponse,
    Position, RestClient as PrivateRestClient,
};
pub use public::{
    BarSize, ConvertContractCoinData, ConvertContractCoinRequest, ConvertContractCoinResponse,
    DeliveryExerciseDetail, DeliveryExerciseHistory, DiscountDetail, DiscountRateInterestFreeQuota,
    EstimatedPriceData, EstimatedSettlementInfo, ExchangeRate, ExchangeRateResponse, FundingRate,
    FundingRateHistory, GetDeliveryExerciseHistoryRequest, GetDeliveryExerciseHistoryResponse,
    GetDiscountRateInterestFreeQuotaRequest, GetDiscountRateInterestFreeQuotaResponse,
    GetEstimatedPriceRequest, GetEstimatedPriceResponse, GetEstimatedSettlementInfoRequest,
    GetEstimatedSettlementInfoResponse, GetFundingRateHistoryRequest,
    GetFundingRateHistoryResponse, GetFundingRateRequest, GetFundingRateResponse,
    GetHistoryIndexCandlesRequest, GetHistoryIndexCandlesResponse,
    GetHistoryMarkPriceCandlesRequest, GetHistoryMarkPriceCandlesResponse, GetIndexCandlesRequest,
    GetIndexCandlesResponse, GetIndexComponentsRequest, GetIndexComponentsResponse,
    GetIndexTickersRequest, GetIndexTickersResponse, GetInstrumentTickBandsRequest,
    GetInstrumentTickBandsResponse, GetInstrumentsRequest, GetInstrumentsResponse,
    GetInsuranceFundRequest, GetInsuranceFundResponse, GetInterestRateLoanQuotaRequest,
    GetInterestRateLoanQuotaResponse, GetMarkPriceCandlesHistoryRequest,
    GetMarkPriceCandlesHistoryResponse, GetMarkPriceCandlesRequest, GetMarkPriceCandlesResponse,
    GetMarkPriceRequest, GetMarkPriceResponse, GetOpenInterestRequest, GetOpenInterestResponse,
    GetOptSummaryRequest, GetOptSummaryResponse, GetPositionTiersRequest, GetPositionTiersResponse,
    GetPremiumHistoryRequest, GetPremiumHistoryResponse, GetPriceLimitRequest,
    GetPriceLimitResponse, GetSettlementHistoryRequest, GetSettlementHistoryResponse,
    GetTimeResponse, GetUnderlyingRequest, GetUnderlyingResponse, IndexCandle, IndexComponent,
    IndexComponentData, IndexTicker, Instrument, InstrumentTickBandData, InsuranceFundData,
    InsuranceFundDetail, InterestRateLoanQuotaData, MarkPrice, MarkPriceCandle, OpenInterest,
    OptSummary, PositionTier, PremiumHistory, PriceLimit, RestClient as PublicRestClient,
    SettlementDetail, SettlementHistory, TickBand, TickBandInstrumentType, TimeData,
};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};
pub use rate_limiter_trait::OkxRateLimiter;

/// Type alias for results returned by OKX API operations
pub type RestResult<T> = Result<response::OkxApiResponse<T>, Errors>;
