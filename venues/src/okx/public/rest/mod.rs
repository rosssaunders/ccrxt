mod client;
mod get_funding_rate_history;
mod get_instruments;
mod get_premium_history;

pub use client::RestClient;
pub use get_funding_rate_history::{GetFundingRateHistoryRequest, GetFundingRateHistoryResponse, FundingRateHistory};
pub use get_instruments::{GetInstrumentsRequest, GetInstrumentsResponse, Instrument};
pub use get_premium_history::{GetPremiumHistoryRequest, GetPremiumHistoryResponse, PremiumHistory};
