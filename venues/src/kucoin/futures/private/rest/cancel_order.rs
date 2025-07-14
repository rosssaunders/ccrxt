use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Endpoint URL for cancel order (format string)
pub const CANCEL_ORDER_ENDPOINT: &str = "/api/v1/orders/";

#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    pub order_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub cancelled_order_ids: Vec<String>,
}

impl super::RestClient {
    /// Cancel an order
    pub async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> Result<(RestResponse<CancelOrderResponse>, ResponseHeaders)> {
        let endpoint = format!("{}{}", CANCEL_ORDER_ENDPOINT, request.order_id);
        self.delete(&endpoint, None::<Option<()>>).await
    }
}
