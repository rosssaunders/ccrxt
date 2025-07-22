use super::RestClient;
use crate::gateio::delivery::models::{DeliveryOrder, ListDeliveryOrdersRequest};

const DELIVERY_ORDERS_ENDPOINT: &str = "/delivery/{}/orders";
const DELIVERY_ORDER_ENDPOINT: &str = "/delivery/{}/orders/{}";

impl RestClient {
    /// List delivery orders
    ///
    /// This endpoint returns delivery orders for the authenticated user.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery orders list request parameters
    ///
    /// # Returns
    /// List of delivery orders
    pub async fn list_delivery_orders(
        &self,
        params: ListDeliveryOrdersRequest,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryOrder>> {
        let endpoint = DELIVERY_ORDERS_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Get a specific delivery order
    ///
    /// This endpoint returns details for a specific delivery order.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `order_id` - Order ID to retrieve
    ///
    /// # Returns
    /// Specific delivery order details
    pub async fn get_delivery_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> crate::gateio::delivery::Result<DeliveryOrder> {
        let endpoint = DELIVERY_ORDER_ENDPOINT
            .replace("{}", settle)
            .replace("{}", order_id);
        self.get(&endpoint).await
    }
}
