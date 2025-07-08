use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request to amend an order in batch
#[derive(Debug, Clone, Serialize)]
pub struct AmendOrderRequest {
    /// Order ID to amend
    pub id: String,

    /// New price (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// New amount (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,

    /// Currency pair
    pub currency_pair: String,

    /// Account mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

/// Request to amend multiple orders in batch
#[derive(Debug, Clone, Serialize)]
pub struct AmendBatchOrdersRequest {
    /// List of orders to amend
    pub orders: Vec<AmendOrderRequest>,
}

/// Amended order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmendedOrder {
    /// Order ID
    pub id: String,

    /// Currency pair
    pub currency_pair: String,

    /// Order status
    pub status: String,

    /// Account mode
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

    /// Amendment succeeded
    pub succeeded: bool,

    /// Error message if amendment failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Error code if amendment failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl RestClient {
    /// Amend multiple orders in batch
    ///
    /// This endpoint allows amending price and/or amount for multiple orders at once.
    /// Returns information about each order amendment attempt.
    pub async fn amend_batch_orders(
        &self,
        request: AmendBatchOrdersRequest,
    ) -> crate::gateio::Result<Vec<AmendedOrder>> {
        self.post("/spot/amend_batch_orders", &request).await
    }
}
