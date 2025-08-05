use super::RestClient;
use crate::gateio::delivery::RestResult;
use crate::gateio::delivery::models::DeliveryOrder;

const DELIVERY_ORDERS_ENDPOINT: &str = "/delivery/{}/orders";
const DELIVERY_ORDER_ENDPOINT: &str = "/delivery/{}/orders/{}";

impl RestClient {
    /// Cancel all delivery orders
    ///
    /// This endpoint cancels all delivery orders for a specific contract or all contracts.
    ///
    /// See: <https://www.gate.com/docs/developers/apiv4/#cancel-single-order-3>
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
    ) -> RestResult<Vec<DeliveryOrder>> {
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
    /// See: <https://www.gate.com/docs/developers/apiv4/#cancel-single-order-3>
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
    ) -> RestResult<DeliveryOrder> {
        let endpoint = DELIVERY_ORDER_ENDPOINT
            .replacen("{}", settle, 1)
            .replacen("{}", order_id, 1);
        self.delete(&endpoint).await
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
    fn test_cancel_all_delivery_orders_endpoint_without_contract() {
        let settle = "BTC";
        let expected = "/delivery/BTC/orders";
        let actual = DELIVERY_ORDERS_ENDPOINT.replace("{}", settle);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_cancel_all_delivery_orders_endpoint_with_contract() {
        let settle = "USDT";
        let contract = "BTC_USDT_20240315";
        let mut endpoint = DELIVERY_ORDERS_ENDPOINT.replace("{}", settle);
        endpoint.push_str(&format!("?contract={}", contract));
        
        assert_eq!(endpoint, "/delivery/USDT/orders?contract=BTC_USDT_20240315");
    }

    #[test]
    fn test_cancel_delivery_order_endpoint_construction() {
        let settle = "BTC";
        let order_id = "12345678";
        let endpoint = DELIVERY_ORDER_ENDPOINT
            .replacen("{}", settle, 1)
            .replacen("{}", order_id, 1);
        
        assert_eq!(endpoint, "/delivery/BTC/orders/12345678");
    }

    #[test]
    fn test_cancel_delivery_order_endpoint_different_params() {
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
    fn test_cancel_all_delivery_orders_different_settlements() {
        let test_cases = vec![
            ("BTC", None, "/delivery/BTC/orders"),
            ("USDT", None, "/delivery/USDT/orders"),
            ("BTC", Some("BTC_USDT_20240315"), "/delivery/BTC/orders?contract=BTC_USDT_20240315"),
            ("USDT", Some("ETH_USDT_20240415"), "/delivery/USDT/orders?contract=ETH_USDT_20240415"),
        ];

        for (settle, contract, expected) in test_cases {
            let mut endpoint = DELIVERY_ORDERS_ENDPOINT.replace("{}", settle);
            
            if let Some(contract) = contract {
                endpoint.push_str(&format!("?contract={}", contract));
            }
            
            assert_eq!(endpoint, expected, "Failed for settle: {}, contract: {:?}", settle, contract);
        }
    }

    #[test]
    fn test_endpoint_parameter_handling() {
        // Test that endpoints handle string replacements correctly
        let settle_values = vec!["BTC", "USDT", "ETH", "SOL"];
        let order_ids = vec!["123", "order_abc", "12345678901234567890"];

        for settle in settle_values {
            let orders_endpoint = DELIVERY_ORDERS_ENDPOINT.replace("{}", settle);
            assert!(orders_endpoint.contains(settle));
            assert!(!orders_endpoint.contains("{}"));

            for order_id in &order_ids {
                let order_endpoint = DELIVERY_ORDER_ENDPOINT
                    .replacen("{}", settle, 1)
                    .replacen("{}", order_id, 1);
                assert!(order_endpoint.contains(settle));
                assert!(order_endpoint.contains(order_id));
                assert!(!order_endpoint.contains("{}"));
            }
        }
    }
}
