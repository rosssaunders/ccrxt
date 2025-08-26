use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const ORDERS_ENDPOINT: &str = "/loan/multi_collateral/orders";

/// Query params to list multi-collateral orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListOrdersQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_currency: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Order summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSummary {
    pub id: String,

    pub borrow_currency: String,

    pub borrow_amount: String,

    pub status: String,
}

impl RestClient {
    /// List multi-collateral orders
    pub async fn list_multi_collateral_orders(
        &self,
        query: ListOrdersQuery,
    ) -> RestResult<Vec<OrderSummary>> {
        self.send_get_request(ORDERS_ENDPOINT, Some(&query)).await
    }
}
