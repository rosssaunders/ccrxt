use serde::Serialize;

use super::RestClient;
use crate::gateio::private::rest::create_order::Order;

/// Cancel order request parameters
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    /// Currency pair
    pub currency_pair: String,

    /// Account type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

impl RestClient {
    /// Cancel an order
    ///
    /// This endpoint cancels a specific order.
    pub async fn cancel_order(
        &self,
        order_id: &str,
        currency_pair: &str,
    ) -> crate::gateio::Result<Order> {
        let query = CancelOrderRequest {
            currency_pair: currency_pair.to_string(),
            account: None,
        };
        let endpoint = format!("/spot/orders/{}", order_id);
        self.delete_with_query(&endpoint, &query).await
    }
}
