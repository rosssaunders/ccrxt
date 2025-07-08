use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for listing open orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListOpenOrdersRequest {
    /// Currency pair
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Page number (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records to return (1-100, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Order side filter (buy or sell)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,

    /// Account mode (spot, margin, cross_margin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

/// Open order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenOrder {
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

    /// Iceberg order amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<String>,

    /// Auto borrow enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow: Option<bool>,

    /// Auto repay enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repay: Option<bool>,

    /// STP action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_act: Option<String>,

    /// Finish as
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_as: Option<String>,
}

impl RestClient {
    /// List open orders
    ///
    /// This endpoint returns all open (active) orders for the authenticated user.
    /// You can filter by currency pair, side, and account type.
    pub async fn list_open_orders(
        &self,
        params: ListOpenOrdersRequest,
    ) -> crate::gateio::Result<Vec<OpenOrder>> {
        self.get_with_query("/spot/open_orders", &params).await
    }
}
