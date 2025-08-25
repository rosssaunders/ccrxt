pub mod enums;
pub mod errors;
pub mod rate_limit;
pub mod rate_limiter_trait;

pub mod private {
    mod rest;

    pub use self::rest::{
        BalanceData, GetWalletBalanceRequest, GetWalletBalanceResponse,
        RestClient as PrivateRestClient, WalletBalance,
    };
}

pub mod public {
    mod rest;

    pub use self::rest::{
        GetBorrowableCoinsData,
        GetBorrowableCoinsRequest,
        GetBorrowableCoinsResponse,
        GetCollateralCoinsData,
        GetCollateralCoinsRequest,
        GetCollateralCoinsResponse,
        GetCollateralRatioData,
        GetCollateralRatioRequest,
        GetCollateralRatioResponse,
        GetDeliveryPriceData,
        // Risk management endpoints
        GetDeliveryPriceRequest,
        GetDeliveryPriceResponse,
        GetFundingHistoryData,
        // Trading & market statistics endpoints
        GetFundingHistoryRequest,
        GetFundingHistoryResponse,
        GetHistoricalVolatilityData,
        GetHistoricalVolatilityRequest,
        GetHistoricalVolatilityResponse,
        GetIndexPriceKlineData,
        // Price kline endpoints
        GetIndexPriceKlineRequest,
        GetIndexPriceKlineResponse,
        GetInsMarginCoinInfoData,
        GetInsMarginCoinInfoRequest,
        GetInsMarginCoinInfoResponse,
        GetInsProductInfoData,
        GetInsProductInfoRequest,
        GetInsProductInfoResponse,
        GetInstrumentsInfoData,
        GetInstrumentsInfoRequest,
        GetInstrumentsInfoResponse,
        GetInsuranceData,
        GetInsuranceRequest,
        GetInsuranceResponse,
        GetKlineData,
        GetKlineRequest,
        GetKlineResponse,
        GetLongShortRatioData,
        GetLongShortRatioRequest,
        GetLongShortRatioResponse,
        GetMarkPriceKlineData,
        GetMarkPriceKlineRequest,
        GetMarkPriceKlineResponse,
        GetOpenInterestData,
        GetOpenInterestRequest,
        GetOpenInterestResponse,
        GetOrderbookData,
        GetOrderbookRequest,
        GetOrderbookResponse,
        GetPremiumIndexPriceKlineData,
        GetPremiumIndexPriceKlineRequest,
        GetPremiumIndexPriceKlineResponse,
        GetRecentTradesData,
        GetRecentTradesRequest,
        GetRecentTradesResponse,
        GetRiskLimitData,
        GetRiskLimitRequest,
        GetRiskLimitResponse,
        // Re-export public REST types for integration tests
        GetServerTimeRequest,
        GetServerTimeResponse,
        GetTickersData,
        GetTickersRequest,
        GetTickersResponse,
        GetVipMarginDataData,
        // Margin & loan endpoints
        GetVipMarginDataRequest,
        GetVipMarginDataResponse,
        InstrumentInfo,
        Kline,
        OrderbookLevel,
        RestClient as PublicRestClient,
        ServerTimeData,
        TickerInfo,
        TradeInfo,
    };
}

// Re-export public modules
pub use enums::*;
pub use errors::{ApiError, Errors};
// Export clients
pub use private::PrivateRestClient;
pub use private::{BalanceData, GetWalletBalanceRequest, GetWalletBalanceResponse, WalletBalance};
pub use public::PublicRestClient;
// Re-export public REST types for integration tests
pub use public::{
    GetBorrowableCoinsData,
    GetBorrowableCoinsRequest,
    GetBorrowableCoinsResponse,
    GetCollateralCoinsData,
    GetCollateralCoinsRequest,
    GetCollateralCoinsResponse,
    GetCollateralRatioData,
    GetCollateralRatioRequest,
    GetCollateralRatioResponse,
    GetDeliveryPriceData,
    // Risk management endpoints
    GetDeliveryPriceRequest,
    GetDeliveryPriceResponse,
    GetFundingHistoryData,
    // Trading & market statistics endpoints
    GetFundingHistoryRequest,
    GetFundingHistoryResponse,
    GetHistoricalVolatilityData,
    GetHistoricalVolatilityRequest,
    GetHistoricalVolatilityResponse,
    GetIndexPriceKlineData,
    // Price kline endpoints
    GetIndexPriceKlineRequest,
    GetIndexPriceKlineResponse,
    GetInsMarginCoinInfoData,
    GetInsMarginCoinInfoRequest,
    GetInsMarginCoinInfoResponse,
    GetInsProductInfoData,
    GetInsProductInfoRequest,
    GetInsProductInfoResponse,
    GetInstrumentsInfoData,
    GetInstrumentsInfoRequest,
    GetInstrumentsInfoResponse,
    GetInsuranceData,
    GetInsuranceRequest,
    GetInsuranceResponse,
    GetKlineData,
    GetKlineRequest,
    GetKlineResponse,
    GetLongShortRatioData,
    GetLongShortRatioRequest,
    GetLongShortRatioResponse,
    GetMarkPriceKlineData,
    GetMarkPriceKlineRequest,
    GetMarkPriceKlineResponse,
    GetOpenInterestData,
    GetOpenInterestRequest,
    GetOpenInterestResponse,
    GetOrderbookData,
    GetOrderbookRequest,
    GetOrderbookResponse,
    GetPremiumIndexPriceKlineData,
    GetPremiumIndexPriceKlineRequest,
    GetPremiumIndexPriceKlineResponse,
    GetRecentTradesData,
    GetRecentTradesRequest,
    GetRecentTradesResponse,
    GetRiskLimitData,
    GetRiskLimitRequest,
    GetRiskLimitResponse,
    GetServerTimeRequest,
    GetServerTimeResponse,
    GetTickersData,
    GetTickersRequest,
    GetTickersResponse,
    GetVipMarginDataData,
    // Margin & loan endpoints
    GetVipMarginDataRequest,
    GetVipMarginDataResponse,
    InstrumentInfo,
    Kline,
    OrderbookLevel,
    ServerTimeData,
    TickerInfo,
    TradeInfo,
};
// Note: Trade and Position endpoint types are available via the private module
// Example usage: bybit::private::CreateOrderRequest
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by ByBit API operations
pub type RestResult<T> = Result<T, Errors>;
