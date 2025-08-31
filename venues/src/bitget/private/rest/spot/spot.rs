//! Spot trading related exports
//!
//! This module re-exports all the spot trading related types for convenience.

// Re-export spot trading types
pub use super::{
    cancel_order::{CancelOrderRequest, CancelOrderResponse},
    get_current_orders::{GetCurrentOrdersRequest, GetCurrentOrdersResponse},
    get_fills::{FillFeeDetail, FillInfo, GetFillsRequest, GetFillsResponse, TradeScope},
    get_order_history::{GetOrderHistoryRequest, GetOrderHistoryResponse},
    get_order_info::{
        EntryPointSource, FeeDetails, GetOrderInfoRequest, GetOrderInfoResponse, OrderSource,
    },
    place_order::{Force, PlaceOrderRequest, PlaceOrderResponse, STPMode},
};
// Re-export common enums from the main bitget module
pub use crate::bitget::enums::{OrderSide, OrderStatus, OrderType};
