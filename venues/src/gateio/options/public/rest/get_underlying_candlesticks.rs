use super::{RestClient, candlestick::{UnderlyingCandlestick, UnderlyingCandlesticksRequest}};

const OPTIONS_UNDERLYING_CANDLESTICKS_ENDPOINT: &str = "/options/underlying/candlesticks";

impl RestClient {
    /// Mark price candlesticks of an underlying
    ///
    /// Retrieves mark price candlestick data for an underlying asset.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#mark-price-candlesticks-of-underlying>
    pub async fn get_underlying_candlesticks(
        &self,
        params: UnderlyingCandlesticksRequest,
    ) -> crate::gateio::options::Result<Vec<UnderlyingCandlestick>> {
        self.get_with_query(OPTIONS_UNDERLYING_CANDLESTICKS_ENDPOINT, Some(&params))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_underlying_candlesticks_request_minimal() {
        let request = UnderlyingCandlesticksRequest {
            underlying: "BTC_USDT".to_string(),
            from: None,
            to: None,
            limit: None,
            interval: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "BTC_USDT");
        assert!(!json.as_object().unwrap().contains_key("from"));
        assert!(!json.as_object().unwrap().contains_key("to"));
        assert!(!json.as_object().unwrap().contains_key("limit"));
        assert!(!json.as_object().unwrap().contains_key("interval"));
    }

    #[test]
    fn test_underlying_candlesticks_request_with_time_range() {
        let request = UnderlyingCandlesticksRequest {
            underlying: "ETH_USDT".to_string(),
            from: Some(1640995200),
            to: Some(1640995800),
            limit: None,
            interval: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "ETH_USDT");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
    }

    #[test]
    fn test_underlying_candlesticks_request_full() {
        let request = UnderlyingCandlesticksRequest {
            underlying: "BNB_USDT".to_string(),
            from: Some(1640995200),
            to: Some(1640995800),
            limit: Some(500),
            interval: Some("1d".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "BNB_USDT");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
        assert_eq!(json["limit"], 500);
        assert_eq!(json["interval"], "1d");
    }

    #[test]
    fn test_underlying_candlesticks_request_underlyings() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "BNB_USDT", "SOL_USDT", "ADA_USDT"];

        for underlying in underlyings {
            let request = UnderlyingCandlesticksRequest {
                underlying: underlying.to_string(),
                from: None,
                to: None,
                limit: None,
                interval: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["underlying"], underlying);
        }
    }

    #[test]
    fn test_underlying_candlesticks_request_intervals() {
        let intervals = vec![
            "10s", "1m", "5m", "15m", "30m", "1h", "4h", "8h", "1d", "7d", "30d",
        ];

        for interval in intervals {
            let request = UnderlyingCandlesticksRequest {
                underlying: "BTC_USDT".to_string(),
                from: None,
                to: None,
                limit: None,
                interval: Some(interval.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["interval"], interval);
        }
    }

    #[test]
    fn test_underlying_candlesticks_request_limits() {
        let limits = vec![1, 50, 100, 500, 1000];

        for limit in limits {
            let request = UnderlyingCandlesticksRequest {
                underlying: "BTC_USDT".to_string(),
                from: None,
                to: None,
                limit: Some(limit),
                interval: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
        }
    }

    #[test]
    fn test_underlying_candlesticks_endpoint() {
        assert_eq!(
            OPTIONS_UNDERLYING_CANDLESTICKS_ENDPOINT,
            "/options/underlying/candlesticks"
        );
    }

    #[test]
    fn test_underlying_candlesticks_request_extreme_values() {
        let request = UnderlyingCandlesticksRequest {
            underlying: "BTC_USDT".to_string(),
            from: Some(i64::MIN),
            to: Some(i64::MAX),
            limit: Some(i32::MAX),
            interval: Some("30d".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "BTC_USDT");
        assert_eq!(json["from"], i64::MIN);
        assert_eq!(json["to"], i64::MAX);
        assert_eq!(json["limit"], i32::MAX);
        assert_eq!(json["interval"], "30d");
    }
}
