use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::private::rest::create_order::{CreateOrderRequest, Order};

/// Batch order creation request
#[derive(Debug, Clone, Serialize)]
pub struct CreateBatchOrdersRequest {
    /// List of orders to create
    pub orders: Vec<CreateOrderRequest>,
}

/// Batch order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOrderResponse {
    /// Whether the order was successfully created
    pub succeeded: bool,

    /// Order details if successful
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,

    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl RestClient {
    /// Create multiple orders in batch
    ///
    /// This endpoint creates multiple orders in a single request.
    /// Maximum 10 orders per batch.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#create-a-batch-of-orders>
    pub async fn create_batch_orders(
        &self,
        orders: Vec<CreateOrderRequest>,
    ) -> crate::gateio::Result<Vec<BatchOrderResponse>> {
        let request = CreateBatchOrdersRequest { orders };
        self.post("/spot/batch_orders", &request).await
    }
}
