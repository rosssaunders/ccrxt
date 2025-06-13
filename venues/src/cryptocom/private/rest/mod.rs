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
pub mod get_order_history;
pub mod get_trades;
pub mod get_transactions;
pub mod create_order_list;
pub mod cancel_order_list;
pub mod get_order_list;
pub mod create_order;
pub mod amend_order;
pub mod cancel_order;
pub mod cancel_all_orders;
pub mod close_position;
pub mod get_open_orders;
pub mod get_order_detail;
pub mod change_account_leverage;
pub mod change_account_settings;
pub mod get_account_settings;
pub mod get_fee_rate;
pub mod get_instrument_fee_rate;

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
pub use create_order_list::{CreateOrderListRequest, OrderListItem, CreateOrderListResponse, CreateOcoOrderResponse, OrderCreationResult};
pub use cancel_order_list::{CancelOrderListRequest, CancelOcoOrderRequest, CancelOrderListItem, CancelOrderListResponse, OrderCancellationResult};
pub use get_order_list::{GetOrderListRequest, GetOrderListResponse, OrderDetails};