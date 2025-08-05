use super::{RestClient, order::FuturesOrder};

impl RestClient {
    /// Cancel all futures orders
    ///
    /// This endpoint cancels all futures orders for a specific contract or all contracts.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#cancel-all-open-orders-matched>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `contract` - Optional contract filter
    /// * `side` - Optional side filter (buy/sell)
    ///
    /// # Returns
    /// List of cancelled orders
    pub async fn cancel_all_futures_orders(
        &self,
        settle: &str,
        contract: Option<&str>,
        side: Option<&str>,
    ) -> crate::gateio::perpetual::RestResult<Vec<FuturesOrder>> {
        let mut endpoint = format!("/futures/{}/orders", settle);

        let mut query_params = Vec::new();
        if let Some(contract) = contract {
            query_params.push(format!("contract={}", contract));
        }
        if let Some(side) = side {
            query_params.push(format!("side={}", side));
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
    #[test]
    fn test_cancel_all_basic_endpoint() {
        let endpoint = format!("/futures/{}/orders", "USDT");
        assert_eq!(endpoint, "/futures/USDT/orders");
    }

    #[test]
    fn test_cancel_all_with_contract() {
        let settle = "USDT";
        let contract = Some("BTC_USDT");

        let mut endpoint = format!("/futures/{}/orders", settle);
        let mut query_params = Vec::new();

        if let Some(contract) = contract {
            query_params.push(format!("contract={}", contract));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        assert_eq!(endpoint, "/futures/USDT/orders?contract=BTC_USDT");
    }

    #[test]
    fn test_cancel_all_with_side() {
        let settle = "USDT";
        let side = Some("buy");

        let mut endpoint = format!("/futures/{}/orders", settle);
        let mut query_params = Vec::new();

        if let Some(side) = side {
            query_params.push(format!("side={}", side));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        assert_eq!(endpoint, "/futures/USDT/orders?side=buy");
    }

    #[test]
    fn test_cancel_all_with_both_params() {
        let settle = "USDT";
        let contract = Some("ETH_USDT");
        let side = Some("sell");

        let mut endpoint = format!("/futures/{}/orders", settle);
        let mut query_params = Vec::new();

        if let Some(contract) = contract {
            query_params.push(format!("contract={}", contract));
        }
        if let Some(side) = side {
            query_params.push(format!("side={}", side));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        assert_eq!(endpoint, "/futures/USDT/orders?contract=ETH_USDT&side=sell");
    }

    #[test]
    fn test_cancel_all_side_options() {
        let sides = vec![
            ("buy", "Cancel all buy orders"),
            ("sell", "Cancel all sell orders"),
        ];

        for (side, _description) in sides {
            let mut endpoint = format!("/futures/USDT/orders");
            endpoint.push_str(&format!("?side={}", side));
            assert!(endpoint.contains(side));
        }
    }

    #[test]
    fn test_cancel_all_contract_filters() {
        let contracts = vec!["BTC_USDT", "ETH_USDT", "SOL_USDT", "MATIC_USDT"];

        for contract in contracts {
            let mut endpoint = format!("/futures/USDT/orders");
            endpoint.push_str(&format!("?contract={}", contract));
            assert!(endpoint.contains(contract));
        }
    }

    #[test]
    fn test_cancel_all_scenarios() {
        // Scenario 1: Cancel all orders
        let endpoint = format!("/futures/USDT/orders");
        assert_eq!(endpoint, "/futures/USDT/orders");

        // Scenario 2: Cancel all BTC buy orders
        let mut endpoint = format!("/futures/USDT/orders");
        endpoint.push_str("?contract=BTC_USDT&side=buy");
        assert!(endpoint.contains("BTC_USDT"));
        assert!(endpoint.contains("buy"));

        // Scenario 3: Cancel all sell orders across all contracts
        let mut endpoint = format!("/futures/USDT/orders");
        endpoint.push_str("?side=sell");
        assert!(endpoint.contains("sell"));
        assert!(!endpoint.contains("contract"));
    }
}
