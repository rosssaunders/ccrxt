pub mod enums;
pub mod errors;
pub mod rate_limit;

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
        GetInstrumentsInfoData,
        GetInstrumentsInfoRequest,
        GetInstrumentsInfoResponse,
        GetKlineData,
        GetKlineRequest,
        GetKlineResponse,
        GetOrderbookData,
        GetOrderbookRequest,
        GetOrderbookResponse,
        GetRecentTradesData,
        GetRecentTradesRequest,
        GetRecentTradesResponse,
        // Re-export public REST types for integration tests
        GetServerTimeRequest,
        GetServerTimeResponse,
        GetTickersData,
        GetTickersRequest,
        GetTickersResponse,
        InstrumentInfo,
        Kline,
        OrderbookLevel,
        RestClient as PublicRestClient,
        ServerTimeData,
        TickerInfo,
        TradeInfo,
        // Price kline endpoints
        GetIndexPriceKlineRequest,
        GetIndexPriceKlineResponse,
        GetIndexPriceKlineData,
        GetMarkPriceKlineRequest,
        GetMarkPriceKlineResponse,
        GetMarkPriceKlineData,
        GetPremiumIndexPriceKlineRequest,
        GetPremiumIndexPriceKlineResponse,
        GetPremiumIndexPriceKlineData,
        // Trading & market statistics endpoints
        GetFundingHistoryRequest,
        GetFundingHistoryResponse,
        GetFundingHistoryData,
        GetHistoricalVolatilityRequest,
        GetHistoricalVolatilityResponse,
        GetHistoricalVolatilityData,
        GetOpenInterestRequest,
        GetOpenInterestResponse,
        GetOpenInterestData,
        GetLongShortRatioRequest,
        GetLongShortRatioResponse,
        GetLongShortRatioData,
        // Risk management endpoints
        GetDeliveryPriceRequest,
        GetDeliveryPriceResponse,
        GetDeliveryPriceData,
        GetInsuranceRequest,
        GetInsuranceResponse,
        GetInsuranceData,
        GetRiskLimitRequest,
        GetRiskLimitResponse,
        GetRiskLimitData,
        // Margin & loan endpoints
        GetVipMarginDataRequest,
        GetVipMarginDataResponse,
        GetVipMarginDataData,
        GetCollateralRatioRequest,
        GetCollateralRatioResponse,
        GetCollateralRatioData,
        GetBorrowableCoinsRequest,
        GetBorrowableCoinsResponse,
        GetBorrowableCoinsData,
        GetCollateralCoinsRequest,
        GetCollateralCoinsResponse,
        GetCollateralCoinsData,
        GetInsMarginCoinInfoRequest,
        GetInsMarginCoinInfoResponse,
        GetInsMarginCoinInfoData,
        GetInsProductInfoRequest,
        GetInsProductInfoResponse,
        GetInsProductInfoData,
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
    GetInstrumentsInfoData, GetInstrumentsInfoRequest, GetInstrumentsInfoResponse, GetKlineData,
    GetKlineRequest, GetKlineResponse, GetOrderbookData, GetOrderbookRequest, GetOrderbookResponse,
    GetRecentTradesData, GetRecentTradesRequest, GetRecentTradesResponse, GetServerTimeRequest,
    GetServerTimeResponse, GetTickersData, GetTickersRequest, GetTickersResponse, InstrumentInfo,
    Kline, OrderbookLevel, ServerTimeData, TickerInfo, TradeInfo,
    // Price kline endpoints
    GetIndexPriceKlineRequest, GetIndexPriceKlineResponse, GetIndexPriceKlineData,
    GetMarkPriceKlineRequest, GetMarkPriceKlineResponse, GetMarkPriceKlineData,
    GetPremiumIndexPriceKlineRequest, GetPremiumIndexPriceKlineResponse, GetPremiumIndexPriceKlineData,
    // Trading & market statistics endpoints
    GetFundingHistoryRequest, GetFundingHistoryResponse, GetFundingHistoryData,
    GetHistoricalVolatilityRequest, GetHistoricalVolatilityResponse, GetHistoricalVolatilityData,
    GetOpenInterestRequest, GetOpenInterestResponse, GetOpenInterestData,
    GetLongShortRatioRequest, GetLongShortRatioResponse, GetLongShortRatioData,
    // Risk management endpoints
    GetDeliveryPriceRequest, GetDeliveryPriceResponse, GetDeliveryPriceData,
    GetInsuranceRequest, GetInsuranceResponse, GetInsuranceData,
    GetRiskLimitRequest, GetRiskLimitResponse, GetRiskLimitData,
    // Margin & loan endpoints
    GetVipMarginDataRequest, GetVipMarginDataResponse, GetVipMarginDataData,
    GetCollateralRatioRequest, GetCollateralRatioResponse, GetCollateralRatioData,
    GetBorrowableCoinsRequest, GetBorrowableCoinsResponse, GetBorrowableCoinsData,
    GetCollateralCoinsRequest, GetCollateralCoinsResponse, GetCollateralCoinsData,
    GetInsMarginCoinInfoRequest, GetInsMarginCoinInfoResponse, GetInsMarginCoinInfoData,
    GetInsProductInfoRequest, GetInsProductInfoResponse, GetInsProductInfoData,
};
// Note: Trade and Position endpoint types are available via the private module
// Example usage: bybit::private::CreateOrderRequest
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by ByBit API operations
pub type RestResult<T> = Result<T, Errors>;
