mod client;
mod convert_contract_coin;
mod exchange_rate;
mod get_delivery_exercise_history;
mod get_discount_rate_interest_free_quota;
mod get_estimated_price;
mod get_estimated_settlement_info;
mod get_funding_rate;
mod get_funding_rate_history;
mod get_history_index_candles;
mod get_history_mark_price_candles;
mod get_index_candles;
mod get_index_components;
mod get_index_tickers;
mod get_instrument_tick_bands;
mod get_instruments;
mod get_insurance_fund;
mod get_interest_rate_loan_quota;
mod get_mark_price;
mod get_mark_price_candles;
mod get_mark_price_candles_history;
mod get_open_interest;
mod get_opt_summary;
mod get_position_tiers;
mod get_premium_history;
mod get_price_limit;
mod get_settlement_history;
mod get_time;
mod get_underlying;

use crate::okx::response::OkxApiResponse;

pub use client::RestClient;
pub use convert_contract_coin::{ConvertContractCoinData, ConvertContractCoinRequest};
pub type ConvertContractCoinResponse = OkxApiResponse<ConvertContractCoinData>;
pub use exchange_rate::ExchangeRate;
pub type ExchangeRateResponse = OkxApiResponse<ExchangeRate>;
pub use get_delivery_exercise_history::{
    DeliveryExerciseDetail, DeliveryExerciseHistory, GetDeliveryExerciseHistoryRequest,
};
pub type GetDeliveryExerciseHistoryResponse = OkxApiResponse<DeliveryExerciseHistory>;
pub use get_discount_rate_interest_free_quota::{
    DiscountDetail, DiscountRateInterestFreeQuota, GetDiscountRateInterestFreeQuotaRequest,
};
pub type GetDiscountRateInterestFreeQuotaResponse = OkxApiResponse<DiscountRateInterestFreeQuota>;
pub use get_estimated_price::{EstimatedPriceData, GetEstimatedPriceRequest};
pub type GetEstimatedPriceResponse = OkxApiResponse<EstimatedPriceData>;
pub use get_estimated_settlement_info::{
    EstimatedSettlementInfo, GetEstimatedSettlementInfoRequest,
};
pub type GetEstimatedSettlementInfoResponse = OkxApiResponse<EstimatedSettlementInfo>;
pub use get_funding_rate::{FundingRate, GetFundingRateRequest};
pub type GetFundingRateResponse = OkxApiResponse<FundingRate>;
pub use get_funding_rate_history::{FundingRateHistory, GetFundingRateHistoryRequest};
pub type GetFundingRateHistoryResponse = OkxApiResponse<FundingRateHistory>;
pub use get_history_index_candles::GetHistoryIndexCandlesRequest;
pub type GetHistoryIndexCandlesResponse = OkxApiResponse<IndexCandle>;
pub use get_history_mark_price_candles::{
    BarSize, GetHistoryMarkPriceCandlesRequest, MarkPriceCandle,
};
pub type GetHistoryMarkPriceCandlesResponse = OkxApiResponse<MarkPriceCandle>;
pub use get_index_candles::{GetIndexCandlesRequest, IndexCandle};
pub type GetIndexCandlesResponse = OkxApiResponse<IndexCandle>;
pub use get_index_components::{
    GetIndexComponentsRequest, IndexComponent, IndexComponentData,
};
pub type GetIndexComponentsResponse = OkxApiResponse<IndexComponentData>;
pub use get_index_tickers::{GetIndexTickersRequest, IndexTicker};
pub type GetIndexTickersResponse = OkxApiResponse<IndexTicker>;
pub use get_instrument_tick_bands::{
    GetInstrumentTickBandsRequest, InstrumentTickBandData, TickBand, TickBandInstrumentType,
};
pub type GetInstrumentTickBandsResponse = OkxApiResponse<InstrumentTickBandData>;
pub use get_instruments::{GetInstrumentsRequest, Instrument};
pub type GetInstrumentsResponse = OkxApiResponse<Instrument>;
pub use get_insurance_fund::{
    GetInsuranceFundRequest, InsuranceFundData, InsuranceFundDetail,
};
pub type GetInsuranceFundResponse = OkxApiResponse<InsuranceFundData>;
pub use get_interest_rate_loan_quota::{
    BasicInterestRate, GetInterestRateLoanQuotaRequest, InterestRateLoanQuotaData,
    RegularInterestRate, VipInterestRate,
};
pub type GetInterestRateLoanQuotaResponse = OkxApiResponse<InterestRateLoanQuotaData>;
pub use get_mark_price::{GetMarkPriceRequest, MarkPrice};
pub type GetMarkPriceResponse = OkxApiResponse<MarkPrice>;
pub use get_mark_price_candles::GetMarkPriceCandlesRequest;
pub type GetMarkPriceCandlesResponse = OkxApiResponse<MarkPriceCandle>;
pub use get_mark_price_candles_history::GetMarkPriceCandlesHistoryRequest;
pub type GetMarkPriceCandlesHistoryResponse = OkxApiResponse<MarkPriceCandle>;
pub use get_open_interest::{GetOpenInterestRequest, OpenInterest};
pub type GetOpenInterestResponse = OkxApiResponse<OpenInterest>;
pub use get_opt_summary::{GetOptSummaryRequest, OptSummary};
pub type GetOptSummaryResponse = OkxApiResponse<OptSummary>;
pub use get_position_tiers::{GetPositionTiersRequest, PositionTier};
pub type GetPositionTiersResponse = OkxApiResponse<PositionTier>;
pub use get_premium_history::{GetPremiumHistoryRequest, PremiumHistory};
pub type GetPremiumHistoryResponse = OkxApiResponse<PremiumHistory>;
pub use get_price_limit::{GetPriceLimitRequest, PriceLimit};
pub type GetPriceLimitResponse = OkxApiResponse<PriceLimit>;
pub use get_settlement_history::{
    GetSettlementHistoryRequest, SettlementDetail, SettlementHistory,
};
pub type GetSettlementHistoryResponse = OkxApiResponse<SettlementHistory>;
pub use get_time::TimeData;
pub type GetTimeResponse = OkxApiResponse<TimeData>;
pub use get_underlying::{GetUnderlyingRequest, UnderlyingData};
pub type GetUnderlyingResponse = OkxApiResponse<UnderlyingData>;
