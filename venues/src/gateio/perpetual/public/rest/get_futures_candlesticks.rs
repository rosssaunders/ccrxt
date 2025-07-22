use super::RestClient;
use super::candlestick::{FuturesCandlestick, FuturesCandlesticksRequest};

impl RestClient {
    /// Get futures candlesticks
    ///
    /// Retrieves candlestick data for a specific futures contract.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-futures-candlesticks>
    ///
    /// # Arguments
    /// * `params` - The candlestick query parameters
    ///
    /// # Returns
    /// List of candlestick data
    pub async fn get_futures_candlesticks(
        &self,
        params: FuturesCandlesticksRequest,
    ) -> crate::gateio::perpetual::Result<Vec<FuturesCandlestick>> {
        let endpoint = format!("/futures/{}/candlesticks", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateio::shared::enums::CandlestickInterval;

    #[test]
    fn test_endpoint_formatting() {
        let settlements = vec!["USDT", "BTC", "ETH"];

        for settle in settlements {
            let endpoint = format!("/futures/{}/candlesticks", settle);
            assert!(endpoint.contains(settle));
            assert!(endpoint.starts_with("/futures"));
            assert!(endpoint.ends_with("/candlesticks"));
        }
    }

    #[test]
    fn test_various_intervals() {
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

            let json = serde_json::to_value(&request).unwrap();
            assert!(json["interval"].is_string());
        }
    }

    #[test]
    fn test_time_range_scenarios() {
        // Last hour
        let last_hour = FuturesCandlesticksRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            interval: Some(CandlestickInterval::Minutes5),
            from: Some(1640995200),
            to: Some(1640998800),
            limit: None,
        };

        let json = serde_json::to_value(&last_hour).unwrap();
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640998800);

        // Last day
        let last_day = FuturesCandlesticksRequest {
            settle: "USDT".to_string(),
            contract: "ETH_USDT".to_string(),
            interval: Some(CandlestickInterval::Hours1),
            from: Some(1640908800),
            to: Some(1640995200),
            limit: None,
        };

        let json = serde_json::to_value(&last_day).unwrap();
        assert_eq!(json["from"], 1640908800);
        assert_eq!(json["to"], 1640995200);
    }

    #[test]
    fn test_different_contracts() {
        let contracts = vec![
            ("USDT", "BTC_USDT"),
            ("USDT", "ETH_USDT"),
            ("USDT", "SOL_USDT"),
            ("BTC", "BTC_USD"),
            ("ETH", "ETH_USD"),
        ];

        for (settle, contract) in contracts {
            let request = FuturesCandlesticksRequest {
                settle: settle.to_string(),
                contract: contract.to_string(),
                interval: Some(CandlestickInterval::Minutes5),
                from: None,
                to: None,
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_limit_values() {
        let limits = vec![1, 10, 50, 100, 500, 1000];

        for limit in limits {
            let request = FuturesCandlesticksRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                interval: Some(CandlestickInterval::Minutes5),
                from: None,
                to: None,
                limit: Some(limit),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
        }
    }
}
