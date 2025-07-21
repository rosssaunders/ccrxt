use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::{RestResult, enums::KlineInterval};

const MARK_PRICE_KLINES_ENDPOINT: &str = "/fapi/v1/markPriceKlines";

/// Request parameters for mark price kline/candlestick data.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MarkPriceKlinesRequest {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,

    /// Kline interval for the candlestick data.
    pub interval: KlineInterval,

    /// Start time for filtering klines (milliseconds since epoch). If not provided, returns the most recent klines.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time for filtering klines (milliseconds since epoch). If not provided, returns the most recent klines.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of klines to return. Default 500, maximum 1500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

/// Represents a single mark price kline/candlestick bar.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct MarkPriceKline(
    /// Open time (milliseconds since epoch).
    pub u64,
    /// Open price as string.
    pub Cow<'static, str>,
    /// High price as string.
    pub Cow<'static, str>,
    /// Low price as string.
    pub Cow<'static, str>,
    /// Close price (or latest price) as string.
    pub Cow<'static, str>,
    /// Unused field (always empty string).
    pub Cow<'static, str>,
    /// Close time (milliseconds since epoch).
    pub u64,
    /// Unused field (always empty string).
    pub Cow<'static, str>,
    /// Unused field (always 0).
    pub u64,
    /// Unused field (always empty string).
    pub Cow<'static, str>,
    /// Unused field (always empty string).
    pub Cow<'static, str>,
    /// Unused field (always empty string).
    pub Cow<'static, str>,
);

impl RestClient {
    /// Mark Price Kline/Candlestick Data
    ///
    /// Kline/candlestick bars for the mark price of a symbol. Klines are uniquely
    /// identified by their open time.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price-Kline-Candlestick-Data
    ///
    /// Rate limit: based on parameter LIMIT - [1,100): 1, [100,500): 2, [500,1000]: 5, >1000: 10
    ///
    /// # Arguments
    /// * `params` - The mark price klines request parameters
    ///
    /// # Returns
    /// Vector of mark price kline data
    pub async fn get_mark_price_klines(
        &self,
        params: MarkPriceKlinesRequest,
    ) -> RestResult<Vec<MarkPriceKline>> {
        self.send_public_request(
            MARK_PRICE_KLINES_ENDPOINT,
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
    fn test_endpoint_constant() {
        assert_eq!(MARK_PRICE_KLINES_ENDPOINT, "/fapi/v1/markPriceKlines");
    }

    #[test]
    fn test_mark_price_kline_field_access() {
        let kline = MarkPriceKline(
            1625184000000,
            "45380.10".into(),
            "45400.20".into(),
            "45360.00".into(),
            "45390.30".into(),
            "".into(),
            1625184059999,
            "".into(),
            0,
            "".into(),
            "".into(),
            "".into(),
        );

        assert_eq!(kline.0, 1625184000000); // open_time
        assert_eq!(kline.1, "45380.10"); // open
        assert_eq!(kline.2, "45400.20"); // high
        assert_eq!(kline.3, "45360.00"); // low
        assert_eq!(kline.4, "45390.30"); // close
        assert_eq!(kline.6, 1625184059999); // close_time
    }

    #[test]
    fn test_mark_price_klines_request_serialization() {
        let request = MarkPriceKlinesRequest {
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
    fn test_mark_price_klines_request_minimal() {
        let request = MarkPriceKlinesRequest {
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
    fn test_mark_price_kline_deserialization() {
        let json = r#"[
            [
                1625184000000,
                "45380.10",
                "45400.20",
                "45360.00",
                "45390.30",
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
                "45390.30",
                "45410.50",
                "45385.10",
                "45405.40",
                "",
                1625184119999,
                "",
                0,
                "",
                "",
                ""
            ]
        ]"#;

        let klines: Vec<MarkPriceKline> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 2);

        assert_eq!(klines[0].0, 1625184000000); // open_time
        assert_eq!(klines[0].1, "45380.10"); // open
        assert_eq!(klines[0].2, "45400.20"); // high
        assert_eq!(klines[0].3, "45360.00"); // low
        assert_eq!(klines[0].4, "45390.30"); // close
        assert_eq!(klines[0].6, 1625184059999); // close_time

        assert_eq!(klines[1].0, 1625184060000);
        assert_eq!(klines[1].1, "45390.30");
        assert_eq!(klines[1].4, "45405.40");
    }

    #[test]
    fn test_mark_price_klines_different_intervals() {
        let intervals = vec![
            KlineInterval::I1m,
            KlineInterval::I3m,
            KlineInterval::I5m,
            KlineInterval::I15m,
            KlineInterval::I30m,
            KlineInterval::I1h,
            KlineInterval::I2h,
            KlineInterval::I4h,
            KlineInterval::I6h,
            KlineInterval::I8h,
            KlineInterval::I12h,
            KlineInterval::I1d,
            KlineInterval::I3d,
            KlineInterval::I1w,
            KlineInterval::I1M,
        ];

        for interval in intervals {
            let request = MarkPriceKlinesRequest {
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
    fn test_mark_price_kline_empty_response() {
        let json = r#"[]"#;
        let klines: Vec<MarkPriceKline> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 0);
    }

    #[test]
    fn test_mark_price_klines_max_limit() {
        let request = MarkPriceKlinesRequest {
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
    fn test_mark_price_kline_small_values() {
        let json = r#"[
            [
                1625184000000,
                "0.00001234",
                "0.00001240",
                "0.00001230",
                "0.00001235",
                "",
                1625184059999,
                "",
                0,
                "",
                "",
                ""
            ]
        ]"#;

        let klines: Vec<MarkPriceKline> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 1);
        assert_eq!(klines[0].1, "0.00001234");
        assert_eq!(klines[0].2, "0.00001240");
        assert_eq!(klines[0].3, "0.00001230");
        assert_eq!(klines[0].4, "0.00001235");
    }

    #[test]
    fn test_mark_price_kline_large_values() {
        let json = r#"[
            [
                1625184000000,
                "100000.00",
                "105000.00",
                "99000.00",
                "102000.00",
                "",
                1625184059999,
                "",
                0,
                "",
                "",
                ""
            ]
        ]"#;

        let klines: Vec<MarkPriceKline> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 1);
        assert_eq!(klines[0].1, "100000.00");
        assert_eq!(klines[0].2, "105000.00");
        assert_eq!(klines[0].3, "99000.00");
        assert_eq!(klines[0].4, "102000.00");
    }
}
