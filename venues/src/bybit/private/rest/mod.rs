pub mod client;
pub mod get_wallet_balance;

pub use client::RestClient;
pub use get_wallet_balance::{
    GetWalletBalanceRequest, GetWalletBalanceResponse, WalletBalance, BalanceData,
};