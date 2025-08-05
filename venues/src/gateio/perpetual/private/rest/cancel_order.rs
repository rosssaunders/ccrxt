use super::{RestClient, order::FuturesOrder};

const FUTURES_ORDERS_ENDPOINT: &str = "/futures/{}/orders/{}";

impl RestClient {
    /// Cancel a specific futures order
    ///
    /// This endpoint cancels a specific futures order.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#cancel-a-single-order>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `order_id` - Order ID to cancel
    ///
    /// # Returns
    /// Cancelled order details
    pub async fn cancel_futures_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> crate::gateio::perpetual::Result<FuturesOrder> {
        let endpoint = FUTURES_ORDERS_ENDPOINT
            .replacen("{}", settle, 1)
            .replacen("{}", order_id, 1);
        self.delete(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_order_endpoint_formatting() {
        let test_cases = vec![
            ("USDT", "12345", "/futures/USDT/orders/12345"),
            ("BTC", "67890", "/futures/BTC/orders/67890"),
            ("ETH", "11111", "/futures/ETH/orders/11111"),
        ];

        for (settle, order_id, expected) in test_cases {
            let endpoint = FUTURES_ORDERS_ENDPOINT
                .replacen("{}", settle, 1)
                .replacen("{}", order_id, 1);
            assert_eq!(endpoint, expected);
        }
    }

    #[test]
    fn test_cancel_various_order_ids() {
        let order_ids = vec!["12345", "9876543210", "1", "999999999999"];

        for order_id in order_ids {
            let endpoint = FUTURES_ORDERS_ENDPOINT
                .replacen("{}", "USDT", 1)
                .replacen("{}", order_id, 1);
            assert!(endpoint.contains(order_id));
            assert!(endpoint.ends_with(order_id));
        }
    }

    #[test]
    fn test_cancel_different_settlements() {
        let settlements = vec!["USDT", "BTC", "ETH"];

        for settle in settlements {
            let endpoint = FUTURES_ORDERS_ENDPOINT
                .replacen("{}", settle, 1)
                .replacen("{}", "12345", 1);
            assert!(endpoint.contains(settle));
            assert!(endpoint.starts_with(&format!("/futures/{}", settle)));
        }
    }
}
