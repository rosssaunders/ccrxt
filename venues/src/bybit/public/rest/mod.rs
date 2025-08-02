pub mod client;

// Market Data endpoints
pub mod get_kline;
pub mod get_orderbook;
pub mod get_recent_trades;
pub mod get_server_time;
pub mod get_tickers;

// Price Kline endpoints
pub mod get_index_price_kline;
pub mod get_mark_price_kline;
pub mod get_premium_index_price_kline;

// Instrument & Trading endpoints
pub mod get_funding_history;
pub mod get_instruments_info;

// Market Statistics endpoints
pub mod get_historical_volatility;
pub mod get_open_interest;

// Risk Management endpoints
pub mod get_delivery_price;
pub mod get_insurance;
pub mod get_risk_limit;

// Account Ratio endpoint
pub mod get_long_short_ratio;

// Margin Trade endpoints
pub mod get_collateral_ratio;
pub mod get_vip_margin_data;

// Loan endpoints
pub mod get_borrowable_coins;
pub mod get_collateral_coins;
pub mod get_ins_margin_coin_info;
pub mod get_ins_product_info;

pub use client::RestClient;
// Re-export margin & loan endpoints
pub use get_borrowable_coins::{
    GetBorrowableCoinsData, GetBorrowableCoinsRequest, GetBorrowableCoinsResponse,
};
pub use get_collateral_coins::{
    GetCollateralCoinsData, GetCollateralCoinsRequest, GetCollateralCoinsResponse,
};
pub use get_collateral_ratio::{
    GetCollateralRatioData, GetCollateralRatioRequest, GetCollateralRatioResponse,
};
// Re-export risk management endpoints
pub use get_delivery_price::{
    GetDeliveryPriceData, GetDeliveryPriceRequest, GetDeliveryPriceResponse,
};
// Re-export trading & market statistics endpoints
pub use get_funding_history::{
    GetFundingHistoryData, GetFundingHistoryRequest, GetFundingHistoryResponse,
};
pub use get_historical_volatility::{
    GetHistoricalVolatilityData, GetHistoricalVolatilityRequest, GetHistoricalVolatilityResponse,
};
// Re-export price kline endpoints
pub use get_index_price_kline::{
    GetIndexPriceKlineData, GetIndexPriceKlineRequest, GetIndexPriceKlineResponse,
};
pub use get_ins_margin_coin_info::{
    GetInsMarginCoinInfoData, GetInsMarginCoinInfoRequest, GetInsMarginCoinInfoResponse,
};
pub use get_ins_product_info::{
    GetInsProductInfoData, GetInsProductInfoRequest, GetInsProductInfoResponse,
};
pub use get_instruments_info::{
    GetInstrumentsInfoData, GetInstrumentsInfoRequest, GetInstrumentsInfoResponse, InstrumentInfo,
};
pub use get_insurance::{GetInsuranceData, GetInsuranceRequest, GetInsuranceResponse};
pub use get_kline::{GetKlineData, GetKlineRequest, GetKlineResponse, Kline};
pub use get_long_short_ratio::{
    GetLongShortRatioData, GetLongShortRatioRequest, GetLongShortRatioResponse,
};
pub use get_mark_price_kline::{
    GetMarkPriceKlineData, GetMarkPriceKlineRequest, GetMarkPriceKlineResponse,
};
pub use get_open_interest::{GetOpenInterestData, GetOpenInterestRequest, GetOpenInterestResponse};
pub use get_orderbook::{
    GetOrderbookData, GetOrderbookRequest, GetOrderbookResponse, OrderbookLevel,
};
pub use get_premium_index_price_kline::{
    GetPremiumIndexPriceKlineData, GetPremiumIndexPriceKlineRequest,
    GetPremiumIndexPriceKlineResponse,
};
pub use get_recent_trades::{
    GetRecentTradesData, GetRecentTradesRequest, GetRecentTradesResponse, TradeInfo,
};
pub use get_risk_limit::{GetRiskLimitData, GetRiskLimitRequest, GetRiskLimitResponse};
// Re-export key types for integration tests
pub use get_server_time::{GetServerTimeRequest, GetServerTimeResponse, ServerTimeData};
pub use get_tickers::{GetTickersData, GetTickersRequest, GetTickersResponse, TickerInfo};
pub use get_vip_margin_data::{
    GetVipMarginDataData, GetVipMarginDataRequest, GetVipMarginDataResponse,
};
