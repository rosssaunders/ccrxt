// Sub-account management endpoints module for BingX
// Documented in venues/src/bingx/docs/sub_account.md

pub mod authorize_sub_account_transfer;
pub mod batch_sub_account_assets;
pub mod create_sub_account;
pub mod create_sub_account_api_key;
pub mod create_sub_account_deposit_address;
pub mod delete_sub_account_api_key;
pub mod edit_sub_account_api_key;
pub mod freeze_sub_account;
pub mod get_sub_account_assets;
pub mod get_sub_account_deposit_address;
pub mod get_sub_account_deposit_records;
pub mod get_sub_account_internal_transfer_records;
pub mod get_sub_account_list;
pub mod get_sub_account_transfer_history;
pub mod get_transferable_amounts;
pub mod get_uid;
pub mod main_account_internal_transfer;
pub mod query_api_key;
pub mod sub_account_transfer;
pub mod transfer_sub_account_assets;
