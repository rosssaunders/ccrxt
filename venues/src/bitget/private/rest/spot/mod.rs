//! Spot trading endpoints for Bitget
//!
//! This module contains implementations for all Bitget spot trading endpoints
//! including order placement, cancellation, and trade information retrieval.

mod batch_cancel_orders;
mod batch_cancel_plan_orders;
mod batch_cancel_replace_orders;
mod batch_orders;
mod cancel_order;
mod cancel_plan_order;
mod cancel_replace_order;
mod cancel_symbol_order;
mod current_plan_order;
mod get_current_orders;
mod get_fills;
mod get_order_history;
mod get_order_info;
mod history_plan_order;
mod modify_plan_order;
mod place_order;
mod place_plan_order;
mod plan_sub_order;

// Re-export all endpoint types and functions for external use
pub use batch_cancel_orders::*;
pub use batch_cancel_plan_orders::*;
pub use batch_cancel_replace_orders::*;
pub use batch_orders::*;
pub use cancel_order::*;
pub use cancel_plan_order::*;
pub use cancel_replace_order::*;
pub use cancel_symbol_order::*;
pub use current_plan_order::*;
pub use get_current_orders::*;
pub use get_fills::*;
pub use get_order_history::*;
pub use get_order_info::*;
pub use history_plan_order::*;
pub use modify_plan_order::*;
pub use place_order::*;
pub use place_plan_order::*;
pub use plan_sub_order::*;
