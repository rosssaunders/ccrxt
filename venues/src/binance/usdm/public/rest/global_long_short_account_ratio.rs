use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::binance::usdm::{enums::Period, public_client::RestClient};

const GLOBAL_LONG_SHORT_ACCOUNT_RATIO_ENDPOINT: &str = "/futures/data/globalLongShortAccountRatio";

/// Represents a single global long/short account ratio data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalLongShortAccountRatioResponse<'a> {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'a, str>,

    /// Long/short account number ratio of all traders.
    pub long_short_ratio: Cow<'a, str>,

    /// Long account number ratio of all traders.
    pub long_account: Cow<'a, str>,

    /// Short account number ratio of all traders.
    pub short_account: Cow<'a, str>,

    /// Timestamp in milliseconds since epoch.
    pub timestamp: u64,
}

/// Request parameters for the global long/short account ratio endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalLongShortAccountRatioRequest<'a> {
    /// The symbol to query (e.g., "BTCUSDT"). Optional - if not provided, returns data for all symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'a, str>>,

    /// The period interval. Valid values: "5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d".
    pub period: Period,

    /// Number of data points to return. Default 30, maximum 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Start time in milliseconds since epoch. Only data from the latest 30 days is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds since epoch. If not provided along with start_time, returns most recent data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
}

impl RestClient {
    /// Long/Short Ratio
    ///
    /// Query symbol Long/Short Ratio
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Long-Short-Ratio)
    ///
    /// Rate limit: IP rate limit 1000 requests/5min
    ///
    /// # Arguments
    /// * `params` - The global long/short account ratio request parameters
    ///
    /// # Returns
    /// Vector of global long/short account ratio data points
    pub async fn global_long_short_account_ratio<'a>(
        &self,
        params: GlobalLongShortAccountRatioRequest<'a>,
    ) -> crate::binance::usdm::RestResult<Vec<GlobalLongShortAccountRatioResponse<'a>>> {
        self.send_get_request(GLOBAL_LONG_SHORT_ACCOUNT_RATIO_ENDPOINT, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_long_short_account_ratio_request_serialization() {
        let request = GlobalLongShortAccountRatioRequest {
            symbol: Some("BTCUSDT".into()),
            period: Period::I5m,
            limit: Some(100),
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("period=5m"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("startTime=1625184000000"));
        assert!(serialized.contains("endTime=1625270400000"));
    }

    #[test]
    fn test_global_long_short_account_ratio_request_minimal() {
        let request = GlobalLongShortAccountRatioRequest {
            symbol: None,
            period: Period::I1h,
            limit: None,
            start_time: None,
            end_time: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(!serialized.contains("symbol="));
        assert!(serialized.contains("period=1h"));
        assert!(!serialized.contains("limit="));
        assert!(!serialized.contains("startTime="));
        assert!(!serialized.contains("endTime="));
    }

    #[test]
    fn test_global_long_short_account_ratio_response_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "longShortRatio": "1.8567",
                "longAccount": "0.6500",
                "shortAccount": "0.3500",
                "timestamp": 1625184000000
            },
            {
                "symbol": "BTCUSDT",
                "longShortRatio": "1.7234",
                "longAccount": "0.6328",
                "shortAccount": "0.3672",
                "timestamp": 1625184300000
            }
        ]"#;

        let response: Vec<GlobalLongShortAccountRatioResponse> =
            serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        assert_eq!(response[0].symbol, "BTCUSDT");
        assert_eq!(response[0].long_short_ratio, "1.8567");
        assert_eq!(response[0].long_account, "0.6500");
        assert_eq!(response[0].short_account, "0.3500");
        assert_eq!(response[0].timestamp, 1625184000000);

        assert_eq!(response[1].symbol, "BTCUSDT");
        assert_eq!(response[1].long_short_ratio, "1.7234");
        assert_eq!(response[1].long_account, "0.6328");
        assert_eq!(response[1].short_account, "0.3672");
        assert_eq!(response[1].timestamp, 1625184300000);
    }

    #[test]
    fn test_global_long_short_account_ratio_default_construction() {
        // Test that we can construct a minimal request with only required fields
        let request = GlobalLongShortAccountRatioRequest {
            symbol: None,
            period: Period::I1h,
            limit: None,
            start_time: None,
            end_time: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("period=1h"));
        assert!(!serialized.contains("symbol="));
    }

    #[test]
    fn test_global_long_short_account_ratio_market_sentiment() {
        // Test extremely bullish market
        let json = r#"[
            {
                "symbol": "BULLISH",
                "longShortRatio": "10.0000",
                "longAccount": "0.9091",
                "shortAccount": "0.0909",
                "timestamp": 1625184000000
            }
        ]"#;

        let response: Vec<GlobalLongShortAccountRatioResponse> =
            serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].long_short_ratio, "10.0000");
        assert_eq!(response[0].long_account, "0.9091");

        // Test extremely bearish market
        let json = r#"[
            {
                "symbol": "BEARISH",
                "longShortRatio": "0.1000",
                "longAccount": "0.0909",
                "shortAccount": "0.9091",
                "timestamp": 1625184000000
            }
        ]"#;

        let response: Vec<GlobalLongShortAccountRatioResponse> =
            serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].long_short_ratio, "0.1000");
        assert_eq!(response[0].short_account, "0.9091");
    }

    #[test]
    fn test_global_long_short_account_ratio_empty_response() {
        let json = r#"[]"#;
        let response: Vec<GlobalLongShortAccountRatioResponse> =
            serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }

    #[test]
    fn test_global_long_short_account_ratio_different_periods() {
        let periods = vec![
            Period::I5m,
            Period::I15m,
            Period::I30m,
            Period::I1h,
            Period::I2h,
            Period::I4h,
            Period::I6h,
            Period::I12h,
            Period::I1d,
        ];

        for period in periods {
            let request = GlobalLongShortAccountRatioRequest {
                symbol: Some("BTCUSDT".into()),
                period,
                limit: None,
                start_time: None,
                end_time: None,
            };
            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("period={}", period.as_str())));
        }
    }

    #[test]
    fn test_global_long_short_account_ratio_max_limit() {
        let request = GlobalLongShortAccountRatioRequest {
            symbol: Some("BTCUSDT".into()),
            period: Period::I1h,
            limit: Some(500), // max limit
            start_time: None,
            end_time: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=500"));
    }

    #[test]
    fn test_global_long_short_account_ratio_balanced_market() {
        let json = r#"[
            {
                "symbol": "BALANCED",
                "longShortRatio": "1.0000",
                "longAccount": "0.5000",
                "shortAccount": "0.5000",
                "timestamp": 1625184000000
            }
        ]"#;

        let response: Vec<GlobalLongShortAccountRatioResponse> =
            serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].long_short_ratio, "1.0000");
        assert_eq!(response[0].long_account, "0.5000");
        assert_eq!(response[0].short_account, "0.5000");
    }

    #[test]
    fn test_global_long_short_account_ratio_without_symbol() {
        // Test that symbol is optional and can query all symbols
        let request = GlobalLongShortAccountRatioRequest {
            symbol: None,
            period: Period::I5m,
            limit: Some(10),
            start_time: None,
            end_time: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(!serialized.contains("symbol="));
        assert!(serialized.contains("period=5m"));
        assert!(serialized.contains("limit=10"));
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(
            GLOBAL_LONG_SHORT_ACCOUNT_RATIO_ENDPOINT,
            "/futures/data/globalLongShortAccountRatio"
        );
    }
}
