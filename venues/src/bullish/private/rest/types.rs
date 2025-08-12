use serde::Deserialize;

use crate::bullish::enums::*;

/// Order details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// Unique order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Market symbol
    pub symbol: String,

    /// Order price
    pub price: String,

    /// Stop price
    #[serde(rename = "stopPrice")]
    pub stop_price: Option<String>,

    /// Average fill price
    #[serde(rename = "averageFillPrice")]
    pub average_fill_price: Option<String>,

    /// Whether borrowing is allowed
    #[serde(rename = "allowBorrow")]
    pub allow_borrow: bool,

    /// Order quantity
    pub quantity: String,

    /// Filled quantity
    #[serde(rename = "quantityFilled")]
    pub quantity_filled: String,

    /// Quote amount
    #[serde(rename = "quoteAmount")]
    pub quote_amount: String,

    /// Base fee
    #[serde(rename = "baseFee")]
    pub base_fee: String,

    /// Quote fee
    #[serde(rename = "quoteFee")]
    pub quote_fee: String,

    /// Quantity borrowed (base asset), when borrowing occurred
    #[serde(rename = "borrowedBaseQuantity")]
    pub borrowed_base_quantity: Option<String>,

    /// Quantity borrowed (quote asset), when borrowing occurred
    #[serde(rename = "borrowedQuoteQuantity")]
    pub borrowed_quote_quantity: Option<String>,

    /// Whether this is a liquidation order
    #[serde(rename = "isLiquidation")]
    pub is_liquidation: bool,

    /// Order side
    pub side: OrderSide,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Time in force
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    /// Order status
    pub status: OrderStatus,

    /// Status reason
    #[serde(rename = "statusReason")]
    pub status_reason: String,

    /// Status reason code
    #[serde(rename = "statusReasonCode")]
    pub status_reason_code: String,

    /// Creation timestamp
    #[serde(rename = "createdAtTimestamp")]
    pub created_at_timestamp: String,

    /// Creation datetime
    #[serde(rename = "createdAtDatetime")]
    pub created_at_datetime: String,
}
