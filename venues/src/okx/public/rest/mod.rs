mod client;
mod get_instruments;
mod get_interest_rate_loan_quota;
mod get_premium_history;

pub use client::RestClient;
pub use get_instruments::{GetInstrumentsRequest, GetInstrumentsResponse, Instrument};
pub use get_interest_rate_loan_quota::{
    BasicInterestRate, GetInterestRateLoanQuotaRequest, GetInterestRateLoanQuotaResponse,
    InterestRateLoanQuotaData, RegularInterestRate, VipInterestRate,
};
pub use get_premium_history::{GetPremiumHistoryRequest, GetPremiumHistoryResponse, PremiumHistory};
