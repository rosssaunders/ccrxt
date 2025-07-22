use super::RestClient;
use crate::gateio::delivery::models::DeliveryOrder;

const DELIVERY_ORDERS_ENDPOINT: &str = "/delivery/{}/orders";
const DELIVERY_ORDER_ENDPOINT: &str = "/delivery/{}/orders/{}";

impl RestClient {
    /// Cancel all delivery orders
    ///
    /// This endpoint cancels all delivery orders for a specific contract or all contracts.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `contract` - Optional contract filter
    ///
    /// # Returns
    /// List of cancelled delivery orders
    pub async fn cancel_all_delivery_orders(
        &self,
        settle: &str,
        contract: Option<&str>,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryOrder>> {
        let mut endpoint = DELIVERY_ORDERS_ENDPOINT.replace("{}", settle);

        if let Some(contract) = contract {
            endpoint.push_str(&format!("?contract={}", contract));
        }

        self.delete(&endpoint).await
    }

    /// Cancel a specific delivery order
    ///
    /// This endpoint cancels a specific delivery order.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `order_id` - Order ID to cancel
    ///
    /// # Returns
    /// Cancelled delivery order details
    pub async fn cancel_delivery_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> crate::gateio::delivery::Result<DeliveryOrder> {
        let endpoint = DELIVERY_ORDER_ENDPOINT
            .replace("{}", settle)
            .replace("{}", order_id);
        self.delete(&endpoint).await
    }
}
