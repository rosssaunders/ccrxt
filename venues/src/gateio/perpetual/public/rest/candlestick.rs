use serde::{Deserialize, Serialize};

use crate::gateio::shared::enums::CandlestickInterval;

/// Request parameters for futures candlesticks (common struct used by multiple endpoints)
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesCandlesticksRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Interval time between data points
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<CandlestickInterval>,
    /// Start time for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// End time for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    /// Maximum number of records to return (1-1000, default 100)  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Futures candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesCandlestick {
    /// Unix timestamp in seconds
    pub t: i64,

    /// Trading volume (in quote currency)
    pub v: i64,

    /// Close price
    pub c: String,

    /// Highest price
    pub h: String,

    /// Lowest price
    pub l: String,

    /// Open price
    pub o: String,

    /// Trading volume (in base currency)
    pub sum: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_candlestick_deserialization() {
        let json = r#"{
            "t": 1640995200,
            "v": 100000,
            "c": "43250.5",
            "h": "43500.0",
            "l": "43000.0",
            "o": "43100.0",
            "sum": "2.31"
        }"#;

        let candlestick: FuturesCandlestick = serde_json::from_str(json).unwrap();
        assert_eq!(candlestick.t, 1640995200);
        assert_eq!(candlestick.v, 100000);
        assert_eq!(candlestick.c, "43250.5");
        assert_eq!(candlestick.h, "43500.0");
        assert_eq!(candlestick.l, "43000.0");
        assert_eq!(candlestick.o, "43100.0");
        assert_eq!(candlestick.sum, "2.31");
    }

    #[test]
    fn test_futures_candlesticks_request_minimal() {
        let request = FuturesCandlesticksRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            interval: None,
            from: None,
            to: None,
            limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
        assert!(!obj.contains_key("interval"));
        assert!(!obj.contains_key("from"));
        assert!(!obj.contains_key("to"));
        assert!(!obj.contains_key("limit"));
    }

    #[test]
    fn test_futures_candlesticks_request_full() {
        let request = FuturesCandlesticksRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            interval: Some(CandlestickInterval::Minutes5),
            from: Some(1640995200),
            to: Some(1640998800),
            limit: Some(100),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["interval"], "5m");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640998800);
        assert_eq!(json["limit"], 100);
    }
}
