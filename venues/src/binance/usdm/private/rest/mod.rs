pub mod client;

pub use client::RestClient;

// Re-export placeholder modules for endpoints that will be implemented later
pub mod account {}
pub mod account_trades {}
pub mod batch_order {}
pub mod order {}
pub mod open_orders {}
pub mod position_risk {}
pub mod query_order {}
pub mod all_orders {}