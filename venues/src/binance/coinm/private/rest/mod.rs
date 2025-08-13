// Private REST endpoints module for Binance Coin-M

pub mod account;
pub mod account_trades;
pub mod all_orders;
pub mod auto_cancel_all_open_orders;
pub mod batch_order;
pub mod cancel_all_open_orders;
pub mod cancel_order;
pub mod change_initial_leverage;
pub mod change_margin_type;
pub mod change_position_mode;
pub mod client;
pub mod create_listen_key;
pub mod delete_listen_key;
pub mod extend_listen_key;
pub mod force_orders;
pub mod futures_account_balance;
pub mod get_current_position_mode;
pub mod get_transaction_history_download_id;
pub mod get_transaction_history_download_link;
pub mod income_history;
pub mod modify_isolated_position_margin;
pub mod modify_multiple_orders;
pub mod modify_order;
pub mod notional_brackets;
pub mod open_orders;
pub mod order;
pub mod order_modify_history;
pub mod position_adl_quantile;
pub mod position_margin_change_history;
pub mod position_risk;
pub mod query_current_open_order;
pub mod query_order;
pub mod user_commission_rate;

pub use client::RestClient;

pub use crate::binance::shared::credentials::Credentials;
