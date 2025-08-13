use rust_decimal::Decimal;
use serde::Serialize;

use super::client::RestClient;
use crate::binance::spot::RestResult;

const UI_KLINES_ENDPOINT: &str = "/api/v3/uiKlines";

/// Request parameters for UI klines
#[derive(Debug, Clone, Serialize)]
pub struct UiKlinesRequest {
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

/// UI Kline data - same structure as regular klines
/// Array format: [Open time, Open, High, Low, Close, Volume, Close time, Quote asset volume, Number of trades, Taker buy base asset volume, Taker buy quote asset volume, Unused field]
pub type UiKlineData = (
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
    /// Get UI kline/candlestick data
    ///
    /// The request is similar to klines having the same parameters and response.
    /// uiKlines return modified kline data, optimized for presentation of candlestick charts.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#uiklines)
    ///
    /// Method: GET /api/v3/uiKlines
    /// Weight: 2
    /// Security: None
    pub async fn get_ui_klines(&self, params: UiKlinesRequest) -> RestResult<Vec<UiKlineData>> {
        self.send_get_request(UI_KLINES_ENDPOINT, Some(params), 2)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_klines_request_serialization() {
        let request = UiKlinesRequest {
            symbol: "BTCUSDT".to_string(),
            interval: "1h".to_string(),
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
            time_zone: Some("0".to_string()),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("interval=1h"));
        assert!(serialized.contains("startTime=1625184000000"));
        assert!(serialized.contains("endTime=1625270400000"));
        assert!(serialized.contains("timeZone=0"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_ui_klines_request_minimal() {
        let request = UiKlinesRequest {
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
        assert!(!serialized.contains("timeZone"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_ui_kline_data_deserialization() {
        let json = r#"[
            1625184000000,
            "45000.00000000",
            "45500.00000000",
            "44800.00000000",
            "45200.00000000",
            "100.50000000",
            1625187600000,
            "4537000.00000000",
            5000,
            "60.30000000",
            "2724000.00000000",
            "0"
        ]"#;

        let kline: UiKlineData = serde_json::from_str(json).unwrap();
        assert_eq!(kline.0, 1625184000000); // Open time
        assert_eq!(kline.1.to_string(), "45000.00000000"); // Open
        assert_eq!(kline.2.to_string(), "45500.00000000"); // High
        assert_eq!(kline.3.to_string(), "44800.00000000"); // Low
        assert_eq!(kline.4.to_string(), "45200.00000000"); // Close
        assert_eq!(kline.5.to_string(), "100.50000000"); // Volume
        assert_eq!(kline.6, 1625187600000); // Close time
        assert_eq!(kline.7.to_string(), "4537000.00000000"); // Quote volume
        assert_eq!(kline.8, 5000); // Number of trades
        assert_eq!(kline.9.to_string(), "60.30000000"); // Taker buy base volume
        assert_eq!(kline.10.to_string(), "2724000.00000000"); // Taker buy quote volume
        assert_eq!(kline.11, "0"); // Unused field
    }

    #[test]
    fn test_ui_klines_array_deserialization() {
        let json = r#"[
            [
                1625184000000,
                "45000.00000000",
                "45500.00000000",
                "44800.00000000",
                "45200.00000000",
                "100.50000000",
                1625187600000,
                "4537000.00000000",
                5000,
                "60.30000000",
                "2724000.00000000",
                "0"
            ],
            [
                1625187600000,
                "45200.00000000",
                "45600.00000000",
                "45100.00000000",
                "45400.00000000",
                "120.75000000",
                1625191200000,
                "5475500.00000000",
                6200,
                "70.45000000",
                "3200000.00000000",
                "0"
            ]
        ]"#;

        let klines: Vec<UiKlineData> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 2);

        // First kline
        assert_eq!(klines[0].0, 1625184000000);
        assert_eq!(klines[0].4.to_string(), "45200.00000000"); // Close price

        // Second kline
        assert_eq!(klines[1].0, 1625187600000);
        assert_eq!(klines[1].4.to_string(), "45400.00000000"); // Close price
    }

    #[test]
    fn test_ui_klines_request_intervals() {
        let intervals = vec![
            "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d", "1w",
            "1M",
        ];

        for interval in intervals {
            let request = UiKlinesRequest {
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
    fn test_ui_klines_request_max_limit() {
        let request = UiKlinesRequest {
            symbol: "BTCUSDT".to_string(),
            interval: "1d".to_string(),
            start_time: None,
            end_time: None,
            time_zone: None,
            limit: Some(1000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=1000"));
    }

    #[test]
    fn test_ui_kline_data_empty_response() {
        let json = r#"[]"#;
        let klines: Vec<UiKlineData> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 0);
    }

    #[test]
    fn test_ui_klines_request_with_timezone() {
        let timezones = vec!["0", "+01:00", "-05:00", "+08:00"];

        for tz in timezones {
            let request = UiKlinesRequest {
                symbol: "ETHUSDT".to_string(),
                interval: "1h".to_string(),
                start_time: None,
                end_time: None,
                time_zone: Some(tz.to_string()),
                limit: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains("timeZone="));
        }
    }

    #[test]
    fn test_ui_kline_data_high_precision_altcoin() {
        let json = r#"[
            1625184000000,
            "0.00001234",
            "0.00001300",
            "0.00001200",
            "0.00001250",
            "10000000.00000000",
            1625187600000,
            "125.00000000",
            1500,
            "5000000.00000000",
            "62.50000000",
            "0"
        ]"#;

        let kline: UiKlineData = serde_json::from_str(json).unwrap();
        assert_eq!(kline.1.to_string(), "0.00001234"); // Open
        assert_eq!(kline.2.to_string(), "0.00001300"); // High
        assert_eq!(kline.3.to_string(), "0.00001200"); // Low
        assert_eq!(kline.4.to_string(), "0.00001250"); // Close
        assert_eq!(kline.5.to_string(), "10000000.00000000"); // Large volume
    }

    #[test]
    fn test_ui_kline_data_bullish_candle() {
        let json = r#"[
            1625184000000,
            "30000.00000000",
            "35000.00000000",
            "29500.00000000",
            "34500.00000000",
            "500.00000000",
            1625187600000,
            "16250000.00000000",
            10000,
            "300.00000000",
            "9750000.00000000",
            "0"
        ]"#;

        let kline: UiKlineData = serde_json::from_str(json).unwrap();
        let open = kline.1;
        let close = kline.4;
        assert!(close > open); // Bullish candle
        assert_eq!((close - open).to_string(), "4500.00000000");
    }

    #[test]
    fn test_ui_kline_data_bearish_candle() {
        let json = r#"[
            1625184000000,
            "35000.00000000",
            "35500.00000000",
            "30000.00000000",
            "30500.00000000",
            "750.00000000",
            1625187600000,
            "24375000.00000000",
            15000,
            "400.00000000",
            "13000000.00000000",
            "0"
        ]"#;

        let kline: UiKlineData = serde_json::from_str(json).unwrap();
        let open = kline.1;
        let close = kline.4;
        assert!(close < open); // Bearish candle
        assert_eq!((open - close).to_string(), "4500.00000000");
    }
}
