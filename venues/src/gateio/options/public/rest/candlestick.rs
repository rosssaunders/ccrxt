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
}