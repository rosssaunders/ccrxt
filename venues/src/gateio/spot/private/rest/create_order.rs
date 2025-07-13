use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::spot::{OrderSide, OrderStatus, OrderType, StpMode, TimeInForce};

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
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#create-an-order>
    pub async fn create_order(
        &self,
        order: CreateOrderRequest,
    ) -> crate::gateio::spot::Result<Order> {
        self.post("/spot/orders", &order).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateio::spot::enums::{OrderSide, OrderType, TimeInForce};

    #[test]
    fn test_request_validation() {
        // Test valid order request
        let valid_order = CreateOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            order_type: OrderType::Limit,
            account: Some("spot".to_string()),
            side: OrderSide::Buy,
            amount: "0.001".to_string(),
            price: Some("30000".to_string()),
            time_in_force: Some(TimeInForce::GoodTillCanceled),
            text: Some("test_order".to_string()),
            iceberg: None,
            stp_mode: None,
        };

        // Verify required fields are present
        assert!(!valid_order.currency_pair.is_empty());
        assert!(matches!(valid_order.side, OrderSide::Buy));
        assert!(!valid_order.amount.is_empty());

        // Test amount parsing
        assert!(valid_order.amount.parse::<f64>().is_ok());
        if let Some(ref price) = valid_order.price {
            assert!(price.parse::<f64>().is_ok());
        }
    }
}
