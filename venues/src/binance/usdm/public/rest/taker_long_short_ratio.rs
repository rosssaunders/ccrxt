use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::binance::usdm::PublicRestClient as RestClient;
use crate::binance::usdm::{RestResult, enums::Period};

/// Endpoint path for Taker Buy/Sell Volume.
const TAKER_LONG_SHORT_RATIO_ENDPOINT: &str = "/futures/data/takerlongshortRatio";

/// Request parameters for the Taker Buy/Sell Volume endpoint.
///
/// All fields correspond to query parameters for the endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct TakerLongShortRatioRequest<'a> {
    /// Trading symbol to query (e.g., "BTCUSDT").
    /// Must be a valid symbol listed on Binance USDM futures.
    pub symbol: Cow<'a, str>,

    /// Period interval for aggregation. Valid values: "5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d".
    /// See [`Period`] enum for all supported intervals. This field uses an enum for type safety and fixed values.
    pub period: Period,

    /// Number of data points to return. Default: 30, Maximum: 500.
    /// Optional. If not provided, the API will use the default value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Start time in milliseconds since epoch. Optional.
    /// If not provided, the API will return the most recent data points.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds since epoch. Optional.
    /// If not provided, the API will return data up to the current time.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
}

/// Represents a single Taker Buy/Sell Volume data point returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct TakerLongShortRatioResponse<'a> {
    /// Ratio of buy volume to sell volume for the period, as a string.
    /// This value is returned as a string by the API and may contain decimal places.
    #[serde(rename = "buySellRatio")]
    pub buy_sell_ratio: Cow<'a, str>,

    /// Total buy volume for the period, as a string.
    /// This value is returned as a string by the API and may contain decimal places.
    #[serde(rename = "buyVol")]
    pub buy_vol: Cow<'a, str>,

    /// Total sell volume for the period, as a string.
    /// This value is returned as a string by the API and may contain decimal places.
    #[serde(rename = "sellVol")]
    pub sell_vol: Cow<'a, str>,

    /// Timestamp for the data point (milliseconds since epoch).
    pub timestamp: u64,
}

impl RestClient {
    /// Taker Buy/Sell Volume
    ///
    /// Taker Buy/Sell Volume
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Taker-BuySell-Volume)
    ///
    /// Rate limit: IP rate limit 1000 requests/5min
    ///
    /// # Arguments
    /// * `params` - Request parameters for the endpoint
    ///
    /// # Returns
    /// A vector of [`TakerLongShortRatioResponse`] containing buy/sell ratios and volumes for each period.
    pub async fn taker_long_short_ratio<'a>(
        &self,
        params: TakerLongShortRatioRequest<'a>,
    ) -> RestResult<Vec<TakerLongShortRatioResponse<'a>>> {
        self.send_get_request(TAKER_LONG_SHORT_RATIO_ENDPOINT, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taker_long_short_ratio_request_serialization() {
        let request = TakerLongShortRatioRequest {
            symbol: "BTCUSDT".into(),
            period: Period::I5m,
            limit: Some(100),
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("period=5m"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
    }

    #[test]
    fn test_taker_long_short_ratio_request_minimal() {
        let request = TakerLongShortRatioRequest {
            symbol: "ETHUSDT".into(),
            period: Period::I1h,
            limit: None,
            start_time: None,
            end_time: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("period=1h"));
        assert!(!serialized.contains("limit"));
        assert!(!serialized.contains("startTime"));
        assert!(!serialized.contains("endTime"));
    }

    #[test]
    fn test_taker_long_short_ratio_response_deserialization() {
        let json = r#"[
            {
                "buySellRatio": "1.2345",
                "buyVol": "123456.789",
                "sellVol": "100000.000",
                "timestamp": 1625097600000
            },
            {
                "buySellRatio": "0.8765",
                "buyVol": "87650.000",
                "sellVol": "100000.000",
                "timestamp": 1625098500000
            }
        ]"#;

        let ratios: Vec<TakerLongShortRatioResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(ratios.len(), 2);

        let first = &ratios[0];
        assert_eq!(first.buy_sell_ratio, "1.2345");
        assert_eq!(first.buy_vol, "123456.789");
        assert_eq!(first.sell_vol, "100000.000");
        assert_eq!(first.timestamp, 1625097600000);

        let second = &ratios[1];
        assert_eq!(second.buy_sell_ratio, "0.8765");
        assert_eq!(second.buy_vol, "87650.000");
        assert_eq!(second.sell_vol, "100000.000");
        assert_eq!(second.timestamp, 1625098500000);
    }

    #[test]
    fn test_taker_long_short_ratio_extreme_values() {
        let json = r#"[
            {
                "buySellRatio": "10.0000",
                "buyVol": "1000000000.000",
                "sellVol": "100000000.000",
                "timestamp": 1625097600000
            },
            {
                "buySellRatio": "0.0001",
                "buyVol": "1.000",
                "sellVol": "10000.000",
                "timestamp": 1625098500000
            }
        ]"#;

        let ratios: Vec<TakerLongShortRatioResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(ratios.len(), 2);
        assert_eq!(ratios[0].buy_sell_ratio, "10.0000");
        assert_eq!(ratios[1].buy_sell_ratio, "0.0001");
    }

    #[test]
    fn test_taker_long_short_ratio_different_periods() {
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
            let request = TakerLongShortRatioRequest {
                symbol: "BTCUSDT".into(),
                period,
                limit: None,
                start_time: None,
                end_time: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("period={}", period.as_str())));
        }
    }
}
