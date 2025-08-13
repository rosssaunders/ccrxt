pub mod amend_order;
pub mod cancel_all_orders;
pub mod cancel_oco_order;
pub mod cancel_order;
pub mod cancel_order_list;
pub mod change_account_leverage;
pub mod change_account_settings;
pub mod client;
pub mod close_position;
pub mod create_oco_order;
pub mod create_order;
pub mod create_order_list;
pub mod create_subaccount_transfer;
pub mod create_withdrawal;
pub mod credentials;
pub mod get_account_settings;
pub mod get_accounts;
pub mod get_currency_networks;
pub mod get_deposit_address;
pub mod get_deposit_history;
pub mod get_fee_rate;
pub mod get_instrument_fee_rate;
pub mod get_open_orders;
pub mod get_order_detail;
pub mod get_order_history;
pub mod get_order_history_by_currency;
pub mod get_order_history_by_instrument;
pub mod get_order_list;
pub mod get_positions;
pub mod get_subaccount_balances;
pub mod get_trades;
pub mod get_transactions;
pub mod get_withdrawal_history;
pub mod user_balance;
pub mod user_balance_history;

// Staking endpoints
pub mod convert;
pub mod get_convert_history;
pub mod get_open_convert;
pub mod get_open_stake;
pub mod get_reward_history;
pub mod get_stake_history;
pub mod get_staking_instruments;
pub mod get_staking_position;
pub mod stake;
pub mod unstake;

pub use cancel_oco_order::CancelOcoOrderRequest;
pub use cancel_order_list::{
    CancelOrderListItem, CancelOrderListRequest, CancelOrderListResponse, OrderCancellationResult,
};
pub use client::RestClient;
pub use create_oco_order::CreateOcoOrderResponse;
pub use create_order_list::{
    CreateOrderListRequest, CreateOrderListResponse, OrderCreationResult, OrderListItem,
};
pub use credentials::Credentials;
pub use get_order_history_by_currency::{
    GetOrderHistoryByCurrencyRequest, GetOrderHistoryByCurrencyResponse,
    GetOrderHistoryByCurrencyWithContinuationResponse,
};
pub use get_order_list::{GetOrderListRequest, GetOrderListResponse, OrderDetails};
