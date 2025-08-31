// Wallet endpoints as documented in docs/private_wallet.md

pub mod add_to_address_book;
pub mod create_deposit_address;
pub mod get_address_book;
pub mod get_current_deposit_address;
pub mod get_deposits;
pub mod get_transfers;
pub mod get_withdrawals;
pub mod remove_from_address_book;
pub mod submit_transfer_between_subaccounts;
pub mod submit_transfer_to_subaccount;
pub mod submit_transfer_to_user;
pub mod update_in_address_book;
pub mod withdraw;

// Re-export all types
pub use add_to_address_book::*;
pub use create_deposit_address::*;
pub use get_address_book::*;
pub use get_current_deposit_address::*;
pub use get_deposits::*;
pub use get_transfers::*;
pub use get_withdrawals::*;
pub use remove_from_address_book::*;
pub use submit_transfer_between_subaccounts::*;
pub use submit_transfer_to_subaccount::*;
pub use submit_transfer_to_user::*;
pub use update_in_address_book::*;
pub use withdraw::*;
