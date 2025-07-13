use serde::Serialize;

use super::RestClient;
use crate::gateio::spotandmargin::private::rest::create_order::Order;

/// Get order request parameters
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderRequest {
    /// Currency pair
    pub currency_pair: String,

    /// Account type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

impl RestClient {
    /// Get a specific order
    ///
    /// This endpoint returns details of a specific order.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-a-single-order>
    pub async fn get_order(
        &self,
        order_id: &str,
        currency_pair: &str,
    ) -> crate::gateio::spotandmargin::Result<Order> {
        let query = GetOrderRequest {
            currency_pair: currency_pair.to_string(),
            account: None,
        };
        let endpoint = format!("/spot/orders/{}", order_id);
        self.get_with_query(&endpoint, &query).await
    }
}
