use super::RestClient;
use super::order::OptionsOrder;

const OPTIONS_ORDERS_ENDPOINT: &str = "/options/orders";

impl RestClient {
    /// Cancel a specific options order
    ///
    /// This endpoint cancels a specific options order.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#cancel-a-single-options-order>
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `order_id` - Order ID to cancel
    ///
    /// # Returns
    /// Cancelled options order details
    pub async fn cancel_options_order(
        &self,
        order_id: &str,
    ) -> crate::gateio::options::Result<OptionsOrder> {
        let endpoint = format!("{}/{}", OPTIONS_ORDERS_ENDPOINT, order_id);
        self.delete(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cancel_options_order_endpoint() {
        let order_ids = vec!["12345", "67890", "999999999", "1", "0"];

        for order_id in order_ids {
            let endpoint = format!("{}/{}", OPTIONS_ORDERS_ENDPOINT, order_id);
            assert!(endpoint.starts_with("/options/orders/"));
            assert!(endpoint.ends_with(order_id));
        }
    }

    #[test]
    fn test_cancel_options_order_endpoint_format() {
        let order_id = "12345678";
        let endpoint = format!("{}/{}", OPTIONS_ORDERS_ENDPOINT, order_id);
        assert_eq!(endpoint, "/options/orders/12345678");
    }

    #[test]
    fn test_cancel_options_order_endpoint_with_long_ids() {
        let long_ids = vec!["123456789012345", "999999999999999", "111111111111111"];

        for order_id in long_ids {
            let endpoint = format!("{}/{}", OPTIONS_ORDERS_ENDPOINT, order_id);
            assert_eq!(endpoint, format!("/options/orders/{}", order_id));
        }
    }

    #[test]
    fn test_cancel_options_order_endpoint_with_special_chars() {
        // Test with order IDs that might contain special characters
        let order_ids = vec!["abc123", "123-456", "order_789"];

        for order_id in order_ids {
            let endpoint = format!("{}/{}", OPTIONS_ORDERS_ENDPOINT, order_id);
            assert!(endpoint.contains(order_id));
            assert_eq!(endpoint, format!("/options/orders/{}", order_id));
        }
    }
}
