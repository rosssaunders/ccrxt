use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const ORDERS_ENDPOINT: &str = "/loan/multi_collateral/orders";

/// Query params to list multi-collateral orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListOrdersQuery {
    /// Page number for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Records per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Sorting options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,

    /// Order type filter: "current" or "fixed"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
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
    /// Query Multi-Currency Collateral Orders
    ///
    /// Retrieves a paginated list of multi-currency collateral loan orders with optional filtering.
    /// Supports filtering by order type and flexible sorting with comprehensive pagination support.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#query-multi-currency-collateral-orders)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `query` - Optional query parameters for pagination, sorting, and order type filtering
    ///
    /// # Returns
    /// List of order summaries containing basic order information and current status
    pub async fn list_multi_collateral_orders(
        &self,
        query: Option<ListOrdersQuery>,
    ) -> RestResult<Vec<OrderSummary>> {
        self.send_get_request(ORDERS_ENDPOINT, query.as_ref()).await
    }
}
