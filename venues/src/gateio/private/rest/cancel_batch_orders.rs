use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request to cancel batch orders
#[derive(Debug, Clone, Serialize)]
pub struct CancelBatchOrdersRequest {
    /// List of order IDs to cancel
    pub order_ids: Vec<String>,
}

/// Cancel batch orders response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelBatchOrdersResponse {
    /// Successfully cancelled order IDs
    pub succeeded: Vec<String>,

    /// Failed order cancellations with error details
    pub failed: Vec<CancelBatchOrderError>,
}

/// Failed batch order cancellation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelBatchOrderError {
    /// Order ID that failed to cancel
    pub id: String,

    /// Error message
    pub message: String,

    /// Error code
    pub code: String,
}

impl RestClient {
    /// Cancel multiple orders in batch
    ///
    /// This endpoint allows cancelling multiple orders at once. It returns
    /// information about which orders were successfully cancelled and which failed.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#cancel-a-batch-of-orders-with-an-id-list>
    pub async fn cancel_batch_orders(
        &self,
        request: CancelBatchOrdersRequest,
    ) -> crate::gateio::Result<CancelBatchOrdersResponse> {
        self.post("/spot/cancel_batch_orders", &request).await
    }
}
