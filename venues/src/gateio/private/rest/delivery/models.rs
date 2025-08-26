use serde::{Deserialize, Serialize};

use super::CandlestickInterval;

/// Request to create delivery order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeliveryOrderRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// Order size (positive for long, negative for short)
    pub size: i64,

    /// Order price (omit for market orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force (gtc, ioc, poc, fok)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,

    /// Text label for order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Reduce only order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Close position order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<bool>,

    /// Iceberg order amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<i64>,

    /// Auto size for closing position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_size: Option<String>,
}

/// Delivery order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryOrder {
    /// Order ID
    pub id: i64,

    /// User ID
    pub user: i64,

    /// Contract name
    pub contract: String,

    /// Creation timestamp
    pub create_time: f64,

    /// Finish timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<f64>,

    /// Finish reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_as: Option<String>,

    /// Order status
    pub status: String,

    /// Order size
    pub size: i64,

    /// Iceberg amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<i64>,

    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force
    pub tif: String,

    /// Left amount
    pub left: i64,

    /// Filled total
    pub fill_price: String,

    /// Order text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Reduce only
    pub reduce_only: bool,

    /// Close position
    pub close: bool,

    /// STP action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_act: Option<String>,
}

/// Request parameters for listing delivery orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListDeliveryOrdersRequest {
    /// Settlement currency
    pub settle: String,

    /// Order status (open, finished)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Start time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of records to return (1-1000, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Request parameters for delivery positions
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryPositionsRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Maximum number of records to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Delivery position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPosition {
    /// User ID
    pub user: i64,

    /// Contract name
    pub contract: String,

    /// Position size (positive for long, negative for short)
    pub size: i64,

    /// Average entry price
    pub entry_price: String,

    /// Mark price
    pub mark_price: String,

    /// Realized PnL
    pub realised_pnl: String,

    /// Unrealized PnL
    pub unrealised_pnl: String,

    /// Position margin
    pub margin: String,

    /// Leverage
    pub leverage: String,

    /// Risk limit
    pub risk_limit: String,

    /// Liquidation price
    pub liq_price: String,

    /// Bankruptcy price
    pub bankruptcy_price: String,

    /// Cross margin leverage limit
    pub cross_leverage_limit: String,

    /// Position mode
    pub mode: String,

    /// Last update timestamp
    pub update_time: i64,
}

/// Request to set delivery leverage
#[derive(Debug, Clone, Serialize)]
pub struct SetDeliveryLeverageRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// Leverage value
    pub leverage: String,

    /// Cross margin leverage limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_leverage_limit: Option<String>,
}

/// Delivery leverage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryLeverageResponse {
    /// Leverage value
    pub leverage: String,

    /// Cross margin leverage limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_leverage_limit: Option<String>,
}

/// Request to update delivery position margin
#[derive(Debug, Clone, Serialize)]
pub struct UpdateDeliveryPositionMarginRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// Change amount (positive to add, negative to remove)
    pub change: String,
}

/// Delivery position margin response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPositionMarginResponse {
    /// New margin amount
    pub margin: String,
}

/// Request to update delivery risk limit
#[derive(Debug, Clone, Serialize)]
pub struct UpdateDeliveryRiskLimitRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// Risk limit value
    pub risk_limit: String,
}

/// Delivery risk limit response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryRiskLimitResponse {
    /// Risk limit value
    pub risk_limit: String,
}

/// Request parameters for delivery candlesticks
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeliveryCandlesticksRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// Interval time between data points
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<CandlestickInterval>,

    /// Start time for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of records to return (1-1000, default 100)  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Delivery candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryCandlestick {
    /// Unix timestamp in seconds
    pub t: i64,

    /// Trading volume (in quote currency)
    pub v: Option<i64>,

    /// Close price
    pub c: String,

    /// Highest price
    pub h: String,

    /// Lowest price
    pub l: String,

    /// Open price
    pub o: String,

    /// Trading volume (in base currency)
    pub sum: Option<String>,
}
