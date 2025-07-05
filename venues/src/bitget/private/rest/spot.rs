//! Spot trading related exports
//!
//! This module re-exports all the spot trading related types for convenience.

// Re-export spot trading types
pub use super::cancel_order::{CancelOrderRequest, CancelOrderResponse};
pub use super::get_current_orders::{GetCurrentOrdersRequest, GetCurrentOrdersResponse};
pub use super::get_fills::{
    FillFeeDetail, FillInfo, GetFillsRequest, GetFillsResponse, TradeScope,
};
pub use super::get_order_history::{GetOrderHistoryRequest, GetOrderHistoryResponse};
pub use super::get_order_info::{
    EntryPointSource, FeeDetails, GetOrderInfoRequest, GetOrderInfoResponse, OrderSource,
};
pub use super::place_order::{Force, PlaceOrderRequest, PlaceOrderResponse, STPMode};

// Re-export common enums from the main bitget module
pub use crate::bitget::enums::{OrderSide, OrderStatus, OrderType};
