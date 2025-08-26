// Delegate to centralized PrivateRestClient at the gateio root.
pub use crate::gateio::PrivateRestClient as RestClient;
pub use crate::gateio::RestResult;
pub mod eth2_swap;
pub mod eth2_rate_records;
pub mod dual_investment_plan;
pub mod dual_orders;
pub mod place_dual_order;
pub mod structured_products;
pub mod structured_orders;
pub mod place_structured_order;
pub mod staking_coins;
pub mod staking_swap;
