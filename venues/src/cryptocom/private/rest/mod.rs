pub mod client;
pub mod user_balance;
pub mod user_balance_history;
pub mod get_accounts;
pub mod create_subaccount_transfer;
pub mod get_subaccount_balances;
pub mod get_positions;
pub mod create_order_list;
pub mod cancel_order_list;
pub mod get_order_list;

#[cfg(test)]
mod integration_tests;

pub use client::RestClient;
pub use create_order_list::{CreateOrderListRequest, OrderListItem, CreateOrderListResponse, CreateOcoOrderResponse, OrderCreationResult};
pub use cancel_order_list::{CancelOrderListRequest, CancelOcoOrderRequest, CancelOrderListItem, CancelOrderListResponse, OrderCancellationResult};
pub use get_order_list::{GetOrderListRequest, GetOrderListResponse, OrderDetails};