use serde::{Deserialize, Serialize};

/// Options candlestick data (common struct used by multiple endpoints)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsCandlestick {
    /// Timestamp
    pub t: i64,

    /// Trade volume (unit: Quote currency, unit: underlying corresponding option price)
    pub v: String,

    /// Close price (quote currency, unit: underlying corresponding option price)
    pub c: String,

    /// Highest price (quote currency, unit: underlying corresponding option price)
    pub h: String,

    /// Lowest price (quote currency, unit: underlying corresponding option price)
    pub l: String,

    /// Open price (quote currency, unit: underlying corresponding option price)
    pub o: String,
}

/// Underlying mark price candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnderlyingCandlestick {
    /// Timestamp
    pub t: i64,

    /// Close price (quote currency)
    pub c: String,

    /// Highest price (quote currency)
    pub h: String,

    /// Lowest price (quote currency)
    pub l: String,

    /// Open price (quote currency)
    pub o: String,

    /// Trading volume (unit: Quote currency)
    pub sum: Option<String>,
}

/// Request parameters for options candlesticks
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsCandlesticksRequest {
    /// Contract name
    pub contract: String,

    /// Start time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Interval time frame (10s, 1m, 5m, 15m, 30m, 1h, 4h, 8h, 1d, 7d, 30d)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}

/// Request parameters for underlying candlesticks
#[derive(Debug, Clone, Serialize, Default)]
pub struct UnderlyingCandlesticksRequest {
    /// Underlying asset
    pub underlying: String,

    /// Start time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Interval time frame (10s, 1m, 5m, 15m, 30m, 1h, 4h, 8h, 1d, 7d, 30d)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_candlestick_deserialization() {
        let json = r#"{
            "t": 1640995200,
            "v": "1000.50",
            "c": "0.08",
            "h": "0.085",
            "l": "0.075",
            "o": "0.08"
        }"#;

        let candlestick: OptionsCandlestick = serde_json::from_str(json).unwrap();
        assert_eq!(candlestick.t, 1640995200);
        assert_eq!(candlestick.v, "1000.50");
        assert_eq!(candlestick.c, "0.08");
        assert_eq!(candlestick.h, "0.085");
        assert_eq!(candlestick.l, "0.075");
        assert_eq!(candlestick.o, "0.08");
    }

    #[test]
    fn test_options_candlestick_high_precision() {
        let json = r#"{
            "t": 1640995300,
            "v": "1234.123456789",
            "c": "0.123456789",
            "h": "0.987654321",
            "l": "0.111111111",
            "o": "0.555555555"
        }"#;

        let candlestick: OptionsCandlestick = serde_json::from_str(json).unwrap();
        assert_eq!(candlestick.v, "1234.123456789");
        assert_eq!(candlestick.c, "0.123456789");
        assert_eq!(candlestick.h, "0.987654321");
        assert_eq!(candlestick.l, "0.111111111");
        assert_eq!(candlestick.o, "0.555555555");
    }

    #[test]
    fn test_underlying_candlestick_deserialization() {
        let json = r#"{
            "t": 1640995200,
            "c": "42000.50",
            "h": "42500.00",
            "l": "41500.00",
            "o": "42000.00",
            "sum": "1000000.50"
        }"#;

        let candlestick: UnderlyingCandlestick = serde_json::from_str(json).unwrap();
        assert_eq!(candlestick.t, 1640995200);
        assert_eq!(candlestick.c, "42000.50");
        assert_eq!(candlestick.h, "42500.00");
        assert_eq!(candlestick.l, "41500.00");
        assert_eq!(candlestick.o, "42000.00");
        assert_eq!(candlestick.sum, Some("1000000.50".to_string()));
    }

    #[test]
    fn test_underlying_candlestick_without_sum() {
        let json = r#"{
            "t": 1640995300,
            "c": "3000.75",
            "h": "3050.00",
            "l": "2950.00",
            "o": "3000.00"
        }"#;

        let candlestick: UnderlyingCandlestick = serde_json::from_str(json).unwrap();
        assert_eq!(candlestick.sum, None);
    }

    #[test]
    fn test_candlestick_price_validation() {
        // Bullish candlestick (close > open)
        let bullish_json = r#"{
            "t": 1640995200,
            "v": "5000.00",
            "c": "0.09",
            "h": "0.095",
            "l": "0.08",
            "o": "0.08"
        }"#;

        let bullish: OptionsCandlestick = serde_json::from_str(bullish_json).unwrap();
        let open: f64 = bullish.o.parse().unwrap();
        let close: f64 = bullish.c.parse().unwrap();
        let high: f64 = bullish.h.parse().unwrap();
        let low: f64 = bullish.l.parse().unwrap();

        assert!(close > open); // Bullish
        assert!(high >= close && high >= open); // High is highest
        assert!(low <= close && low <= open); // Low is lowest
    }

    #[test]
    fn test_options_candlesticks_request_minimal() {
        let request = OptionsCandlesticksRequest {
            contract: "BTC-20240101-50000-C".to_string(),
            from: None,
            to: None,
            limit: None,
            interval: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["contract"], "BTC-20240101-50000-C");
        assert!(!json.as_object().unwrap().contains_key("from"));
        assert!(!json.as_object().unwrap().contains_key("to"));
        assert!(!json.as_object().unwrap().contains_key("limit"));
        assert!(!json.as_object().unwrap().contains_key("interval"));
    }

    #[test]
    fn test_options_candlesticks_request_with_time_range() {
        let request = OptionsCandlesticksRequest {
            contract: "ETH-20240201-3000-P".to_string(),
            from: Some(1640995200),
            to: Some(1640995800),
            limit: None,
            interval: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["contract"], "ETH-20240201-3000-P");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
    }

    #[test]
    fn test_options_candlesticks_request_full() {
        let request = OptionsCandlesticksRequest {
            contract: "ADA-20240301-1-C".to_string(),
            from: Some(1640995200),
            to: Some(1640995800),
            limit: Some(200),
            interval: Some("5m".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["contract"], "ADA-20240301-1-C");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
        assert_eq!(json["limit"], 200);
        assert_eq!(json["interval"], "5m");
    }

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
    fn test_underlying_candlesticks_request_full() {
        let request = UnderlyingCandlesticksRequest {
            underlying: "ETH_USDT".to_string(),
            from: Some(1640995200),
            to: Some(1640995800),
            limit: Some(100),
            interval: Some("1h".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "ETH_USDT");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
        assert_eq!(json["limit"], 100);
        assert_eq!(json["interval"], "1h");
    }

    #[test]
    fn test_candlesticks_request_intervals() {
        let intervals = vec![
            "10s", "1m", "5m", "15m", "30m", "1h", "4h", "8h", "1d", "7d", "30d",
        ];

        for interval in intervals {
            let request = OptionsCandlesticksRequest {
                contract: "BTC-20240101-50000-C".to_string(),
                from: None,
                to: None,
                limit: None,
                interval: Some(interval.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["interval"], interval);
        }
    }
}
