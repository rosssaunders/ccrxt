use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::spot::CandlestickInterval;

/// Request parameters for retrieving candlestick data
#[derive(Debug, Clone, Serialize, Default)]
pub struct CandlesticksRequest {
    /// Currency pair to query candlesticks for
    pub currency_pair: String,

    /// Candlestick interval (e.g., 1m, 5m, 1h, 1d)
    pub interval: CandlestickInterval,

    /// Maximum number of candlesticks to return (default: 100, max: 1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Start time for candlestick range (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time for candlestick range (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candlestick {
    /// Unix timestamp in seconds
    #[serde(rename = "0")]
    pub timestamp: String,

    /// Trading volume (quote currency)
    #[serde(rename = "1")]
    pub volume: String,

    /// Close price
    #[serde(rename = "2")]
    pub close: String,

    /// Highest price
    #[serde(rename = "3")]
    pub high: String,

    /// Lowest price
    #[serde(rename = "4")]
    pub low: String,

    /// Open price
    #[serde(rename = "5")]
    pub open: String,

    /// Trading volume (base currency)
    #[serde(rename = "6")]
    pub base_volume: String,
}

impl RestClient {
    /// Get candlestick data for a currency pair
    ///
    /// This endpoint returns OHLCV candlestick data for the specified currency pair and interval.
    /// You can filter by time range and limit the number of results.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#market-candlesticks>
    pub async fn get_candlesticks(
        &self,
        params: CandlesticksRequest,
    ) -> crate::gateio::spot::RestResult<Vec<Vec<String>>> {
        self.get_with_query("/spot/candlesticks", Some(&params))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candlesticks_request_minimal_serialization() {
        let request = CandlesticksRequest {
            currency_pair: "BTC_USDT".to_string(),
            interval: CandlestickInterval::Minutes1,
            limit: None,
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=BTC_USDT"));
        assert!(serialized.contains("interval=1m"));
        assert!(!serialized.contains("limit="));
        assert!(!serialized.contains("from="));
        assert!(!serialized.contains("to="));
    }

    #[test]
    fn test_candlesticks_request_with_limit() {
        let request = CandlesticksRequest {
            currency_pair: "ETH_USDT".to_string(),
            interval: CandlestickInterval::Hours1,
            limit: Some(100),
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=ETH_USDT"));
        assert!(serialized.contains("interval=1h"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_candlesticks_request_with_time_range() {
        let request = CandlesticksRequest {
            currency_pair: "BNB_USDT".to_string(),
            interval: CandlestickInterval::Days1,
            limit: None,
            from: Some(1640995200),
            to: Some(1641081600),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=BNB_USDT"));
        assert!(serialized.contains("interval=1d"));
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
    }

    #[test]
    fn test_candlesticks_request_full_parameters() {
        let request = CandlesticksRequest {
            currency_pair: "SOL_USDT".to_string(),
            interval: CandlestickInterval::Minutes5,
            limit: Some(500),
            from: Some(1640995200),
            to: Some(1641081600),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=SOL_USDT"));
        assert!(serialized.contains("interval=5m"));
        assert!(serialized.contains("limit=500"));
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
    }

    #[test]
    fn test_candlesticks_request_different_intervals() {
        let intervals = vec![
            (CandlestickInterval::Seconds10, "10s"),
            (CandlestickInterval::Minutes1, "1m"),
            (CandlestickInterval::Minutes5, "5m"),
            (CandlestickInterval::Minutes15, "15m"),
            (CandlestickInterval::Minutes30, "30m"),
            (CandlestickInterval::Hours1, "1h"),
            (CandlestickInterval::Hours4, "4h"),
            (CandlestickInterval::Hours8, "8h"),
            (CandlestickInterval::Days1, "1d"),
            (CandlestickInterval::Days7, "7d"),
            (CandlestickInterval::Days30, "30d"),
        ];

        for (interval, expected) in intervals {
            let request = CandlesticksRequest {
                currency_pair: "BTC_USDT".to_string(),
                interval,
                limit: None,
                from: None,
                to: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("interval={}", expected)));
        }
    }

    #[test]
    fn test_candlesticks_request_limit_edge_cases() {
        let limits = vec![1, 100, 500, 1000];

        for limit in limits {
            let request = CandlesticksRequest {
                currency_pair: "BTC_USDT".to_string(),
                interval: CandlestickInterval::Minutes1,
                limit: Some(limit),
                from: None,
                to: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("limit={}", limit)));
        }
    }

    #[test]
    fn test_candlesticks_request_max_limit() {
        let request = CandlesticksRequest {
            currency_pair: "BTC_USDT".to_string(),
            interval: CandlestickInterval::Minutes1,
            limit: Some(u32::MAX),
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains(&format!("limit={}", u32::MAX)));
    }

    #[test]
    fn test_candlesticks_request_negative_timestamps() {
        let request = CandlesticksRequest {
            currency_pair: "BTC_USDT".to_string(),
            interval: CandlestickInterval::Hours1,
            limit: None,
            from: Some(-1640995200),
            to: Some(-1640908800),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("from=-1640995200"));
        assert!(serialized.contains("to=-1640908800"));
    }

    #[test]
    fn test_candlesticks_request_extreme_timestamps() {
        let request = CandlesticksRequest {
            currency_pair: "BTC_USDT".to_string(),
            interval: CandlestickInterval::Days1,
            limit: None,
            from: Some(i64::MIN),
            to: Some(i64::MAX),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains(&format!("from={}", i64::MIN)));
        assert!(serialized.contains(&format!("to={}", i64::MAX)));
    }

    #[test]
    fn test_candlesticks_request_different_pairs() {
        let pairs = vec![
            "BTC_USDT",
            "ETH_USDT",
            "BNB_USDT",
            "SOL_USDT",
            "ADA_USDT",
            "DOT_USDT",
            "MATIC_USDT",
        ];

        for pair in pairs {
            let request = CandlesticksRequest {
                currency_pair: pair.to_string(),
                interval: CandlestickInterval::Hours1,
                limit: None,
                from: None,
                to: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("currency_pair={}", pair)));
        }
    }

    #[test]
    fn test_candlestick_deserialization() {
        let json = r#"[
            "1640995200",
            "123456.78",
            "48500.50",
            "49000.00",
            "48000.00",
            "48200.25",
            "2543.5678"
        ]"#;

        let candlestick: Vec<String> = serde_json::from_str(json).unwrap();
        assert_eq!(candlestick[0], "1640995200"); // timestamp
        assert_eq!(candlestick[1], "123456.78"); // volume
        assert_eq!(candlestick[2], "48500.50"); // close
        assert_eq!(candlestick[3], "49000.00"); // high
        assert_eq!(candlestick[4], "48000.00"); // low
        assert_eq!(candlestick[5], "48200.25"); // open
        assert_eq!(candlestick[6], "2543.5678"); // base_volume
    }

    #[test]
    fn test_candlestick_array_deserialization() {
        let json = r#"[
            [
                "1640995200",
                "123456.78",
                "48500.50",
                "49000.00",
                "48000.00",
                "48200.25",
                "2543.5678"
            ],
            [
                "1640995260",
                "234567.89",
                "48600.75",
                "48700.00",
                "48400.00",
                "48500.50",
                "4821.1234"
            ],
            [
                "1640995320",
                "345678.90",
                "48700.00",
                "48800.00",
                "48500.00",
                "48600.75",
                "7089.4567"
            ]
        ]"#;

        let candlesticks: Vec<Vec<String>> = serde_json::from_str(json).unwrap();
        assert_eq!(candlesticks.len(), 3);

        // First candlestick
        assert_eq!(candlesticks[0][0], "1640995200");
        assert_eq!(candlesticks[0][2], "48500.50");

        // Second candlestick
        assert_eq!(candlesticks[1][0], "1640995260");
        assert_eq!(candlesticks[1][2], "48600.75");

        // Third candlestick
        assert_eq!(candlesticks[2][0], "1640995320");
        assert_eq!(candlesticks[2][2], "48700.00");
    }

    #[test]
    fn test_candlestick_empty_array_deserialization() {
        let json = r#"[]"#;
        let candlesticks: Vec<Vec<String>> = serde_json::from_str(json).unwrap();
        assert_eq!(candlesticks.len(), 0);
    }

    #[test]
    fn test_candlestick_extreme_values() {
        let json = r#"[
            "9999999999",
            "999999999999.999999999",
            "999999.999999999",
            "999999.999999999",
            "0.000000001",
            "500000.123456789",
            "999999999.999999999"
        ]"#;

        let candlestick: Vec<String> = serde_json::from_str(json).unwrap();
        assert_eq!(candlestick[0], "9999999999");
        assert_eq!(candlestick[1], "999999999999.999999999");
        assert_eq!(candlestick[2], "999999.999999999");
        assert_eq!(candlestick[3], "999999.999999999");
        assert_eq!(candlestick[4], "0.000000001");
        assert_eq!(candlestick[5], "500000.123456789");
        assert_eq!(candlestick[6], "999999999.999999999");
    }

    #[test]
    fn test_candlestick_zero_values() {
        let json = r#"[
            "0",
            "0",
            "0",
            "0",
            "0",
            "0",
            "0"
        ]"#;

        let candlestick: Vec<String> = serde_json::from_str(json).unwrap();
        for value in &candlestick {
            assert_eq!(value, "0");
        }
    }

    #[test]
    fn test_candlestick_struct_deserialization() {
        let json = r#"{
            "0": "1640995200",
            "1": "123456.78",
            "2": "48500.50",
            "3": "49000.00",
            "4": "48000.00",
            "5": "48200.25",
            "6": "2543.5678"
        }"#;

        let candlestick: Candlestick = serde_json::from_str(json).unwrap();
        assert_eq!(candlestick.timestamp, "1640995200");
        assert_eq!(candlestick.volume, "123456.78");
        assert_eq!(candlestick.close, "48500.50");
        assert_eq!(candlestick.high, "49000.00");
        assert_eq!(candlestick.low, "48000.00");
        assert_eq!(candlestick.open, "48200.25");
        assert_eq!(candlestick.base_volume, "2543.5678");
    }

    #[test]
    fn test_candlestick_struct_serialization() {
        let candlestick = Candlestick {
            timestamp: "1640995200".to_string(),
            volume: "123456.78".to_string(),
            close: "48500.50".to_string(),
            high: "49000.00".to_string(),
            low: "48000.00".to_string(),
            open: "48200.25".to_string(),
            base_volume: "2543.5678".to_string(),
        };

        let json = serde_json::to_value(&candlestick).unwrap();
        assert_eq!(json["0"], "1640995200");
        assert_eq!(json["1"], "123456.78");
        assert_eq!(json["2"], "48500.50");
        assert_eq!(json["3"], "49000.00");
        assert_eq!(json["4"], "48000.00");
        assert_eq!(json["5"], "48200.25");
        assert_eq!(json["6"], "2543.5678");
    }

    #[test]
    fn test_candlestick_struct_round_trip() {
        let original = Candlestick {
            timestamp: "1640995200".to_string(),
            volume: "987654.321".to_string(),
            close: "42000.00".to_string(),
            high: "43000.00".to_string(),
            low: "41000.00".to_string(),
            open: "41500.00".to_string(),
            base_volume: "23.456789".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Candlestick = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.timestamp, original.timestamp);
        assert_eq!(deserialized.volume, original.volume);
        assert_eq!(deserialized.close, original.close);
        assert_eq!(deserialized.high, original.high);
        assert_eq!(deserialized.low, original.low);
        assert_eq!(deserialized.open, original.open);
        assert_eq!(deserialized.base_volume, original.base_volume);
    }

    #[test]
    fn test_candlestick_realistic_btc_data() {
        let json = r#"[
            [
                "1640995200",
                "12345678.90",
                "47234.56",
                "48123.45",
                "46789.01",
                "47890.12",
                "261.234567"
            ],
            [
                "1640998800",
                "23456789.01",
                "46234.56",
                "47234.56",
                "45678.90",
                "47234.56",
                "507.890123"
            ],
            [
                "1641002400",
                "34567890.12",
                "45234.56",
                "46234.56",
                "44567.89",
                "46234.56",
                "764.567890"
            ]
        ]"#;

        let candlesticks: Vec<Vec<String>> = serde_json::from_str(json).unwrap();
        assert_eq!(candlesticks.len(), 3);

        // Check timestamps are 1 hour apart (3600 seconds)
        let ts1: i64 = candlesticks[0][0].parse().unwrap();
        let ts2: i64 = candlesticks[1][0].parse().unwrap();
        let ts3: i64 = candlesticks[2][0].parse().unwrap();
        assert_eq!(ts2 - ts1, 3600);
        assert_eq!(ts3 - ts2, 3600);

        // Verify high >= low for each candlestick
        for candle in &candlesticks {
            let high: f64 = candle[3].parse().unwrap();
            let low: f64 = candle[4].parse().unwrap();
            assert!(high >= low);

            // Verify open and close are within high-low range
            let open: f64 = candle[5].parse().unwrap();
            let close: f64 = candle[2].parse().unwrap();
            assert!(open >= low && open <= high);
            assert!(close >= low && close <= high);
        }
    }

    #[test]
    fn test_candlestick_different_time_frames() {
        // 10 second candlesticks
        let request_10s = CandlesticksRequest {
            currency_pair: "BTC_USDT".to_string(),
            interval: CandlestickInterval::Seconds10,
            limit: Some(100),
            from: Some(1640995200),
            to: Some(1640996200),
        };
        let serialized = serde_urlencoded::to_string(&request_10s).unwrap();
        assert!(serialized.contains("interval=10s"));

        // 30 day candlesticks
        let request_30d = CandlesticksRequest {
            currency_pair: "BTC_USDT".to_string(),
            interval: CandlestickInterval::Days30,
            limit: Some(12),        // One year of data
            from: Some(1609459200), // 2021-01-01
            to: Some(1640995200),   // 2022-01-01
        };
        let serialized = serde_urlencoded::to_string(&request_30d).unwrap();
        assert!(serialized.contains("interval=30d"));
    }

    #[test]
    fn test_candlestick_request_default() {
        let request = CandlesticksRequest::default();
        assert_eq!(request.currency_pair, "");
        assert_eq!(request.limit, None);
        assert_eq!(request.from, None);
        assert_eq!(request.to, None);
    }

    #[test]
    fn test_candlestick_clone() {
        let original = Candlestick {
            timestamp: "1640995200".to_string(),
            volume: "123456.78".to_string(),
            close: "48500.50".to_string(),
            high: "49000.00".to_string(),
            low: "48000.00".to_string(),
            open: "48200.25".to_string(),
            base_volume: "2543.5678".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(cloned.timestamp, original.timestamp);
        assert_eq!(cloned.volume, original.volume);
        assert_eq!(cloned.close, original.close);
        assert_eq!(cloned.high, original.high);
        assert_eq!(cloned.low, original.low);
        assert_eq!(cloned.open, original.open);
        assert_eq!(cloned.base_volume, original.base_volume);
    }

    #[test]
    fn test_candlestick_debug() {
        let candlestick = Candlestick {
            timestamp: "1640995200".to_string(),
            volume: "123456.78".to_string(),
            close: "48500.50".to_string(),
            high: "49000.00".to_string(),
            low: "48000.00".to_string(),
            open: "48200.25".to_string(),
            base_volume: "2543.5678".to_string(),
        };

        let debug_str = format!("{:?}", candlestick);
        assert!(debug_str.contains("Candlestick"));
        assert!(debug_str.contains("1640995200"));
        assert!(debug_str.contains("48500.50"));
    }
}
