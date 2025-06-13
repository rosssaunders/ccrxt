pub mod client;
pub mod user_balance;
pub mod user_balance_history;
pub mod get_accounts;
pub mod create_subaccount_transfer;
pub mod get_subaccount_balances;
pub mod get_positions;
pub mod create_withdrawal;
pub mod get_currency_networks;
pub mod get_deposit_address;
pub mod get_deposit_history;
pub mod get_withdrawal_history;

#[cfg(test)]
mod integration_tests;

pub use client::RestClient;