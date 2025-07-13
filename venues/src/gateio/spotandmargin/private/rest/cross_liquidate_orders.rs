use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request to create cross liquidate orders
#[derive(Debug, Clone, Serialize)]
pub struct CrossLiquidateOrdersRequest {
    /// Currency pair to liquidate
    pub currency_pair: String,

    /// Liquidation type (close_position, auto_borrow, auto_repay)
    #[serde(rename = "type")]
    pub liquidation_type: String,

    /// Client order ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Cross liquidate order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossLiquidateOrder {
    /// Order ID
    pub id: String,

    /// Currency pair
    pub currency_pair: String,

    /// Order status
    pub status: String,

    /// Account mode (cross_margin)
    pub account: String,

    /// Order side (buy or sell)
    pub side: String,

    /// Order amount
    pub amount: String,

    /// Order price
    pub price: String,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: String,

    /// Time in force
    pub time_in_force: String,

    /// Filled amount
    pub filled_amount: String,

    /// Left amount
    pub left: String,

    /// Average fill price
    pub avg_deal_price: String,

    /// Order fee
    pub fee: String,

    /// Fee currency
    pub fee_currency: String,

    /// Points used for fee
    pub points_fee: String,

    /// GT discount fee
    pub gt_fee: String,

    /// Create time timestamp
    pub create_time: String,

    /// Update time timestamp
    pub update_time: String,

    /// Client order id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl RestClient {
    /// Create cross liquidate orders
    ///
    /// This endpoint creates orders for cross margin liquidation.
    /// Used to automatically close positions or handle margin calls.
    pub async fn cross_liquidate_orders(
        &self,
        request: CrossLiquidateOrdersRequest,
    ) -> crate::gateio::spotandmargin::Result<CrossLiquidateOrder> {
        self.post("/spot/cross_liquidate_orders", &request).await
    }
}
