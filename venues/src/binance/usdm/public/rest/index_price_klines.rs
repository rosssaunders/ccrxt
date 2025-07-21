use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::{RestResult, enums::KlineInterval};

const INDEX_PRICE_KLINES_ENDPOINT: &str = "/fapi/v1/indexPriceKlines";

/// Request parameters for index price kline/candlestick data.
#[derive(Debug, Clone, Serialize)]
pub struct IndexPriceKlinesRequest {
    /// Trading pair (e.g., "BTCUSDT"). Required.
    pub pair: Cow<'static, str>,

    /// Kline interval for the candlestick data. Required.
    pub interval: KlineInterval,

    /// Start time for filtering klines (milliseconds since epoch). Optional.
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time for filtering klines (milliseconds since epoch). Optional.
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of klines to return. Default 500; max 1500. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

/// Represents a single index price kline/candlestick bar.
///
/// Klines are returned as arrays with the following structure:
/// [Open time, Open, High, Low, Close, Ignore, Close time, Ignore, Ignore, Ignore, Ignore, Ignore]
#[derive(Debug, Clone, Deserialize)]
pub struct IndexPriceKline(
    /// Open time (milliseconds since epoch).
    pub u64,
    /// Open price as string.
    pub String,
    /// High price as string.
    pub String,
    /// Low price as string.
    pub String,
    /// Close price as string.
    pub String,
    /// Ignored field.
    pub String,
    /// Close time (milliseconds since epoch).
    pub u64,
    /// Ignored field.
    pub String,
    /// Ignored field.
    pub u64,
    /// Ignored field.
    pub String,
    /// Ignored field.
    pub String,
    /// Ignored field.
    pub String,
);

impl RestClient {
    /// Index Price Kline/Candlestick Data
    ///
    /// Kline/candlestick bars for the index price of a pair. Klines are uniquely
    /// identified by their open time.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Price-Kline-Candlestick-Data
    ///
    /// Rate limit: based on parameter LIMIT
    /// - [1,100): 1
    /// - [100, 500): 2
    /// - [500, 1000]: 5
    /// - > 1000: 10
    ///
    /// # Arguments
    /// * `params` - The index price klines request parameters
    ///
    /// # Returns
    /// Vector of index price kline/candlestick data
    pub async fn get_index_price_klines(
        &self,
        params: IndexPriceKlinesRequest,
    ) -> RestResult<Vec<IndexPriceKline>> {
        self.send_public_request(
            INDEX_PRICE_KLINES_ENDPOINT,
            reqwest::Method::GET,
            Some(params),
            2,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_price_klines_request_serialization() {
        let request = IndexPriceKlinesRequest {
            pair: "BTCUSDT".into(),
            interval: KlineInterval::I1m,
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("pair=BTCUSDT"));
        assert!(serialized.contains("interval=1m"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_index_price_klines_request_minimal() {
        let request = IndexPriceKlinesRequest {
            pair: "ETHUSDT".into(),
            interval: KlineInterval::I1h,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("pair=ETHUSDT"));
        assert!(serialized.contains("interval=1h"));
        assert!(!serialized.contains("startTime"));
        assert!(!serialized.contains("endTime"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_index_price_kline_response_deserialization() {
        let json = r#"[
            [
                1625097600000,
                "45000.00",
                "45500.00",
                "44800.00",
                "45200.00",
                "0",
                1625101199999,
                "0",
                0,
                "0",
                "0",
                "0"
            ],
            [
                1625101200000,
                "45200.00",
                "45600.00",
                "45100.00",
                "45400.00",
                "0",
                1625104799999,
                "0",
                0,
                "0",
                "0",
                "0"
            ]
        ]"#;

        let klines: Vec<IndexPriceKline> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 2);

        let first_kline = &klines[0];
        assert_eq!(first_kline.0, 1625097600000); // Open time
        assert_eq!(first_kline.1, "45000.00"); // Open
        assert_eq!(first_kline.2, "45500.00"); // High
        assert_eq!(first_kline.3, "44800.00"); // Low
        assert_eq!(first_kline.4, "45200.00"); // Close
        assert_eq!(first_kline.5, "0"); // Ignore
        assert_eq!(first_kline.6, 1625101199999); // Close time
        assert_eq!(first_kline.7, "0"); // Ignore
        assert_eq!(first_kline.8, 0); // Ignore
        assert_eq!(first_kline.9, "0"); // Ignore
        assert_eq!(first_kline.10, "0"); // Ignore
        assert_eq!(first_kline.11, "0"); // Ignore

        let second_kline = &klines[1];
        assert_eq!(second_kline.0, 1625101200000);
        assert_eq!(second_kline.4, "45400.00"); // Close price
    }

    #[test]
    fn test_index_price_kline_different_intervals() {
        let intervals = vec![
            KlineInterval::I1m,
            KlineInterval::I5m,
            KlineInterval::I15m,
            KlineInterval::I30m,
            KlineInterval::I1h,
            KlineInterval::I4h,
            KlineInterval::I1d,
            KlineInterval::I1w,
            KlineInterval::I1M,
        ];

        for interval in intervals {
            let request = IndexPriceKlinesRequest {
                pair: "BTCUSDT".into(),
                interval,
                start_time: None,
                end_time: None,
                limit: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("interval={}", interval.as_str())));
        }
    }

    #[test]
    fn test_index_price_kline_max_limit() {
        let request = IndexPriceKlinesRequest {
            pair: "BTCUSDT".into(),
            interval: KlineInterval::I1m,
            start_time: None,
            end_time: None,
            limit: Some(1500), // Max limit
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=1500"));
    }
}
