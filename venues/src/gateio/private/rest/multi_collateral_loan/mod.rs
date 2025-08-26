pub mod currencies;
pub mod currency_quota;
pub mod current_rate;
pub mod fixed_rate;
pub mod ltv;
pub mod mortgage_add;
pub mod mortgage_list;
pub mod order_get;
pub mod orders_create;
pub mod orders_list;
pub mod repay;
pub mod repay_records;

// Use the centralized root PrivateRestClient for this module's endpoints.
pub use crate::gateio::PrivateRestClient as RestClient;
pub use crate::gateio::RestResult;
