// Existing modules
mod asset_transfer;
mod cancel_multiple_orders;
mod cancel_order;
mod client;
mod currency_config;
mod deposit_address;
mod deposit_records;
mod get_all_account_balance;
mod get_asset_transfer;
mod get_balances;
mod get_commission_rate;
mod get_fund_balance;
mod get_open_orders;
mod get_order_history;
mod get_trade_history;
mod get_uid;
mod internal_transfer_apply;
mod internal_transfer_records;
mod place_order;
mod post_transfer;
mod query_order;
mod query_transferable_coins;
mod withdraw;
mod withdraw_records;

// Trading endpoints
mod cancel_all_after;
mod cancel_all_orders;
mod cancel_replace_order;
mod place_multiple_orders;

// OCO endpoints
mod cancel_oco_order;
mod create_oco_order;
mod get_oco_order_history;
mod get_open_oco_orders;
mod query_oco_order;

// Sub-account endpoints
mod authorize_sub_account_transfer;
mod batch_sub_account_assets;
mod create_sub_account;
mod create_sub_account_api_key;
mod delete_sub_account_api_key;
mod edit_sub_account_api_key;
mod freeze_sub_account;
mod get_sub_account_assets;
mod get_sub_account_list;
mod get_sub_account_transfer_history;
mod query_api_key;
mod sub_account_transfer;

// Sub-account exports
// Trading exports
// OCO exports
pub use client::RestClient;
// Wallet/Fund exports
pub use get_balances::{Balance, GetBalancesRequest, GetBalancesResponse};
