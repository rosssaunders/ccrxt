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
    ) -> crate::gateio::delivery::RestResult<Vec<DeliveryOrder>> {
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
    ) -> crate::gateio::delivery::RestResult<DeliveryOrder> {
        let endpoint = DELIVERY_ORDER_ENDPOINT
            .replacen("{}", settle, 1)
            .replacen("{}", order_id, 1);
        self.get(&endpoint).await
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
    fn test_delivery_order_endpoint() {
        assert_eq!(DELIVERY_ORDER_ENDPOINT, "/delivery/{}/orders/{}");
    }

    #[test]
    fn test_list_delivery_orders_endpoint_construction() {
        let settle = "BTC";
        let endpoint = DELIVERY_ORDERS_ENDPOINT.replace("{}", settle);
        assert_eq!(endpoint, "/delivery/BTC/orders");
    }

    #[test]
    fn test_get_delivery_order_endpoint_construction() {
        let settle = "USDT";
        let order_id = "12345678";
        let endpoint = DELIVERY_ORDER_ENDPOINT
            .replacen("{}", settle, 1)
            .replacen("{}", order_id, 1);
        assert_eq!(endpoint, "/delivery/USDT/orders/12345678");
    }

    #[test]
    fn test_endpoint_different_settlements() {
        let test_cases = vec![
            ("BTC", "/delivery/BTC/orders"),
            ("USDT", "/delivery/USDT/orders"),
            ("ETH", "/delivery/ETH/orders"),
        ];

        for (settle, expected) in test_cases {
            let endpoint = DELIVERY_ORDERS_ENDPOINT.replace("{}", settle);
            assert_eq!(endpoint, expected, "Failed for settlement: {}", settle);
        }
    }

    #[test]
    fn test_get_order_endpoint_different_params() {
        let test_cases = vec![
            ("BTC", "order123", "/delivery/BTC/orders/order123"),
            ("USDT", "987654321", "/delivery/USDT/orders/987654321"),
            ("ETH", "abc-def-123", "/delivery/ETH/orders/abc-def-123"),
        ];

        for (settle, order_id, expected) in test_cases {
            let endpoint = DELIVERY_ORDER_ENDPOINT
                .replacen("{}", settle, 1)
                .replacen("{}", order_id, 1);
            assert_eq!(endpoint, expected, "Failed for settle: {}, order_id: {}", settle, order_id);
        }
    }

    #[test]
    fn test_endpoints_have_correct_placeholders() {
        // Orders endpoint should have one placeholder
        let orders_placeholder_count = DELIVERY_ORDERS_ENDPOINT.matches("{}").count();
        assert_eq!(orders_placeholder_count, 1);

        // Order endpoint should have two placeholders
        let order_placeholder_count = DELIVERY_ORDER_ENDPOINT.matches("{}").count();
        assert_eq!(order_placeholder_count, 2);
    }

    #[test]
    fn test_endpoint_paths_structure() {
        assert!(DELIVERY_ORDERS_ENDPOINT.starts_with("/delivery/"));
        assert!(DELIVERY_ORDERS_ENDPOINT.ends_with("/orders"));
        
        assert!(DELIVERY_ORDER_ENDPOINT.starts_with("/delivery/"));
        assert!(DELIVERY_ORDER_ENDPOINT.contains("/orders/"));
    }

    #[test]
    fn test_endpoint_parameter_replacement_completeness() {
        let settle = "BTC";
        let order_id = "test123";

        let orders_endpoint = DELIVERY_ORDERS_ENDPOINT.replace("{}", settle);
        assert!(!orders_endpoint.contains("{}"));
        assert!(orders_endpoint.contains(settle));

        let order_endpoint = DELIVERY_ORDER_ENDPOINT
            .replacen("{}", settle, 1)
            .replacen("{}", order_id, 1);
        assert!(!order_endpoint.contains("{}"));
        assert!(order_endpoint.contains(settle));
        assert!(order_endpoint.contains(order_id));
    }
}
