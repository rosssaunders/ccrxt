// Sub-account management endpoints

// Sub-account core endpoints
pub mod create_subaccount;
pub mod get_subaccount_list;

// API key management endpoints
pub mod create_apikey;
pub mod delete_apikey;
pub mod modify_apikey;
pub mod query_apikey;

// Balance and withdrawal endpoints
pub mod get_funding_balance;
pub mod get_max_withdrawal;
pub mod get_trading_balance;

// Transfer management endpoints
pub mod get_managed_transfer_history;
pub mod get_transfer_history;
pub mod manage_transfer;
pub mod set_transfer_permission;

// Custody trading endpoints
pub mod get_custody_subaccount_list;

pub use crate::okx::private_client::RestClient;
