use std::borrow::Cow;

use reqwest::Method;
use serde::{Deserialize, Serialize};

const PREMIUM_INDEX_KLINES_ENDPOINT: &str = "/fapi/v1/premiumIndexKlines";

use super::RestClient;
use crate::binance::usdm::{KlineInterval, RestResult};

/// Request parameters for the premium index kline data endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct PremiumIndexKlinesRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Required.
    /// Must be a valid Binance USDM symbol.
    pub symbol: Cow<'static, str>,

    /// Kline interval. Required.
    /// See `KlineInterval` enum for valid values.
    pub interval: KlineInterval,

    /// Start time in milliseconds since epoch. Optional.
    /// If not provided, returns most recent klines.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds since epoch. Optional.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of klines to return. Optional.
    /// Default 500; max 1500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

/// Represents a single premium index kline bar returned by the API.
///
/// The tuple fields correspond to the array response from Binance:
/// 0: Open time (ms since epoch)
/// 1: Open price (string)
/// 2: High price (string)
/// 3: Low price (string)
/// 4: Close price (string)
/// 5: Ignore (string, always "0")
/// 6: Close time (ms since epoch)
/// 7: Ignore (string, always "0")
/// 8: Ignore (u64, count of trades)
/// 9-11: Ignore (string, always "0")
#[derive(Debug, Clone, Deserialize)]
pub struct PremiumIndexKline(
    /// Open time in milliseconds since epoch.
    pub u64,
    /// Open price as string.
    pub String,
    /// High price as string.
    pub String,
    /// Low price as string.
    pub String,
    /// Close price as string.
    pub String,
    /// Ignore field (always "0").
    pub String,
    /// Close time in milliseconds since epoch.
    pub u64,
    /// Ignore field (always "0").
    pub String,
    /// Ignore field (count of trades).
    pub u64,
    /// Ignore field (always "0").
    pub String,
    /// Ignore field (always "0").
    pub String,
    /// Ignore field (always "0").
    pub String,
);

impl RestClient {
    /// Premium Index Kline Data
    ///
    /// Premium index kline bars of a symbol. Klines are uniquely identified by their open time.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Premium-Index-Kline-Data
    ///
    /// Rate limit: based on `limit` parameter - [1,100): 1, [100,500): 2, [500,1000]: 5, >1000: 10
    ///
    /// # Arguments
    /// * `request` - Parameters for the premium index kline data endpoint
    ///
    /// # Returns
    /// Vector of `PremiumIndexKline` structs, each representing a kline bar
    pub async fn premium_index_klines(
        &self,
        request: PremiumIndexKlinesRequest,
    ) -> RestResult<Vec<PremiumIndexKline>> {
        self.send_public_request(PREMIUM_INDEX_KLINES_ENDPOINT, Method::GET, Some(request), 2)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_premium_index_klines_request_serialization() {
        let request = PremiumIndexKlinesRequest {
            symbol: "BTCUSDT".into(),
            interval: KlineInterval::I1m,
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("interval=1m"));
        assert!(serialized.contains("startTime=1625184000000"));
        assert!(serialized.contains("endTime=1625270400000"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_premium_index_klines_request_minimal() {
        let request = PremiumIndexKlinesRequest {
            symbol: "ETHUSDT".into(),
            interval: KlineInterval::I1h,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("interval=1h"));
        assert!(!serialized.contains("startTime="));
        assert!(!serialized.contains("endTime="));
        assert!(!serialized.contains("limit="));
    }

    #[test]
    fn test_premium_index_kline_deserialization() {
        let json = r#"[
            [
                1625184000000,
                "0.0003",
                "0.0004",
                "0.0002",
                "0.0003",
                "",
                1625184059999,
                "",
                0,
                "",
                "",
                ""
            ],
            [
                1625184060000,
                "0.0003",
                "0.0005",
                "0.0003",
                "0.0004",
                "",
                1625184119999,
                "",
                0,
                "",
                "",
                ""
            ]
        ]"#;

        let klines: Vec<PremiumIndexKline> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 2);

        assert_eq!(klines[0].0, 1625184000000); // open_time
        assert_eq!(klines[0].1, "0.0003"); // open
        assert_eq!(klines[0].2, "0.0004"); // high
        assert_eq!(klines[0].3, "0.0002"); // low
        assert_eq!(klines[0].4, "0.0003"); // close
        assert_eq!(klines[0].6, 1625184059999); // close_time

        assert_eq!(klines[1].0, 1625184060000);
        assert_eq!(klines[1].1, "0.0003");
        assert_eq!(klines[1].2, "0.0005");
        assert_eq!(klines[1].3, "0.0003");
        assert_eq!(klines[1].4, "0.0004");
    }

    #[test]
    fn test_premium_index_klines_different_intervals() {
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
            let request = PremiumIndexKlinesRequest {
                symbol: "BTCUSDT".into(),
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
    fn test_premium_index_kline_empty_response() {
        let json = r#"[]"#;
        let klines: Vec<PremiumIndexKline> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 0);
    }

    #[test]
    fn test_premium_index_klines_max_limit() {
        let request = PremiumIndexKlinesRequest {
            symbol: "BTCUSDT".into(),
            interval: KlineInterval::I1m,
            start_time: None,
            end_time: None,
            limit: Some(1500), // max limit
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=1500"));
    }

    #[test]
    fn test_premium_index_kline_negative_values() {
        // Premium index can be negative
        let json = r#"[
            [
                1625184000000,
                "-0.0003",
                "-0.0001",
                "-0.0005",
                "-0.0002",
                "",
                1625184059999,
                "",
                0,
                "",
                "",
                ""
            ]
        ]"#;

        let klines: Vec<PremiumIndexKline> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 1);
        assert_eq!(klines[0].1, "-0.0003");
        assert_eq!(klines[0].2, "-0.0001");
        assert_eq!(klines[0].3, "-0.0005");
        assert_eq!(klines[0].4, "-0.0002");
    }
}
