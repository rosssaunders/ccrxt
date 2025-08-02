use super::RestClient;
use crate::gateio::delivery::models::{CreateDeliveryOrderRequest, DeliveryOrder};

const DELIVERY_ORDERS_ENDPOINT: &str = "/delivery/{}/orders";

impl RestClient {
    /// Create a delivery order
    ///
    /// This endpoint creates a new delivery order.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The delivery order creation request parameters
    ///
    /// # Returns
    /// Created delivery order information
    pub async fn create_delivery_order(
        &self,
        request: CreateDeliveryOrderRequest,
    ) -> crate::gateio::delivery::Result<DeliveryOrder> {
        let endpoint = DELIVERY_ORDERS_ENDPOINT.replace("{}", &request.settle);
        self.post(&endpoint, &request).await
    }
}
