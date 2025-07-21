//! Kline/Candlestick Data endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/klines
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Kline-Candlestick-Data)

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::{RestResult, enums::KlineInterval};

const KLINES_ENDPOINT: &str = "/fapi/v1/klines";

/// Request parameters for kline/candlestick data.
#[derive(Debug, Clone, Serialize)]
pub struct KlinesRequest {
    /// Trading pair symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,

    /// Kline interval.
    pub interval: KlineInterval,

    /// Start time in ms.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in ms.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of klines to return. Default 500; max 1500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

/// Represents a single kline/candlestick bar.
///
/// Klines are returned as arrays with the following structure:
/// [Open time, Open, High, Low, Close, Volume, Close time, Quote asset volume, Number of trades, Taker buy base asset volume, Taker buy quote asset volume, Ignore]
#[derive(Debug, Clone, Deserialize)]
pub struct Kline(
    /// Open time (milliseconds since epoch)
    pub u64,
    /// Open price
    pub String,
    /// High price
    pub String,
    /// Low price
    pub String,
    /// Close price
    pub String,
    /// Volume
    pub String,
    /// Close time (milliseconds since epoch)
    pub u64,
    /// Quote asset volume
    pub String,
    /// Number of trades
    pub u64,
    /// Taker buy base asset volume
    pub String,
    /// Taker buy quote asset volume
    pub String,
    /// Ignore field
    pub String,
);

impl RestClient {
    /// Kline/Candlestick Data
    ///
    /// Kline/candlestick bars for a symbol. Klines are uniquely identified by their open time.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Kline-Candlestick-Data
    ///
    /// Rate limit:
    /// - [1,100) klines: 1 weight
    /// - [100, 500) klines: 2 weight  
    /// - [500, 1000] klines: 5 weight
    /// - >1000 klines: 10 weight
    ///
    /// # Arguments
    /// * `params` - The klines request parameters including symbol, interval, and optional time range
    ///
    /// # Returns
    /// Vector of kline data arrays containing OHLCV and additional trading data
    pub async fn get_klines(&self, params: KlinesRequest) -> RestResult<Vec<Kline>> {
        self.send_public_request(KLINES_ENDPOINT, reqwest::Method::GET, Some(params), 2)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_klines_request_serialization() {
        let request = KlinesRequest {
            symbol: "BTCUSDT".into(),
            interval: KlineInterval::I1m,
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("interval=1m"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_klines_request_minimal() {
        let request = KlinesRequest {
            symbol: "ETHUSDT".into(),
            interval: KlineInterval::I1h,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("interval=1h"));
        // Optional fields should not be present when None due to skip_serializing_if
        assert!(!serialized.contains("startTime"));
        assert!(!serialized.contains("endTime"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_kline_response_deserialization() {
        let json = r#"[
            [
                1625097600000,
                "45000.00",
                "45500.00",
                "44800.00",
                "45200.00",
                "1250.500",
                1625101199999,
                "56475000.00",
                2500,
                "625.250",
                "28237500.00",
                "0"
            ],
            [
                1625101200000,
                "45200.00",
                "45600.00",
                "45100.00",
                "45400.00",
                "1100.300",
                1625104799999,
                "49813600.00",
                2200,
                "550.150",
                "24906800.00",
                "0"
            ]
        ]"#;

        let klines: Vec<Kline> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 2);

        let first_kline = &klines[0];
        assert_eq!(first_kline.0, 1625097600000); // Open time
        assert_eq!(first_kline.1, "45000.00"); // Open
        assert_eq!(first_kline.2, "45500.00"); // High
        assert_eq!(first_kline.3, "44800.00"); // Low
        assert_eq!(first_kline.4, "45200.00"); // Close
        assert_eq!(first_kline.5, "1250.500"); // Volume
        assert_eq!(first_kline.6, 1625101199999); // Close time
        assert_eq!(first_kline.7, "56475000.00"); // Quote asset volume
        assert_eq!(first_kline.8, 2500); // Number of trades
        assert_eq!(first_kline.9, "625.250"); // Taker buy base asset volume
        assert_eq!(first_kline.10, "28237500.00"); // Taker buy quote asset volume
        assert_eq!(first_kline.11, "0"); // Ignore

        let second_kline = &klines[1];
        assert_eq!(second_kline.0, 1625101200000);
        assert_eq!(second_kline.4, "45400.00"); // Close price
    }

    #[test]
    fn test_kline_different_intervals() {
        let intervals = vec![
            (KlineInterval::I1m, "1m"),
            (KlineInterval::I5m, "5m"),
            (KlineInterval::I15m, "15m"),
            (KlineInterval::I30m, "30m"),
            (KlineInterval::I1h, "1h"),
            (KlineInterval::I4h, "4h"),
            (KlineInterval::I1d, "1d"),
            (KlineInterval::I1w, "1w"),
            (KlineInterval::I1M, "1M"),
        ];

        for (interval, expected_str) in intervals {
            let request = KlinesRequest {
                symbol: "BTCUSDT".into(),
                interval,
                start_time: None,
                end_time: None,
                limit: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("interval={}", expected_str)));
        }
    }

    #[test]
    fn test_kline_max_limit() {
        let request = KlinesRequest {
            symbol: "BTCUSDT".into(),
            interval: KlineInterval::I1m,
            start_time: None,
            end_time: None,
            limit: Some(1500), // Max limit
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=1500"));
    }
}
