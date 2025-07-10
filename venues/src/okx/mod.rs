pub mod enums;
mod errors;

pub mod rate_limit;
pub mod public {
    mod rest;
    pub use self::rest::{
        BarSize, BasicInterestRate, ConvertContractCoinData, ConvertContractCoinRequest,
        ConvertContractCoinResponse, DeliveryExerciseDetail, DeliveryExerciseHistory,
        DiscountDetail, DiscountRateInterestFreeQuota, EstimatedPriceData, ExchangeRate,
        ExchangeRateResponse, FundingRate, FundingRateHistory, GetDeliveryExerciseHistoryRequest,
        GetDeliveryExerciseHistoryResponse, GetDiscountRateInterestFreeQuotaRequest,
        GetDiscountRateInterestFreeQuotaResponse, GetEstimatedPriceRequest,
        GetEstimatedPriceResponse, GetFundingRateHistoryRequest, GetFundingRateHistoryResponse,
        GetFundingRateRequest, GetFundingRateResponse, GetHistoryIndexCandlesRequest,
        GetHistoryIndexCandlesResponse, GetHistoryMarkPriceCandlesRequest,
        GetHistoryMarkPriceCandlesResponse, GetIndexCandlesRequest, GetIndexCandlesResponse,
        GetIndexComponentsRequest, GetIndexComponentsResponse, GetIndexTickersRequest,
        GetIndexTickersResponse, GetInstrumentTickBandsRequest, GetInstrumentTickBandsResponse,
        GetInstrumentsRequest, GetInstrumentsResponse, GetInsuranceFundRequest,
        GetInsuranceFundResponse, GetInterestRateLoanQuotaRequest,
        GetInterestRateLoanQuotaResponse, GetMarkPriceCandlesHistoryRequest,
        GetMarkPriceCandlesHistoryResponse, GetMarkPriceCandlesRequest,
        GetMarkPriceCandlesResponse, GetMarkPriceRequest, GetMarkPriceResponse,
        GetOpenInterestRequest, GetOpenInterestResponse, GetOptSummaryRequest,
        GetOptSummaryResponse, GetPositionTiersRequest, GetPositionTiersResponse,
        GetPremiumHistoryRequest, GetPremiumHistoryResponse, GetPriceLimitRequest,
        GetPriceLimitResponse, GetSettlementHistoryRequest, GetSettlementHistoryResponse,
        GetTimeResponse, GetUnderlyingRequest, GetUnderlyingResponse, IndexCandle, IndexComponent,
        IndexComponentData, IndexTicker, Instrument, InstrumentTickBandData, InsuranceFundData,
        InsuranceFundDetail, InterestRateLoanQuotaData, MarkPrice, MarkPriceCandle, OpenInterest,
        OptSummary, PositionTier, PremiumHistory, PriceLimit, RegularInterestRate, RestClient,
        SettlementDetail, SettlementHistory, TickBand, TickBandInstrumentType, TimeData,
        UnderlyingData, VipInterestRate,
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
    EstimatedPriceData, ExchangeRate, ExchangeRateResponse, FundingRate, FundingRateHistory,
    GetDeliveryExerciseHistoryRequest, GetDeliveryExerciseHistoryResponse,
    GetDiscountRateInterestFreeQuotaRequest, GetDiscountRateInterestFreeQuotaResponse,
    GetEstimatedPriceRequest, GetEstimatedPriceResponse, GetFundingRateHistoryRequest,
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
    UnderlyingData,
};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by OKX API operations
pub type RestResult<T> = Result<T, Errors>;
