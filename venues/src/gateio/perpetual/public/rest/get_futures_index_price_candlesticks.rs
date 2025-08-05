use super::{
    RestClient,
    candlestick::{FuturesCandlestick, FuturesCandlesticksRequest},
};

impl RestClient {
    /// Get futures index price candlesticks
    ///
    /// Retrieves index price candlestick data for a specific futures contract.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-futures-candlesticks>
    ///
    /// # Arguments
    /// * `params` - The candlestick query parameters
    ///
    /// # Returns
    /// List of index price candlestick data
    pub async fn get_futures_index_price_candlesticks(
        &self,
        params: FuturesCandlesticksRequest,
    ) -> crate::gateio::perpetual::RestResult<Vec<FuturesCandlestick>> {
        let mut index_params = params;
        index_params.contract = format!("index_{}", index_params.contract);
        let endpoint = format!("/futures/{}/candlesticks", index_params.settle);
        self.get_with_query(&endpoint, Some(&index_params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateio::shared::enums::CandlestickInterval;

    #[test]
    fn test_index_price_contract_prefix() {
        let request = FuturesCandlesticksRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            interval: Some(CandlestickInterval::Minutes5),
            from: None,
            to: None,
            limit: None,
        };

        // Simulate the transformation
        let mut index_params = request.clone();
        index_params.contract = format!("index_{}", index_params.contract);

        assert_eq!(index_params.contract, "index_BTC_USDT");
        assert!(index_params.contract.starts_with("index_"));
    }

    #[test]
    fn test_various_contracts_with_index_prefix() {
        let contracts = vec![
            "BTC_USDT",
            "ETH_USDT",
            "SOL_USDT",
            "MATIC_USDT",
            "BTC_USD",
            "ETH_USD",
        ];

        for contract in contracts {
            let request = FuturesCandlesticksRequest {
                settle: "USDT".to_string(),
                contract: contract.to_string(),
                interval: Some(CandlestickInterval::Hours1),
                from: None,
                to: None,
                limit: None,
            };

            let mut index_params = request;
            index_params.contract = format!("index_{}", index_params.contract);

            assert!(index_params.contract.starts_with("index_"));
            assert!(index_params.contract.contains(contract));
        }
    }

    #[test]
    fn test_endpoint_formatting_with_settlements() {
        let settlements = vec!["USDT", "BTC", "ETH"];

        for settle in settlements {
            let endpoint = format!("/futures/{}/candlesticks", settle);
            assert!(endpoint.contains(settle));
            assert!(endpoint.starts_with("/futures"));
            assert!(endpoint.ends_with("/candlesticks"));
        }
    }

    #[test]
    fn test_index_price_candlesticks_scenarios() {
        // Scenario 1: Short-term index price data
        let short_term = FuturesCandlesticksRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            interval: Some(CandlestickInterval::Minutes1),
            from: Some(1640995200),
            to: Some(1640995800), // 10 minutes
            limit: Some(10),
        };

        let mut index_params = short_term;
        index_params.contract = format!("index_{}", index_params.contract);
        assert_eq!(index_params.contract, "index_BTC_USDT");

        // Scenario 2: Daily index price data
        let daily = FuturesCandlesticksRequest {
            settle: "USDT".to_string(),
            contract: "ETH_USDT".to_string(),
            interval: Some(CandlestickInterval::Days7),
            from: Some(1640908800),
            to: Some(1641513600), // 7 days
            limit: Some(7),
        };

        let mut index_params = daily;
        index_params.contract = format!("index_{}", index_params.contract);
        assert_eq!(index_params.contract, "index_ETH_USDT");

        // Scenario 3: Hourly index price data for analysis
        let hourly = FuturesCandlesticksRequest {
            settle: "USDT".to_string(),
            contract: "SOL_USDT".to_string(),
            interval: Some(CandlestickInterval::Hours1),
            from: Some(1640908800),
            to: Some(1640995200), // 24 hours
            limit: Some(24),
        };

        let mut index_params = hourly;
        index_params.contract = format!("index_{}", index_params.contract);
        assert_eq!(index_params.contract, "index_SOL_USDT");
    }

    #[test]
    fn test_all_interval_options() {
        let intervals = vec![
            CandlestickInterval::Minutes1,
            CandlestickInterval::Minutes5,
            CandlestickInterval::Minutes15,
            CandlestickInterval::Minutes30,
            CandlestickInterval::Hours1,
            CandlestickInterval::Hours4,
            CandlestickInterval::Days1,
            CandlestickInterval::Days7,
        ];

        for interval in intervals {
            let request = FuturesCandlesticksRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                interval: Some(interval),
                from: None,
                to: None,
                limit: None,
            };

            let mut index_params = request;
            index_params.contract = format!("index_{}", index_params.contract);

            assert!(index_params.contract.starts_with("index_"));
            assert!(index_params.interval.is_some());
        }
    }

    #[test]
    fn test_index_vs_mark_price_comparison() {
        let base_request = FuturesCandlesticksRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            interval: Some(CandlestickInterval::Minutes5),
            from: Some(1640995200),
            to: Some(1640998800),
            limit: Some(100),
        };

        // Index price version
        let mut index_params = base_request.clone();
        index_params.contract = format!("index_{}", index_params.contract);
        assert_eq!(index_params.contract, "index_BTC_USDT");

        // Mark price version (for comparison)
        let mut mark_params = base_request.clone();
        mark_params.contract = format!("mark_{}", mark_params.contract);
        assert_eq!(mark_params.contract, "mark_BTC_USDT");

        // Both should have different prefixes
        assert_ne!(index_params.contract, mark_params.contract);
        assert!(index_params.contract.starts_with("index_"));
        assert!(mark_params.contract.starts_with("mark_"));
    }
}
