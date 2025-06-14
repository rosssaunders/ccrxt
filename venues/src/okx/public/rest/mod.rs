mod client;
mod get_economic_calendar;
mod get_instruments;

pub use client::RestClient;
#[allow(unused_imports)] // Public API exports
pub use get_economic_calendar::{EconomicEvent, GetEconomicCalendarRequest, GetEconomicCalendarResponse};
pub use get_instruments::{GetInstrumentsRequest, GetInstrumentsResponse, Instrument};
