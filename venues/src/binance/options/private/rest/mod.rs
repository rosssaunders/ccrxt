//! Private REST endpoints module for Binance Options API
//!
//! This module provides access to all private endpoints for the Binance Options API,
//! including account information, order management, position queries, market maker
//! functionality, and block trading.

pub mod client;

// Account endpoints
pub mod account;
pub mod account_funding_flow;
pub mod income_download_initiate;
pub mod income_download_status;
pub mod margin_account;

// Trading endpoints
pub mod batch_cancel_orders;
pub mod batch_orders;
pub mod cancel_all_orders_by_symbol;
pub mod cancel_all_orders_by_underlying;
pub mod cancel_order;
pub mod exercise_record;
pub mod history_orders;
pub mod open_orders;
pub mod order;
pub mod position;
pub mod query_order;
pub mod user_trades;

// Market maker endpoints
pub mod countdown_cancel;
pub mod get_mmp_config;
pub mod reset_mmp;
pub mod set_mmp_config;

// Block trade endpoints
pub mod block_trade_execution;
pub mod block_trade_orders;
pub mod block_user_trades;
pub mod cancel_block_trade;
pub mod create_block_trade;
pub mod execute_block_trade;
pub mod extend_block_trade;

// Re-export commonly used block trade types
pub use block_trade_execution::QueryBlockTradeExecutionRequest;
pub use block_trade_orders::QueryBlockTradeOrdersRequest;
pub use block_user_trades::QueryBlockUserTradesRequest;
pub use cancel_block_trade::CancelBlockTradeRequest;
pub use client::RestClient;
pub use crate::binance::shared::credentials::Credentials;
pub use create_block_trade::{BlockTradeOrderResponse, CreateBlockTradeRequest};
pub use execute_block_trade::{BlockTradeExecution, ExecuteBlockTradeRequest};
pub use extend_block_trade::ExtendBlockTradeRequest;
// Re-export commonly used MMP types
pub use get_mmp_config::{GetMmpConfigRequest, MmpConfigResponse};
pub use reset_mmp::ResetMmpRequest;
pub use set_mmp_config::SetMmpConfigRequest;
