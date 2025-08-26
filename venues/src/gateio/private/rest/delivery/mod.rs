// Models
pub mod models;

// Re-export key components from shared modules
pub use models::{
    CreateDeliveryOrderRequest, DeliveryCandlestick, DeliveryCandlesticksRequest,
    DeliveryLeverageResponse, DeliveryOrder, DeliveryPosition, DeliveryPositionMarginResponse,
    DeliveryPositionsRequest, DeliveryRiskLimitResponse, ListDeliveryOrdersRequest,
    SetDeliveryLeverageRequest, UpdateDeliveryPositionMarginRequest,
    UpdateDeliveryRiskLimitRequest,
};

pub mod account_book;
pub mod accounts;
pub mod liquidations;
pub mod order_cancel;
pub mod order_create;
pub mod order_list;
pub mod position_close_history;
pub mod position_leverage;
pub mod position_list;
pub mod position_margin;
pub mod position_risk_limit;
pub mod price_orders;
pub mod settlements;
pub mod trades;

pub use crate::gateio::PrivateRestClient as RestClient;
pub use crate::gateio::RestResult;
pub use crate::gateio::enums::CandlestickInterval;
