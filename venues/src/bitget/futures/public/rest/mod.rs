mod vip_fee_rate;
mod ticker;
mod interest_rate_history;
mod exchange_rate;
mod market_depth;
mod recent_transactions;
mod candlestick;
mod open_interest;
mod next_funding_time;
mod current_funding_rate;
mod contract_config;

// Re-export all public endpoints
pub use vip_fee_rate::{VipFeeRate, GetVipFeeRateRequest, GetVipFeeRateResponse};
pub use ticker::{
    FuturesTicker, GetTickerRequest, GetTickerResponse, 
    GetAllTickersRequest, GetAllTickersResponse
};
pub use interest_rate_history::*;
pub use exchange_rate::*;
pub use market_depth::*;
pub use recent_transactions::*;
pub use candlestick::*;
pub use open_interest::*;
pub use next_funding_time::*;
pub use current_funding_rate::*;
pub use contract_config::*;
