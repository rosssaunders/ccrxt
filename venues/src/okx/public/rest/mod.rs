mod client;
mod get_economic_calendar;
mod exchange_rate;
mod get_instruments;
mod get_mark_price_candles;
mod get_mark_price_candles_history;
mod get_index_candles;
mod get_history_index_candles;
mod get_premium_history;

pub use client::RestClient;

#[allow(unused_imports)]
pub use get_economic_calendar::{EconomicEvent, GetEconomicCalendarRequest, GetEconomicCalendarResponse};
pub use exchange_rate::{ExchangeRate, ExchangeRateResponse};
pub use get_instruments::{GetInstrumentsRequest, GetInstrumentsResponse, Instrument};
pub use get_mark_price_candles::{GetMarkPriceCandlesRequest, GetMarkPriceCandlesResponse};
pub use get_mark_price_candles_history::{GetMarkPriceCandlesHistoryRequest, GetMarkPriceCandlesHistoryResponse};
pub use get_index_candles::{GetIndexCandlesRequest, GetIndexCandlesResponse, IndexCandle};
pub use get_history_index_candles::{GetHistoryIndexCandlesRequest, GetHistoryIndexCandlesResponse, IndexCandle};
pub use get_premium_history::{GetPremiumHistoryRequest, GetPremiumHistoryResponse, PremiumHistory};
