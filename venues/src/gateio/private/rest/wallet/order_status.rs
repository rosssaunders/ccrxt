use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const ORDER_STATUS_ENDPOINT: &str = "/wallet/order_status";

/// Request parameters for querying transfer status
#[derive(Debug, Clone, Serialize)]
pub struct OrderStatusRequest {
    /// Currency of the transfer
    pub currency: String,
}

/// Transfer order status information
#[derive(Debug, Clone, Deserialize)]
pub struct OrderStatus {
    /// Currency
    pub currency: String,

    /// Transfer order ID
    pub id: String,

    /// Status of the transfer order
    pub status: String,

    /// Transfer amount
    pub amount: String,

    /// Source account
    pub from: String,

    /// Destination account  
    pub to: String,

    /// Transfer timestamp
    pub create_time: i64,
}

impl RestClient {
    /// Transfer Status Query
    ///
    /// Query the status of internal transfer orders.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#transfer-status-query)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Request with currency to query transfer status for
    ///
    /// # Returns
    /// List of transfer order status information
    pub async fn get_transfer_order_status(
        &self,
        req: OrderStatusRequest,
    ) -> RestResult<Vec<OrderStatus>> {
        self.send_get_request(ORDER_STATUS_ENDPOINT, Some(&req))
            .await
    }
}
