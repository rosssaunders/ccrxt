use super::{RestClient, position::FuturesPosition};

impl RestClient {
    /// Get a specific futures position
    ///
    /// This endpoint returns a specific futures position for the authenticated user.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-single-position>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `contract` - Contract name
    ///
    /// # Returns
    /// Futures position details
    pub async fn get_futures_position(
        &self,
        settle: &str,
        contract: &str,
    ) -> crate::gateio::perpetual::Result<FuturesPosition> {
        let endpoint = format!("/futures/{}/positions/{}", settle, contract);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_position_endpoint_formatting() {
        let test_cases = vec![
            ("USDT", "BTC_USDT", "/futures/USDT/positions/BTC_USDT"),
            ("BTC", "BTC_USD", "/futures/BTC/positions/BTC_USD"),
            ("ETH", "ETH_USD", "/futures/ETH/positions/ETH_USD"),
        ];

        for (settle, contract, expected) in test_cases {
            let endpoint = format!("/futures/{}/positions/{}", settle, contract);
            assert_eq!(endpoint, expected);
        }
    }

    #[test]
    fn test_various_contract_names() {
        let contracts = vec![
            ("USDT", "BTC_USDT"),
            ("USDT", "ETH_USDT"),
            ("USDT", "SOL_USDT"),
            ("USDT", "MATIC_USDT"),
            ("BTC", "BTC_USD"),
            ("ETH", "ETH_USD"),
        ];

        for (settle, contract) in contracts {
            let endpoint = format!("/futures/{}/positions/{}", settle, contract);
            assert!(endpoint.contains(settle));
            assert!(endpoint.contains(contract));
            assert!(endpoint.starts_with("/futures"));
            assert!(endpoint.contains("/positions/"));
        }
    }

    #[test]
    fn test_settlement_currencies() {
        let settlements = vec!["USDT", "BTC", "ETH"];

        for settle in settlements {
            let endpoint = format!("/futures/{}/positions/BTC_USDT", settle);
            assert!(endpoint.contains(settle));
        }
    }
}
