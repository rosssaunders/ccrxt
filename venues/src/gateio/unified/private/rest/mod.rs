pub mod accounts;

// Borrowable-related modules
pub mod borrowable;
pub mod get_unified_borrowable;
pub mod get_batch_borrowable;

pub mod client;

// Discount tiers-related modules
pub mod discount_tiers;
pub mod get_currency_discount_tiers;
pub mod get_loan_margin_tiers;

// Leverage-related modules
pub mod leverage;
pub mod get_leverage_user_currency_config;
pub mod get_leverage_user_currency_setting;
pub mod set_leverage_user_currency_setting;

// Loan-related modules
pub mod loan;
pub mod borrow_or_repay;
pub mod borrow;
pub mod repay;
pub mod repay_all;
pub mod list_loans;
pub mod get_loan_history;
pub mod get_all_loan_history;
pub mod get_max_borrowable;
pub mod list_loan_interest_records;

// Unified margin-related modules
pub mod unified_margin;
pub mod get_unified_margin_currency_pairs;
pub mod get_unified_margin_currency_pair;
pub mod get_unified_margin_loans;
pub mod create_unified_margin_loan;
pub mod get_unified_margin_loan_records;
pub mod get_unified_margin_borrowable;
pub mod get_unified_margin_interest_records;

pub mod portfolio_calculator;
pub mod risk_management;

// Transferable-related modules
pub mod transferable;
pub mod get_unified_transferable;
pub mod get_unified_transferables;

// Unified mode-related modules
pub mod unified_mode;
pub mod get_unified_mode;
pub mod set_unified_mode;
pub mod get_unified_currencies;

pub use client::RestClient;
