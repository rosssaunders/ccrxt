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
    /// Get multi-collateral order by id
    pub async fn get_multi_collateral_order(
        &self,
        order_id: &str,
    ) -> RestResult<OrderDetail> {
        let endpoint = format!("{}/{}", ORDERS_ENDPOINT, order_id);
        self.send_get_request::<OrderDetail, ()>(&endpoint, None).await
    }
}
