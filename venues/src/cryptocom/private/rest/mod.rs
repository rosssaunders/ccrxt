pub mod client;
pub mod user_balance;
pub mod user_balance_history;
pub mod get_accounts;
pub mod create_subaccount_transfer;
pub mod get_subaccount_balances;
pub mod get_positions;

// Staking endpoints
pub mod get_staking_instruments;
pub mod get_staking_position;
pub mod stake;
pub mod unstake;
pub mod get_open_stake;
pub mod get_stake_history;
pub mod get_reward_history;
pub mod convert;
pub mod get_open_convert;
pub mod get_convert_history;

#[cfg(test)]
mod integration_tests;

pub use client::RestClient;