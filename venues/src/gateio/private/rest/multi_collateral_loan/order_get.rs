use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const ORDERS_ENDPOINT: &str = "/loan/multi_collateral/orders";

/// Multi-collateral order details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDetail {
    pub id: String,

    pub borrow_currency: String,

    pub borrow_amount: String,

    pub status: String,

    /// Outstanding principal
    pub outstanding: Option<String>,

    /// Accrued interest
    pub interest: Option<String>,
}

impl RestClient {
    /// Get Multi-Currency Collateral Order Details
    ///
    /// Retrieves comprehensive details for a specific multi-currency collateral loan order,
    /// including current status, outstanding amounts, accrued interest, and collateral information.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#get-multi-currency-collateral-order)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `order_id` - Unique identifier of the multi-collateral loan order
    ///
    /// # Returns
    /// Detailed order information including current balances and status
    pub async fn get_multi_collateral_order(&self, order_id: &str) -> RestResult<OrderDetail> {
        let endpoint = format!("{}/{}", ORDERS_ENDPOINT, order_id);
        self.send_get_request::<OrderDetail, ()>(&endpoint, None)
            .await
    }
}
