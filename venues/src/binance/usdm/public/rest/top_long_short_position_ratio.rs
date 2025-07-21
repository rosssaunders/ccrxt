use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::Period;

const TOP_LONG_SHORT_POSITION_RATIO_ENDPOINT: &str = "/futures/data/topLongShortPositionRatio";

/// Request parameters for the Top Trader Long/Short Ratio (Positions) endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopLongShortPositionRatioRequest<'a> {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'a, str>,

    /// Period interval for the data points.
    pub period: Period,

    /// Number of data points to return (default 30, max 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Start time for filtering data in milliseconds since epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time for filtering data in milliseconds since epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
}

/// Represents a single data point from the Top Trader Long/Short Ratio (Positions) endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopLongShortPositionRatioResponse<'a> {
    /// Trading symbol.
    pub symbol: Cow<'a, str>,

    /// Long/short position ratio of top traders.
    pub long_short_ratio: Cow<'a, str>,

    /// Long positions ratio of top traders.
    pub long_account: Cow<'a, str>,

    /// Short positions ratio of top traders.
    pub short_account: Cow<'a, str>,

    /// Timestamp of the data point in milliseconds since epoch.
    pub timestamp: u64,
}

impl RestClient {
    /// Top Trader Long/Short Ratio (Positions)
    ///
    /// The proportion of net long and net short positions to total open positions of
    /// the top 20% users with the highest margin balance.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Top-Trader-Long-Short-Ratio
    ///
    /// Rate limit: 1000 requests/5min per IP
    ///
    /// # Arguments
    /// * `params` - The request parameters containing symbol, period, and optional filters
    ///
    /// # Returns
    /// Vector of top trader long/short ratio data points
    pub async fn top_long_short_position_ratio<'a>(
        &self,
        params: TopLongShortPositionRatioRequest<'a>,
    ) -> RestResult<Vec<TopLongShortPositionRatioResponse<'a>>> {
        self.send_public_request(
            TOP_LONG_SHORT_POSITION_RATIO_ENDPOINT,
            reqwest::Method::GET,
            Some(params),
            1,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_long_short_position_ratio_request_serialization() {
        let request = TopLongShortPositionRatioRequest {
            symbol: "BTCUSDT".into(),
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
    fn test_top_long_short_position_ratio_request_minimal() {
        let request = TopLongShortPositionRatioRequest {
            symbol: "ETHUSDT".into(),
            period: Period::I1h,
            limit: None,
            start_time: None,
            end_time: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("period=1h"));
        assert!(!serialized.contains("limit="));
        assert!(!serialized.contains("startTime="));
        assert!(!serialized.contains("endTime="));
    }

    #[test]
    fn test_top_long_short_position_ratio_response_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "longShortRatio": "1.2345",
                "longAccount": "0.5522",
                "shortAccount": "0.4478",
                "timestamp": 1625184000000
            },
            {
                "symbol": "BTCUSDT",
                "longShortRatio": "1.1890",
                "longAccount": "0.5432",
                "shortAccount": "0.4568",
                "timestamp": 1625184300000
            }
        ]"#;

        let response: Vec<TopLongShortPositionRatioResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        assert_eq!(response[0].symbol, "BTCUSDT");
        assert_eq!(response[0].long_short_ratio, "1.2345");
        assert_eq!(response[0].long_account, "0.5522");
        assert_eq!(response[0].short_account, "0.4478");
        assert_eq!(response[0].timestamp, 1625184000000);

        assert_eq!(response[1].symbol, "BTCUSDT");
        assert_eq!(response[1].long_short_ratio, "1.1890");
        assert_eq!(response[1].long_account, "0.5432");
        assert_eq!(response[1].short_account, "0.4568");
        assert_eq!(response[1].timestamp, 1625184300000);
    }

    #[test]
    fn test_top_long_short_position_ratio_high_ratio() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "longShortRatio": "5.0000",
                "longAccount": "0.8333",
                "shortAccount": "0.1667",
                "timestamp": 1625184000000
            }
        ]"#;

        let response: Vec<TopLongShortPositionRatioResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].long_short_ratio, "5.0000");
        assert_eq!(response[0].long_account, "0.8333");
        assert_eq!(response[0].short_account, "0.1667");
    }

    #[test]
    fn test_top_long_short_position_ratio_low_ratio() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "longShortRatio": "0.2000",
                "longAccount": "0.1667",
                "shortAccount": "0.8333",
                "timestamp": 1625184000000
            }
        ]"#;

        let response: Vec<TopLongShortPositionRatioResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].long_short_ratio, "0.2000");
        assert_eq!(response[0].long_account, "0.1667");
        assert_eq!(response[0].short_account, "0.8333");
    }

    #[test]
    fn test_top_long_short_position_ratio_empty_response() {
        let json = r#"[]"#;
        let response: Vec<TopLongShortPositionRatioResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }

    #[test]
    fn test_top_long_short_position_ratio_different_periods() {
        // Test with different period values
        let request_5m = TopLongShortPositionRatioRequest {
            symbol: "BTCUSDT".into(),
            period: Period::I5m,
            limit: None,
            start_time: None,
            end_time: None,
        };
        let serialized = serde_urlencoded::to_string(&request_5m).unwrap();
        assert!(serialized.contains("period=5m"));

        let request_1d = TopLongShortPositionRatioRequest {
            symbol: "BTCUSDT".into(),
            period: Period::I1d,
            limit: None,
            start_time: None,
            end_time: None,
        };
        let serialized = serde_urlencoded::to_string(&request_1d).unwrap();
        assert!(serialized.contains("period=1d"));
    }

    #[test]
    fn test_top_long_short_position_ratio_max_limit() {
        let request = TopLongShortPositionRatioRequest {
            symbol: "BTCUSDT".into(),
            period: Period::I1h,
            limit: Some(500), // max limit
            start_time: None,
            end_time: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=500"));
    }
}
