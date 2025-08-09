use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

const KLINES_ENDPOINT: &str = "/eapi/v1/klines";

/// Request parameters for klines
#[derive(Debug, Clone, Serialize)]
pub struct KlinesRequest {
    /// Option trading pair, e.g BTC-200730-9000-C
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Time interval
    #[serde(rename = "interval")]
    pub interval: String,

    /// Start time
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of records (Default: 500, Max: 1500)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Kline/candlestick data
#[derive(Debug, Clone, Deserialize)]
pub struct KlineResponse {
    /// Opening price
    #[serde(rename = "open")]
    pub open: Decimal,

    /// Highest price
    #[serde(rename = "high")]
    pub high: Decimal,

    /// Lowest price
    #[serde(rename = "low")]
    pub low: Decimal,

    /// Closing price (latest price if the current candle has not closed)
    #[serde(rename = "close")]
    pub close: Decimal,

    /// Trading volume (contracts)
    #[serde(rename = "volume")]
    pub volume: Decimal,

    /// Trading amount (in quote asset)
    #[serde(rename = "amount")]
    pub amount: Decimal,

    /// Candle type
    #[serde(rename = "interval")]
    pub interval: String,

    /// Number of completed trades
    #[serde(rename = "tradeCount")]
    pub trade_count: u64,

    /// Taker trading volume (contracts)
    #[serde(rename = "takerVolume")]
    pub taker_volume: Decimal,

    /// Taker trade amount (in quote asset)
    #[serde(rename = "takerAmount")]
    pub taker_amount: Decimal,

    /// Opening time
    #[serde(rename = "openTime")]
    pub open_time: u64,

    /// Closing time
    #[serde(rename = "closeTime")]
    pub close_time: u64,
}

impl RestClient {
    /// Get kline/candlestick data
    ///
    /// Returns kline/candlestick bars for an option symbol. Klines are uniquely identified by their open time.
    ///
    /// [docs]: (https://developers.binance.com/docs/derivatives/option/market-data/Kline-Candlestick-Data)
    /// Method: GET /eapi/v1/klines
    /// Weight: 1
    /// Security: None
    pub async fn get_klines(&self, params: KlinesRequest) -> RestResult<Vec<KlineResponse>> {
        self.send_get_request(KLINES_ENDPOINT, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_klines_request_serialization_minimal() {
        let request = KlinesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            interval: "1m".to_string(),
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTC-240329-70000-C&interval=1m");
    }

    #[test]
    fn test_klines_request_serialization_with_all_fields() {
        let request = KlinesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            interval: "5m".to_string(),
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
        assert!(serialized.contains("interval=5m"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_klines_request_serialization_with_some_fields() {
        let request = KlinesRequest {
            symbol: "ETH-240329-3000-P".to_string(),
            interval: "1h".to_string(),
            start_time: Some(1625097600000),
            end_time: None,
            limit: Some(500),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETH-240329-3000-P"));
        assert!(serialized.contains("interval=1h"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("limit=500"));
        assert!(!serialized.contains("endTime"));
    }

    #[test]
    fn test_klines_request_serialization_different_intervals() {
        let intervals = vec![
            "1m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d",
        ];

        for interval in intervals {
            let request = KlinesRequest {
                symbol: "BTC-240329-70000-C".to_string(),
                interval: interval.to_string(),
                start_time: None,
                end_time: None,
                limit: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("interval={}", interval)));
        }
    }

    #[test]
    fn test_klines_request_serialization_max_limit() {
        let request = KlinesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            interval: "1m".to_string(),
            start_time: None,
            end_time: None,
            limit: Some(1500), // Max limit
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=1500"));
    }

    #[test]
    fn test_klines_request_serialization_default_limit() {
        let request = KlinesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            interval: "1m".to_string(),
            start_time: None,
            end_time: None,
            limit: Some(500), // Default limit
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=500"));
    }

    #[test]
    fn test_klines_request_serialization_different_symbols() {
        let symbols = vec![
            "BTC-240329-70000-C",
            "BTC-240329-70000-P",
            "ETH-240329-3000-C",
            "ETH-240329-3000-P",
        ];

        for symbol in symbols {
            let request = KlinesRequest {
                symbol: symbol.to_string(),
                interval: "1m".to_string(),
                start_time: None,
                end_time: None,
                limit: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("symbol={}", symbol)));
        }
    }

    #[test]
    fn test_kline_response_deserialization() {
        let json = r#"{
            "open": "1500.00",
            "high": "1600.00",
            "low": "1450.00",
            "close": "1550.00",
            "volume": "100.50",
            "amount": "155000.00",
            "interval": "1m",
            "tradeCount": 250,
            "takerVolume": "45.25",
            "takerAmount": "70000.00",
            "openTime": 1625097600000,
            "closeTime": 1625097659999
        }"#;

        let kline: KlineResponse = serde_json::from_str(json).unwrap();
        assert_eq!(kline.open, dec!(1500.00));
        assert_eq!(kline.high, dec!(1600.00));
        assert_eq!(kline.low, dec!(1450.00));
        assert_eq!(kline.close, dec!(1550.00));
        assert_eq!(kline.volume, dec!(100.50));
        assert_eq!(kline.amount, dec!(155000.00));
        assert_eq!(kline.interval, "1m");
        assert_eq!(kline.trade_count, 250);
        assert_eq!(kline.taker_volume, dec!(45.25));
        assert_eq!(kline.taker_amount, dec!(70000.00));
        assert_eq!(kline.open_time, 1625097600000);
        assert_eq!(kline.close_time, 1625097659999);
    }

    #[test]
    fn test_kline_response_deserialization_high_precision() {
        let json = r#"{
            "open": "1500.12345678",
            "high": "1600.87654321",
            "low": "1450.11111111",
            "close": "1550.99999999",
            "volume": "100.50123456",
            "amount": "155000.78901234",
            "interval": "5m",
            "tradeCount": 1250,
            "takerVolume": "45.25987654",
            "takerAmount": "70000.12345678",
            "openTime": 1625097600000,
            "closeTime": 1625097899999
        }"#;

        let kline: KlineResponse = serde_json::from_str(json).unwrap();
        assert_eq!(kline.open.to_string(), "1500.12345678");
        assert_eq!(kline.high.to_string(), "1600.87654321");
        assert_eq!(kline.low.to_string(), "1450.11111111");
        assert_eq!(kline.close.to_string(), "1550.99999999");
        assert_eq!(kline.volume.to_string(), "100.50123456");
        assert_eq!(kline.amount.to_string(), "155000.78901234");
        assert_eq!(kline.taker_volume.to_string(), "45.25987654");
        assert_eq!(kline.taker_amount.to_string(), "70000.12345678");
    }

    #[test]
    fn test_kline_response_deserialization_zero_values() {
        let json = r#"{
            "open": "0.00000000",
            "high": "0.00000000",
            "low": "0.00000000",
            "close": "0.00000000",
            "volume": "0.00000000",
            "amount": "0.00000000",
            "interval": "1h",
            "tradeCount": 0,
            "takerVolume": "0.00000000",
            "takerAmount": "0.00000000",
            "openTime": 1625097600000,
            "closeTime": 1625101199999
        }"#;

        let kline: KlineResponse = serde_json::from_str(json).unwrap();
        assert_eq!(kline.open, dec!(0.00000000));
        assert_eq!(kline.high, dec!(0.00000000));
        assert_eq!(kline.low, dec!(0.00000000));
        assert_eq!(kline.close, dec!(0.00000000));
        assert_eq!(kline.volume, dec!(0.00000000));
        assert_eq!(kline.amount, dec!(0.00000000));
        assert_eq!(kline.trade_count, 0);
        assert_eq!(kline.taker_volume, dec!(0.00000000));
        assert_eq!(kline.taker_amount, dec!(0.00000000));
    }

    #[test]
    fn test_kline_response_deserialization_different_intervals() {
        let intervals = vec![
            "1m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d",
        ];

        for interval in intervals {
            let json = format!(
                r#"{{
                    "open": "1500.00",
                    "high": "1600.00",
                    "low": "1450.00",
                    "close": "1550.00",
                    "volume": "100.50",
                    "amount": "155000.00",
                    "interval": "{}",
                    "tradeCount": 250,
                    "takerVolume": "45.25",
                    "takerAmount": "70000.00",
                    "openTime": 1625097600000,
                    "closeTime": 1625097659999
                }}"#,
                interval
            );

            let kline: KlineResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(kline.interval, interval);
        }
    }

    #[test]
    fn test_kline_response_array_deserialization() {
        let json = r#"[
            {
                "open": "1500.00",
                "high": "1600.00",
                "low": "1450.00",
                "close": "1550.00",
                "volume": "100.50",
                "amount": "155000.00",
                "interval": "1m",
                "tradeCount": 250,
                "takerVolume": "45.25",
                "takerAmount": "70000.00",
                "openTime": 1625097600000,
                "closeTime": 1625097659999
            },
            {
                "open": "1550.00",
                "high": "1650.00",
                "low": "1520.00",
                "close": "1580.00",
                "volume": "85.75",
                "amount": "135000.00",
                "interval": "1m",
                "tradeCount": 180,
                "takerVolume": "40.30",
                "takerAmount": "63000.00",
                "openTime": 1625097660000,
                "closeTime": 1625097719999
            }
        ]"#;

        let klines: Vec<KlineResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 2);

        // First kline
        assert_eq!(klines[0].open, dec!(1500.00));
        assert_eq!(klines[0].close, dec!(1550.00));
        assert_eq!(klines[0].trade_count, 250);
        assert_eq!(klines[0].open_time, 1625097600000);

        // Second kline
        assert_eq!(klines[1].open, dec!(1550.00));
        assert_eq!(klines[1].close, dec!(1580.00));
        assert_eq!(klines[1].trade_count, 180);
        assert_eq!(klines[1].open_time, 1625097660000);
    }

    #[test]
    fn test_kline_response_empty_array_deserialization() {
        let json = r#"[]"#;
        let klines: Vec<KlineResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 0);
    }

    #[test]
    fn test_kline_response_deserialization_large_values() {
        let json = r#"{
            "open": "999999.99999999",
            "high": "1000000.00000000",
            "low": "999900.00000000",
            "close": "999950.12345678",
            "volume": "9999999.99999999",
            "amount": "9999999999.99999999",
            "interval": "1d",
            "tradeCount": 999999,
            "takerVolume": "5000000.00000000",
            "takerAmount": "4999999999.99999999",
            "openTime": 1625097600000,
            "closeTime": 1625184000000
        }"#;

        let kline: KlineResponse = serde_json::from_str(json).unwrap();
        assert_eq!(kline.open.to_string(), "999999.99999999");
        assert_eq!(kline.high.to_string(), "1000000.00000000");
        assert_eq!(kline.low.to_string(), "999900.00000000");
        assert_eq!(kline.close.to_string(), "999950.12345678");
        assert_eq!(kline.volume.to_string(), "9999999.99999999");
        assert_eq!(kline.amount.to_string(), "9999999999.99999999");
        assert_eq!(kline.trade_count, 999999);
        assert_eq!(kline.taker_volume.to_string(), "5000000.00000000");
        assert_eq!(kline.taker_amount.to_string(), "4999999999.99999999");
    }

    #[test]
    fn test_kline_response_deserialization_with_time_range() {
        let json = r#"{
            "open": "1500.00",
            "high": "1600.00",
            "low": "1450.00",
            "close": "1550.00",
            "volume": "100.50",
            "amount": "155000.00",
            "interval": "1h",
            "tradeCount": 250,
            "takerVolume": "45.25",
            "takerAmount": "70000.00",
            "openTime": 1625097600000,
            "closeTime": 1625101199999
        }"#;

        let kline: KlineResponse = serde_json::from_str(json).unwrap();
        assert_eq!(kline.open_time, 1625097600000);
        assert_eq!(kline.close_time, 1625101199999);
        assert_eq!(kline.close_time - kline.open_time, 3599999); // 1 hour - 1 ms
    }

    #[test]
    fn test_klines_request_time_range_validation() {
        let request = KlinesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            interval: "1m".to_string(),
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));

        // Verify that start_time is before end_time in the request
        assert!(request.start_time.unwrap() < request.end_time.unwrap());
    }

    #[test]
    fn test_klines_request_serialization_skips_none_values() {
        let request = KlinesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            interval: "1m".to_string(),
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(!serialized.contains("startTime"));
        assert!(!serialized.contains("endTime"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_kline_response_deserialization_consistent_ohlc() {
        let json = r#"{
            "open": "1500.00",
            "high": "1600.00",
            "low": "1450.00",
            "close": "1550.00",
            "volume": "100.50",
            "amount": "155000.00",
            "interval": "1m",
            "tradeCount": 250,
            "takerVolume": "45.25",
            "takerAmount": "70000.00",
            "openTime": 1625097600000,
            "closeTime": 1625097659999
        }"#;

        let kline: KlineResponse = serde_json::from_str(json).unwrap();

        // Verify OHLC consistency: low <= open, close <= high
        assert!(kline.low <= kline.open);
        assert!(kline.low <= kline.close);
        assert!(kline.open <= kline.high);
        assert!(kline.close <= kline.high);
    }
}
