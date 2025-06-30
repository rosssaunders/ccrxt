use crate::gateio::{OrderSide, OrderStatus, OrderType, StpMode, TimeInForce};
use serde::{Deserialize, Serialize};

use super::RestClient;

/// Order creation request
#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderRequest {
    /// Currency pair
    pub currency_pair: String,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Account type (spot, margin, cross_margin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Order side
    pub side: OrderSide,

    /// Order amount
    pub amount: String,

    /// Order price (required for limit orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Iceberg amount (0 for normal orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<String>,

    /// Self-trade prevention mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<StpMode>,

    /// Client order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl CreateOrderRequest {
    /// Create a new limit order
    pub fn limit(currency_pair: String, side: OrderSide, amount: String, price: String) -> Self {
        Self {
            currency_pair,
            order_type: OrderType::Limit,
            account: None,
            side,
            amount,
            price: Some(price),
            time_in_force: Some(TimeInForce::GoodTillCanceled),
            iceberg: None,
            stp_mode: None,
            text: None,
        }
    }

    /// Create a new market order
    pub fn market(currency_pair: String, side: OrderSide, amount: String) -> Self {
        Self {
            currency_pair,
            order_type: OrderType::Market,
            account: None,
            side,
            amount,
            price: None,
            time_in_force: None,
            iceberg: Some("0".to_string()), // Required for market orders
            stp_mode: None,
            text: None,
        }
    }

    /// Set the account type
    pub fn with_account(mut self, account: String) -> Self {
        self.account = Some(account);
        self
    }

    /// Set the time in force
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    /// Set the self-trade prevention mode
    pub fn with_stp_mode(mut self, stp: StpMode) -> Self {
        self.stp_mode = Some(stp);
        self
    }

    /// Set the client order ID
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }
}

/// Order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Order ID
    pub id: String,

    /// User defined text
    pub text: String,

    /// Whether to cancel remaining orders, only used in batch orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend_text: Option<String>,

    /// Order creation time
    pub create_time: String,

    /// Order update time
    pub update_time: String,

    /// Order status
    pub status: OrderStatus,

    /// Currency pair
    pub currency_pair: String,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Account type
    pub account: String,

    /// Order side
    pub side: OrderSide,

    /// Order amount
    pub amount: String,

    /// Order price
    pub price: String,

    /// Time in force
    pub time_in_force: TimeInForce,

    /// Iceberg amount
    pub iceberg: String,

    /// Amount to display
    pub left: String,

    /// Executed amount
    pub filled_amount: String,

    /// Executed value in quote currency
    pub fill_price: String,

    /// Fee paid
    pub fee: String,

    /// Fee currency
    pub fee_currency: String,

    /// Point fee
    pub point_fee: String,

    /// GT fee
    pub gt_fee: String,

    /// GT discount
    pub gt_discount: bool,

    /// Rebated fee
    pub rebated_fee: String,

    /// Rebated fee currency
    pub rebated_fee_currency: String,

    /// Self-trade prevention mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<StpMode>,

    /// Self-trade prevention triggered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_act: Option<String>,
}

impl RestClient {
    /// Create a new order
    ///
    /// This endpoint creates a new spot order.
    pub async fn create_order(&self, order: CreateOrderRequest) -> crate::gateio::Result<Order> {
        self.post("/spot/orders", &order).await
    }
}
