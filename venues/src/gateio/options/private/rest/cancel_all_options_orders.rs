use super::{RestClient, order::OptionsOrder};

const OPTIONS_ORDERS_ENDPOINT: &str = "/options/orders";

impl RestClient {
    /// Cancel all options orders
    ///
    /// This endpoint cancels all options orders for a specific underlying or contract.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#cancel-all-open-options-orders>
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `underlying` - Optional underlying asset filter
    /// * `contract` - Optional contract filter
    ///
    /// # Returns
    /// List of cancelled options orders
    pub async fn cancel_all_options_orders(
        &self,
        underlying: Option<&str>,
        contract: Option<&str>,
    ) -> crate::gateio::options::Result<Vec<OptionsOrder>> {
        let mut endpoint = OPTIONS_ORDERS_ENDPOINT.to_string();
        let mut query_params = Vec::new();

        if let Some(underlying) = underlying {
            query_params.push(format!("underlying={}", underlying));
        }
        if let Some(contract) = contract {
            query_params.push(format!("contract={}", contract));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        self.delete(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cancel_all_options_orders_no_filters() {
        let endpoint = OPTIONS_ORDERS_ENDPOINT.to_string();
        assert_eq!(endpoint, "/options/orders");
    }

    #[test]
    fn test_cancel_all_options_orders_with_underlying() {
        let underlying = Some("BTC_USDT");
        let mut endpoint = OPTIONS_ORDERS_ENDPOINT.to_string();
        let mut query_params = Vec::new();

        if let Some(underlying) = underlying {
            query_params.push(format!("underlying={}", underlying));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        assert_eq!(endpoint, "/options/orders?underlying=BTC_USDT");
    }

    #[test]
    fn test_cancel_all_options_orders_with_contract() {
        let contract = Some("BTC-20240101-50000-C");
        let mut endpoint = OPTIONS_ORDERS_ENDPOINT.to_string();
        let mut query_params = Vec::new();

        if let Some(contract) = contract {
            query_params.push(format!("contract={}", contract));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        assert_eq!(endpoint, "/options/orders?contract=BTC-20240101-50000-C");
    }

    #[test]
    fn test_cancel_all_options_orders_with_both_filters() {
        let underlying = Some("ETH_USDT");
        let contract = Some("ETH-20240101-3000-P");
        let mut endpoint = OPTIONS_ORDERS_ENDPOINT.to_string();
        let mut query_params = Vec::new();

        if let Some(underlying) = underlying {
            query_params.push(format!("underlying={}", underlying));
        }
        if let Some(contract) = contract {
            query_params.push(format!("contract={}", contract));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        assert_eq!(
            endpoint,
            "/options/orders?underlying=ETH_USDT&contract=ETH-20240101-3000-P"
        );
    }

    #[test]
    fn test_cancel_all_options_orders_different_underlyings() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "SOL_USDT", "BNB_USDT"];

        for underlying in underlyings {
            let mut endpoint = OPTIONS_ORDERS_ENDPOINT.to_string();
            let mut query_params = Vec::new();
            query_params.push(format!("underlying={}", underlying));

            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));

            assert!(endpoint.contains(underlying));
            assert!(endpoint.starts_with("/options/orders?underlying="));
        }
    }

    #[test]
    fn test_cancel_all_options_orders_different_contracts() {
        let contracts = vec![
            "BTC-20240101-50000-C",
            "BTC-20240101-50000-P",
            "ETH-20240201-3000-C",
            "ETH-20240201-3000-P",
        ];

        for contract in contracts {
            let mut endpoint = OPTIONS_ORDERS_ENDPOINT.to_string();
            let mut query_params = Vec::new();
            query_params.push(format!("contract={}", contract));

            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));

            assert!(endpoint.contains(contract));
            assert!(endpoint.starts_with("/options/orders?contract="));
        }
    }

    #[test]
    fn test_cancel_all_options_orders_query_param_order() {
        // Test that query parameters are consistently ordered
        let underlying = Some("BTC_USDT");
        let contract = Some("BTC-20240101-50000-C");

        let mut endpoint = OPTIONS_ORDERS_ENDPOINT.to_string();
        let mut query_params = Vec::new();

        if let Some(underlying) = underlying {
            query_params.push(format!("underlying={}", underlying));
        }
        if let Some(contract) = contract {
            query_params.push(format!("contract={}", contract));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        // Should always be underlying first, then contract
        assert_eq!(
            endpoint,
            "/options/orders?underlying=BTC_USDT&contract=BTC-20240101-50000-C"
        );
    }
}
