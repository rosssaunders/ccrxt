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
#[derive(Debug, Clone)]
pub struct IndexPriceKline {
    /// Open time (milliseconds since epoch).
    pub open_time: u64,
    /// Open price as string.
    pub open: String,
    /// High price as string.
    pub high: String,
    /// Low price as string.
    pub low: String,
    /// Close price as string.
    pub close: String,
    /// Ignored field.
    pub ignore1: String,
    /// Close time (milliseconds since epoch).
    pub close_time: u64,
    /// Ignored field.
    pub ignore2: String,
    /// Ignored field.
    pub ignore3: u64,
    /// Ignored field.
    pub ignore4: String,
    /// Ignored field.
    pub ignore5: String,
    /// Ignored field.
    pub ignore6: String,
}

impl<'de> Deserialize<'de> for IndexPriceKline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let array: (
            u64,
            String,
            String,
            String,
            String,
            String,
            u64,
            String,
            u64,
            String,
            String,
            String,
        ) = Deserialize::deserialize(deserializer)?;

        Ok(IndexPriceKline {
            open_time: array.0,
            open: array.1,
            high: array.2,
            low: array.3,
            close: array.4,
            ignore1: array.5,
            close_time: array.6,
            ignore2: array.7,
            ignore3: array.8,
            ignore4: array.9,
            ignore5: array.10,
            ignore6: array.11,
        })
    }
}

impl RestClient {
    /// Index Price Kline/Candlestick Data
    ///
    /// Kline/candlestick bars for the index price of a pair. Klines are uniquely
    /// identified by their open time.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Price-Kline-Candlestick-Data)
    ///
    /// Rate limit: based on parameter LIMIT
    /// - [1,100): 1
    /// - [100, 500)[]: 2
    /// - [500, 1000][]: 5
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
        self.send_get_request(INDEX_PRICE_KLINES_ENDPOINT, Some(params), 2)
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
        assert_eq!(first_kline.open_time, 1625097600000); // Open time
        assert_eq!(first_kline.open, "45000.00"); // Open
        assert_eq!(first_kline.high, "45500.00"); // High
        assert_eq!(first_kline.low, "44800.00"); // Low
        assert_eq!(first_kline.close, "45200.00"); // Close
        assert_eq!(first_kline.ignore1, "0"); // Ignore
        assert_eq!(first_kline.close_time, 1625101199999); // Close time
        assert_eq!(first_kline.ignore2, "0"); // Ignore
        assert_eq!(first_kline.ignore3, 0); // Ignore
        assert_eq!(first_kline.ignore4, "0"); // Ignore
        assert_eq!(first_kline.ignore5, "0"); // Ignore
        assert_eq!(first_kline.ignore6, "0"); // Ignore

        let second_kline = &klines[1];
        assert_eq!(second_kline.open_time, 1625101200000);
        assert_eq!(second_kline.close, "45400.00"); // Close price
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
