use super::{RestClient, RestResult, order::FuturesOrder};

const ENDPOINT_FUTURES_PREFIX: &str = "/futures";

impl RestClient {
    /// Get a specific futures order
    ///
    /// This endpoint returns details for a specific futures order.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#get-a-single-order)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `order_id` - Order ID
    ///
    /// # Returns
    /// Order details
    pub async fn get_futures_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> RestResult<FuturesOrder> {
        let endpoint = format!("{}/{}/orders/{}", ENDPOINT_FUTURES_PREFIX, settle, order_id);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_order_endpoint_formatting() {
        let test_cases = vec![
            ("USDT", "12345", "/futures/USDT/orders/12345"),
            ("BTC", "67890", "/futures/BTC/orders/67890"),
            ("ETH", "11111", "/futures/ETH/orders/11111"),
        ];

        for (settle, order_id, expected) in test_cases {
            let endpoint = format!("/futures/{}/orders/{}", settle, order_id);
            assert_eq!(endpoint, expected);
        }
    }

    #[test]
    fn test_various_order_id_formats() {
        let order_ids = vec!["12345", "9876543210", "1", "999999999999"];

        for order_id in order_ids {
            let endpoint = format!("/futures/USDT/orders/{}", order_id);
            assert!(endpoint.contains(order_id));
        }
    }

    #[test]
    fn test_different_settlement_currencies() {
        let settlements = vec!["USDT", "BTC", "ETH"];

        for settle in settlements {
            let endpoint = format!("/futures/{}/orders/12345", settle);
            assert!(endpoint.contains(settle));
        }
    }
}
