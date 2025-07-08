pub mod client;
pub mod get_wallet_balance;

// Trade endpoints
pub mod amend_order;
pub mod batch_amend_orders;
pub mod batch_cancel_orders;
pub mod batch_create_orders;
pub mod cancel_all_orders;
pub mod cancel_order;
pub mod create_order;
pub mod get_execution_list;
pub mod get_open_orders;
pub mod get_order_history;
pub mod spot_borrow_check;

// Position endpoints
pub mod get_position_info;

pub use client::RestClient;
pub use get_wallet_balance::{
    BalanceData, GetWalletBalanceRequest, GetWalletBalanceResponse, WalletBalance,
};

// Trade and position endpoint modules are available as sub-modules
// Example usage: bybit::private::create_order::CreateOrderRequest
