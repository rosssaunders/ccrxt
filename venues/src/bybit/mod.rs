pub mod enums;
pub mod errors;
pub mod rate_limit;
pub mod rate_limiter_trait;

// Client modules
pub mod private_client;
pub mod public_client;

pub mod private {
    pub mod rest;

    pub use self::rest::{
        BalanceData, GetWalletBalanceRequest, GetWalletBalanceResponse, WalletBalance,
    };
}

pub mod public {
    pub mod rest;

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
        ServerTimeData,
        TickerInfo,
        TradeInfo,
    };
}

// Re-export public modules
pub use enums::*;
pub use errors::{ApiError, Errors};
// Keep backwards compatibility for other types
pub use private::{BalanceData, GetWalletBalanceRequest, GetWalletBalanceResponse, WalletBalance};
// Export clients (new locations take precedence)
pub use private_client::RestClient as PrivateRestClient;
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
pub use public_client::RestClient as PublicRestClient;
// Note: Trade and Position endpoint types are available via the private module
// Example usage: bybit::private::CreateOrderRequest
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by ByBit API operations
pub type RestResult<T> = Result<T, Errors>;
