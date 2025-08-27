//! Trading Account REST API endpoints
//!
//! The trading account API allows you to manage your trading positions, margins,
//! leverage, and other trading-related account settings.
pub mod account_level_switch_preset;
pub mod activate_option;
pub mod adjust_position_margin_balance;
pub mod bills_history_archive;
pub mod get_account_balance;
pub mod get_account_config;
pub mod get_account_instruments;
pub mod get_account_position_risk;
pub mod get_account_switch_precheck;
pub mod get_adjust_leverage_info;
pub mod get_bills;
pub mod get_bills_archive;
pub mod get_collateral_assets;
pub mod get_greeks;
pub mod get_interest_accrued;
pub mod get_interest_limits;
pub mod get_interest_rate;
pub mod get_leverage_info;
pub mod get_max_avail_size;
pub mod get_max_loan;
pub mod get_max_size;
pub mod get_max_withdrawal;
pub mod get_mmp_config;
pub mod get_move_positions_history;
pub mod get_position_tiers;
pub mod get_positions;
pub mod get_positions_history;
pub mod get_risk_state;
pub mod get_trade_fee;
pub mod mmp_reset;
pub mod move_positions;
pub mod position_builder;
pub mod position_builder_graph;
pub mod set_account_level;
pub mod set_auto_earn;
pub mod set_collateral_assets;
pub mod set_greeks;
pub mod set_isolated_mode;
pub mod set_leverage;
pub mod set_mmp_config;
pub mod set_position_mode;
pub mod set_risk_offset_amt;

pub use crate::okx::private_client::RestClient;
