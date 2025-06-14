mod client;
mod exchange_rate;
mod get_instruments;
mod get_history_index_candles;
mod get_premium_history;

pub use client::RestClient;
pub use exchange_rate::{ExchangeRate, ExchangeRateResponse};
pub use get_instruments::{GetInstrumentsRequest, GetInstrumentsResponse, Instrument};
pub use get_history_index_candles::{GetHistoryIndexCandlesRequest, GetHistoryIndexCandlesResponse, IndexCandle};
pub use get_premium_history::{GetPremiumHistoryRequest, GetPremiumHistoryResponse, PremiumHistory};
