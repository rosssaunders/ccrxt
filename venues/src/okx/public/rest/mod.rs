mod client;
mod convert_contract_coin;
mod get_index_components;
mod get_economic_calendar;
mod exchange_rate;
mod get_instruments;
mod get_history_mark_price_candles;
mod get_insurance_fund;
mod get_open_interest;
mod get_instrument_tick_bands;
mod get_index_tickers;
mod get_mark_price_candles;
mod get_mark_price_candles_history;
mod get_index_candles;
mod get_history_index_candles;
mod get_premium_history;
mod get_time;
mod get_underlying;
mod get_price_limit;

pub use client::RestClient;
pub use convert_contract_coin::{ConvertContractCoinRequest, ConvertContractCoinResponse, ConvertContractCoinData};

#[allow(unused_imports)]
pub use get_index_components::{
    GetIndexComponentsRequest, GetIndexComponentsResponse, IndexComponent, IndexComponentData,
};

#[allow(unused_imports)]
pub use get_economic_calendar::{EconomicEvent, GetEconomicCalendarRequest, GetEconomicCalendarResponse};
pub use exchange_rate::{ExchangeRate, ExchangeRateResponse};
pub use get_instruments::{GetInstrumentsRequest, GetInstrumentsResponse, Instrument};
pub use get_premium_history::{GetPremiumHistoryRequest, GetPremiumHistoryResponse, PremiumHistory};
pub use get_time::{GetTimeResponse, TimeData};
pub use get_history_mark_price_candles::{
    BarSize, GetHistoryMarkPriceCandlesRequest, GetHistoryMarkPriceCandlesResponse, MarkPriceCandle,
};
pub use get_insurance_fund::{
    GetInsuranceFundRequest, GetInsuranceFundResponse, InsuranceFundData, InsuranceFundDetail,
};
pub use get_open_interest::{GetOpenInterestRequest, GetOpenInterestResponse, OpenInterest};
pub use get_instrument_tick_bands::{GetInstrumentTickBandsRequest, GetInstrumentTickBandsResponse, InstrumentTickBandData, TickBand};
pub use get_index_tickers::{GetIndexTickersRequest, GetIndexTickersResponse, IndexTicker};
pub use get_mark_price_candles::{GetMarkPriceCandlesRequest, GetMarkPriceCandlesResponse};
pub use get_mark_price_candles_history::{GetMarkPriceCandlesHistoryRequest, GetMarkPriceCandlesHistoryResponse};
pub use get_index_candles::{GetIndexCandlesRequest, GetIndexCandlesResponse, IndexCandle};
pub use get_history_index_candles::{GetHistoryIndexCandlesRequest, GetHistoryIndexCandlesResponse};
pub use get_underlying::{GetUnderlyingRequest, GetUnderlyingResponse, UnderlyingData};
pub use get_price_limit::{GetPriceLimitRequest, GetPriceLimitResponse, PriceLimit};
