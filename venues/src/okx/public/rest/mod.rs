mod client;
mod exchange_rate;
mod get_instruments;

pub use client::RestClient;
pub use exchange_rate::{ExchangeRate, ExchangeRateResponse};
pub use get_instruments::{GetInstrumentsRequest, GetInstrumentsResponse, Instrument};
