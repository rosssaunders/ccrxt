pub mod chart;
pub mod currencies;
pub mod interest_records;
pub mod interest_status;
pub mod interests;
pub mod lends;
pub mod loan_records;
pub mod modify_loan;
pub mod products;
pub mod rate;
pub mod redeem;
pub mod subscribe;

pub use crate::gateio::{PrivateRestClient as RestClient, RestResult};
