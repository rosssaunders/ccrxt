//! Spot trading endpoints for Bitget
//!
//! This module contains implementations for all Bitget spot trading endpoints
//! including order placement, cancellation, and trade information retrieval.

mod cancel_order;
mod get_current_orders;
mod get_fills;
mod get_order_history;
mod get_order_info;
mod place_order;

// Re-export all endpoint types and functions
pub use cancel_order::{CancelOrderRequest, CancelOrderResponse};
pub use get_current_orders::{GetCurrentOrdersRequest, GetCurrentOrdersResponse, OrderInfo};
pub use get_fills::{FillInfo, GetFillsRequest, GetFillsResponse};
pub use get_order_history::{GetOrderHistoryRequest, GetOrderHistoryResponse, OrderHistoryInfo};
pub use get_order_info::{GetOrderInfoRequest, GetOrderInfoResponse};
pub use place_order::{PlaceOrderRequest, PlaceOrderResponse};
