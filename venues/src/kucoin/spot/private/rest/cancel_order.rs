use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result, private_client::RestClient};

const CANCEL_ORDER_ENDPOINT: &str = "/api/v1/orders/{order_id}";

/// Request for cancelling an order
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    /// Order ID to cancel (goes in path, not serialized)
    #[serde(skip_serializing)]
    pub order_id: String,
}

/// Order cancellation response
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderResponse {
    /// List of cancelled order IDs
    #[serde(rename = "cancelledOrderIds")]
    pub cancelled_order_ids: Vec<String>,
}

impl RestClient {
    /// Cancel an order by order ID
    ///
    /// [docs](https://docs.kucoin.com/#cancel-hf-order-by-orderid)
    pub async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> Result<(CancelOrderResponse, ResponseHeaders)> {
        let endpoint = CANCEL_ORDER_ENDPOINT.replace("{order_id}", &request.order_id);

        let (response, headers): (RestResponse<CancelOrderResponse>, ResponseHeaders) =
            self.delete_with_request(&endpoint, &request).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_order_request_creation() {
        let request = CancelOrderRequest {
            order_id: "test_order_id".to_string(),
        };
        assert_eq!(request.order_id, "test_order_id");
    }
}
