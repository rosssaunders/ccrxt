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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_orders_endpoint() {
        assert_eq!(DELIVERY_ORDERS_ENDPOINT, "/delivery/{}/orders");
    }

    #[test]
    fn test_create_delivery_order_endpoint_construction() {
        let settle = "BTC";
        let endpoint = DELIVERY_ORDERS_ENDPOINT.replace("{}", settle);
        assert_eq!(endpoint, "/delivery/BTC/orders");
    }

    #[test]
    fn test_create_delivery_order_endpoint_different_settlements() {
        let test_cases = vec![
            ("BTC", "/delivery/BTC/orders"),
            ("USDT", "/delivery/USDT/orders"),
            ("ETH", "/delivery/ETH/orders"),
            ("SOL", "/delivery/SOL/orders"),
        ];

        for (settle, expected) in test_cases {
            let endpoint = DELIVERY_ORDERS_ENDPOINT.replace("{}", settle);
            assert_eq!(endpoint, expected, "Failed for settlement: {}", settle);
        }
    }

    #[test]
    fn test_endpoint_parameter_replacement() {
        // Test that the endpoint correctly replaces the placeholder
        let original = DELIVERY_ORDERS_ENDPOINT;
        assert!(original.contains("{}"));

        let settle = "USDT";
        let replaced = original.replace("{}", settle);
        assert!(!replaced.contains("{}"));
        assert!(replaced.contains(settle));
    }

    #[test]
    fn test_endpoint_no_extra_placeholders() {
        // Ensure there's only one placeholder to replace
        let placeholder_count = DELIVERY_ORDERS_ENDPOINT.matches("{}").count();
        assert_eq!(placeholder_count, 1, "Endpoint should have exactly one placeholder");
    }

    #[test]
    fn test_endpoint_starts_with_delivery() {
        assert!(DELIVERY_ORDERS_ENDPOINT.starts_with("/delivery/"));
    }

    #[test]
    fn test_endpoint_ends_with_orders() {
        assert!(DELIVERY_ORDERS_ENDPOINT.ends_with("/orders"));
    }
}
