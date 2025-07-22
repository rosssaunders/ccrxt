use super::RestClient;
use super::order::OptionsOrder;

impl RestClient {
    /// Get a specific options order
    ///
    /// This endpoint returns details for a specific options order.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-a-single-options-order>
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `order_id` - Order ID to retrieve
    ///
    /// # Returns
    /// Specific options order details
    pub async fn get_options_order(
        &self,
        order_id: &str,
    ) -> crate::gateio::options::Result<OptionsOrder> {
        let endpoint = format!("/options/orders/{}", order_id);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_options_order_endpoint() {
        let order_ids = vec!["12345", "67890", "999999999", "1", "0"];

        for order_id in order_ids {
            let endpoint = format!("/options/orders/{}", order_id);
            assert!(endpoint.starts_with("/options/orders/"));
            assert!(endpoint.ends_with(order_id));
        }
    }

    #[test]
    fn test_get_options_order_endpoint_format() {
        let order_id = "12345678";
        let endpoint = format!("/options/orders/{}", order_id);
        assert_eq!(endpoint, "/options/orders/12345678");
    }

    #[test]
    fn test_get_options_order_endpoint_with_special_chars() {
        // Test with order IDs that might contain special characters
        let order_ids = vec!["abc123", "123-456", "order_789"];

        for order_id in order_ids {
            let endpoint = format!("/options/orders/{}", order_id);
            assert!(endpoint.contains(order_id));
        }
    }
}
