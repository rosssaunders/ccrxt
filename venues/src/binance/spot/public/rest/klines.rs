use rust_decimal::Decimal;
use serde::Serialize;

use crate::binance::spot::PublicRestClient as RestClient;
use crate::binance::spot::RestResult;

const KLINES_ENDPOINT: &str = "/api/v3/klines";

/// Request parameters for klines
#[derive(Debug, Clone, Serialize)]
pub struct KlinesRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Kline interval
    #[serde(rename = "interval")]
    pub interval: String,

    /// Start time timestamp in ms
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time timestamp in ms
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Time zone (default: 0 (UTC))
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    /// Number of klines to return. Default 500, Max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Kline data
/// Array format: [Open time, Open, High, Low, Close, Volume, Close time, Quote asset volume, Number of trades, Taker buy base asset volume, Taker buy quote asset volume, Unused field]
pub type KlineData = (
    u64,     // Open time
    Decimal, // Open price
    Decimal, // High price
    Decimal, // Low price
    Decimal, // Close price
    Decimal, // Volume
    u64,     // Close time
    Decimal, // Quote asset volume
    u64,     // Number of trades
    Decimal, // Taker buy base asset volume
    Decimal, // Taker buy quote asset volume
    String,  // Unused field, ignore
);

impl RestClient {
    /// Get kline/candlestick data
    ///
    /// Returns kline/candlestick bars for a symbol.
    /// Klines are uniquely identified by their open time.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#klinecandlestick-data)
    ///
    /// Method: GET /api/v3/klines
    /// Weight: 2
    /// Security: None
    pub async fn get_klines(&self, params: KlinesRequest) -> RestResult<Vec<KlineData>> {
        self.send_get_request(KLINES_ENDPOINT, Some(params), 2)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_klines_request_serialization() {
        let request = KlinesRequest {
            symbol: "BTCUSDT".to_string(),
            interval: "1h".to_string(),
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
            time_zone: None,
            limit: Some(500),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("interval=1h"));
        assert!(serialized.contains("startTime=1625184000000"));
        assert!(serialized.contains("endTime=1625270400000"));
        assert!(serialized.contains("limit=500"));
        assert!(!serialized.contains("timeZone"));
    }

    #[test]
    fn test_klines_request_minimal() {
        let request = KlinesRequest {
            symbol: "ETHUSDT".to_string(),
            interval: "5m".to_string(),
            start_time: None,
            end_time: None,
            time_zone: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("interval=5m"));
        assert!(!serialized.contains("startTime"));
        assert!(!serialized.contains("endTime"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_klines_request_with_timezone() {
        let request = KlinesRequest {
            symbol: "BTCUSDT".to_string(),
            interval: "1d".to_string(),
            start_time: None,
            end_time: None,
            time_zone: Some("+08:00".to_string()),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timeZone=%2B08%3A00"));
    }

    #[test]
    fn test_kline_data_deserialization() {
        let json = r#"[
            1499040000000,
            "0.01634790",
            "0.80000000",
            "0.01575800",
            "0.01577100",
            "148976.11427815",
            1499644799999,
            "2434.19055334",
            308,
            "1756.87402397",
            "28.46694368",
            "0"
        ]"#;

        let kline: KlineData = serde_json::from_str(json).unwrap();
        assert_eq!(kline.0, 1499040000000); // open_time
        assert_eq!(kline.1.to_string(), "0.01634790"); // open
        assert_eq!(kline.2.to_string(), "0.80000000"); // high
        assert_eq!(kline.3.to_string(), "0.01575800"); // low
        assert_eq!(kline.4.to_string(), "0.01577100"); // close
        assert_eq!(kline.5.to_string(), "148976.11427815"); // volume
        assert_eq!(kline.6, 1499644799999); // close_time
        assert_eq!(kline.7.to_string(), "2434.19055334"); // quote_asset_volume
        assert_eq!(kline.8, 308); // number_of_trades
        assert_eq!(kline.9.to_string(), "1756.87402397"); // taker_buy_base_asset_volume
        assert_eq!(kline.10.to_string(), "28.46694368"); // taker_buy_quote_asset_volume
        assert_eq!(kline.11, "0"); // unused field
    }

    #[test]
    fn test_klines_array_deserialization() {
        let json = r#"[
            [
                1499040000000,
                "0.01634790",
                "0.80000000",
                "0.01575800",
                "0.01577100",
                "148976.11427815",
                1499644799999,
                "2434.19055334",
                308,
                "1756.87402397",
                "28.46694368",
                "0"
            ],
            [
                1499040060000,
                "0.01577100",
                "0.01577200",
                "0.01577100",
                "0.01577200",
                "4.00000000",
                1499040119999,
                "0.06308800",
                1,
                "4.00000000",
                "0.06308800",
                "0"
            ]
        ]"#;

        let klines: Vec<KlineData> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 2);

        // First kline
        assert_eq!(klines[0].0, 1499040000000);
        assert_eq!(klines[0].1.to_string(), "0.01634790");

        // Second kline
        assert_eq!(klines[1].0, 1499040060000);
        assert_eq!(klines[1].1.to_string(), "0.01577100");
    }

    #[test]
    fn test_klines_empty_response() {
        let json = r#"[]"#;
        let klines: Vec<KlineData> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 0);
    }

    #[test]
    fn test_klines_request_different_intervals() {
        let intervals = vec![
            "1s", "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d",
            "1w", "1M",
        ];

        for interval in intervals {
            let request = KlinesRequest {
                symbol: "BTCUSDT".to_string(),
                interval: interval.to_string(),
                start_time: None,
                end_time: None,
                time_zone: None,
                limit: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("interval={}", interval)));
        }
    }

    #[test]
    fn test_kline_data_high_precision() {
        let json = r#"[
            1625184000000,
            "45380.12345678",
            "45400.87654321",
            "45370.11111111",
            "45390.99999999",
            "123.45678901",
            1625187599999,
            "5600789.12345678",
            1500,
            "60.22334455",
            "2733445.66778899",
            "0"
        ]"#;

        let kline: KlineData = serde_json::from_str(json).unwrap();
        assert_eq!(kline.1.to_string(), "45380.12345678");
        assert_eq!(kline.2.to_string(), "45400.87654321");
        assert_eq!(kline.3.to_string(), "45370.11111111");
        assert_eq!(kline.4.to_string(), "45390.99999999");
        assert_eq!(kline.5.to_string(), "123.45678901");
    }

    #[test]
    fn test_klines_request_max_limit() {
        let request = KlinesRequest {
            symbol: "BTCUSDT".to_string(),
            interval: "1m".to_string(),
            start_time: None,
            end_time: None,
            time_zone: None,
            limit: Some(1000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=1000"));
    }

    #[test]
    fn test_kline_data_zero_volume() {
        let json = r#"[
            1625184000000,
            "45380.10",
            "45380.10",
            "45380.10",
            "45380.10",
            "0.00000000",
            1625187599999,
            "0.00000000",
            0,
            "0.00000000",
            "0.00000000",
            "0"
        ]"#;

        let kline: KlineData = serde_json::from_str(json).unwrap();
        assert_eq!(kline.1, kline.2); // open == high
        assert_eq!(kline.2, kline.3); // high == low
        assert_eq!(kline.3, kline.4); // low == close
        assert_eq!(kline.5.to_string(), "0.00000000");
        assert_eq!(kline.8, 0); // zero trades
    }

    #[test]
    fn test_klines_request_time_range() {
        let request = KlinesRequest {
            symbol: "ETHBTC".to_string(),
            interval: "4h".to_string(),
            start_time: Some(1609459200000), // 2021-01-01 00:00:00 UTC
            end_time: Some(1640995200000),   // 2022-01-01 00:00:00 UTC
            time_zone: None,
            limit: Some(200),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHBTC"));
        assert!(serialized.contains("interval=4h"));
        assert!(serialized.contains("startTime=1609459200000"));
        assert!(serialized.contains("endTime=1640995200000"));
        assert!(serialized.contains("limit=200"));
    }
}
