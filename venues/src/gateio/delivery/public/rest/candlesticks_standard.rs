use super::RestClient;
use crate::gateio::delivery::models::{DeliveryCandlestick, DeliveryCandlesticksRequest};

const DELIVERY_CANDLESTICKS_ENDPOINT: &str = "/delivery/{}/candlesticks";

impl RestClient {
    /// Get delivery candlesticks
    ///
    /// Retrieves candlestick data for a specific delivery contract.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery candlesticks request parameters
    ///
    /// # Returns
    /// List of delivery candlesticks
    pub async fn get_delivery_candlesticks(
        &self,
        params: DeliveryCandlesticksRequest,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryCandlestick>> {
        let endpoint = DELIVERY_CANDLESTICKS_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateio::shared::enums::CandlestickInterval;

    #[test]
    fn test_delivery_candlesticks_request_minimal() {
        let request = DeliveryCandlesticksRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT_20241227".to_string(),
            interval: None,
            from: None,
            to: None,
            limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT_20241227");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2); // Only settle and contract
        assert!(!obj.contains_key("interval"));
        assert!(!obj.contains_key("from"));
        assert!(!obj.contains_key("to"));
        assert!(!obj.contains_key("limit"));
    }

    #[test]
    fn test_delivery_candlesticks_request_full() {
        let request = DeliveryCandlesticksRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT_20241227".to_string(),
            interval: Some(CandlestickInterval::Minutes5),
            from: Some(1640995200),
            to: Some(1641081600),
            limit: Some(100),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT_20241227");
        assert_eq!(json["interval"], "5m");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1641081600);
        assert_eq!(json["limit"], 100);
    }

    #[test]
    fn test_candlestick_intervals() {
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
            let request = DeliveryCandlesticksRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT_20241227".to_string(),
                interval: Some(interval),
                from: None,
                to: None,
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["interval"], expected);
        }
    }

    #[test]
    fn test_delivery_candlestick_deserialization() {
        let json = r#"{
            "t": 1641024000,
            "v": 125000,
            "c": "43250.5",
            "h": "43300.0",
            "l": "43200.0",
            "o": "43225.0",
            "sum": "5400.5"
        }"#;

        let candle: DeliveryCandlestick = serde_json::from_str(json).unwrap();
        assert_eq!(candle.t, 1641024000);
        assert_eq!(candle.v, Some(125000));
        assert_eq!(candle.c, "43250.5");
        assert_eq!(candle.h, "43300.0");
        assert_eq!(candle.l, "43200.0");
        assert_eq!(candle.o, "43225.0");
        assert_eq!(candle.sum, Some("5400.5".to_string()));
    }

    #[test]
    fn test_delivery_candlestick_minimal() {
        let json = r#"{
            "t": 1641024000,
            "c": "43250.5",
            "h": "43300.0",
            "l": "43200.0",
            "o": "43225.0"
        }"#;

        let candle: DeliveryCandlestick = serde_json::from_str(json).unwrap();
        assert_eq!(candle.t, 1641024000);
        assert!(candle.v.is_none());
        assert_eq!(candle.c, "43250.5");
        assert_eq!(candle.h, "43300.0");
        assert_eq!(candle.l, "43200.0");
        assert_eq!(candle.o, "43225.0");
        assert!(candle.sum.is_none());
    }

    #[test]
    fn test_bullish_candlestick() {
        let json = r#"{
            "t": 1641024000,
            "v": 200000,
            "c": "43500.0",
            "h": "43550.0",
            "l": "43100.0",
            "o": "43150.0",
            "sum": "8650.25"
        }"#;

        let candle: DeliveryCandlestick = serde_json::from_str(json).unwrap();

        // Verify bullish candle (close > open)
        let close: f64 = candle.c.parse().unwrap();
        let open: f64 = candle.o.parse().unwrap();
        assert!(close > open);

        // Verify price range
        let high: f64 = candle.h.parse().unwrap();
        let low: f64 = candle.l.parse().unwrap();
        assert!(high >= close);
        assert!(low <= open);
        assert!(high > low);
    }

    #[test]
    fn test_bearish_candlestick() {
        let json = r#"{
            "t": 1641024000,
            "v": 180000,
            "c": "43000.0",
            "h": "43400.0",
            "l": "42950.0",
            "o": "43350.0",
            "sum": "7740.0"
        }"#;

        let candle: DeliveryCandlestick = serde_json::from_str(json).unwrap();

        // Verify bearish candle (close < open)
        let close: f64 = candle.c.parse().unwrap();
        let open: f64 = candle.o.parse().unwrap();
        assert!(close < open);

        // Verify price range
        let high: f64 = candle.h.parse().unwrap();
        let low: f64 = candle.l.parse().unwrap();
        assert!(high >= open);
        assert!(low <= close);
    }

    #[test]
    fn test_doji_candlestick() {
        let json = r#"{
            "t": 1641024000,
            "v": 50000,
            "c": "43200.0",
            "h": "43250.0",
            "l": "43150.0",
            "o": "43200.0",
            "sum": "2160.0"
        }"#;

        let candle: DeliveryCandlestick = serde_json::from_str(json).unwrap();

        // Verify doji candle (close == open)
        assert_eq!(candle.c, candle.o);

        // Verify wicks exist
        let high: f64 = candle.h.parse().unwrap();
        let low: f64 = candle.l.parse().unwrap();
        let close: f64 = candle.c.parse().unwrap();
        assert!(high > close);
        assert!(low < close);
    }

    #[test]
    fn test_serialization_round_trip() {
        let candle = DeliveryCandlestick {
            t: 1641024000,
            v: Some(125000),
            c: "43250.5".to_string(),
            h: "43300.0".to_string(),
            l: "43200.0".to_string(),
            o: "43225.0".to_string(),
            sum: Some("5400.5".to_string()),
        };

        let json = serde_json::to_string(&candle).unwrap();
        let deserialized: DeliveryCandlestick = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.t, candle.t);
        assert_eq!(deserialized.v, candle.v);
        assert_eq!(deserialized.c, candle.c);
        assert_eq!(deserialized.h, candle.h);
        assert_eq!(deserialized.l, candle.l);
        assert_eq!(deserialized.o, candle.o);
        assert_eq!(deserialized.sum, candle.sum);
    }
}
