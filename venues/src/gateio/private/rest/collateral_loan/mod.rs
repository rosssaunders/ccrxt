pub mod collaterals_add;
pub mod collaterals_list;
pub mod currencies;
pub mod ltv;
pub mod order_get;
pub mod orders_create;
pub mod orders_list;
pub mod repay;
pub mod repay_records;
pub mod total_amount;

pub use crate::gateio::{PrivateRestClient as RestClient, RestResult};
