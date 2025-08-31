use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, enums::Period, public_client::RestClient};

/// Endpoint path for Top Trader Long/Short Ratio (Positions)
const TOP_LONG_SHORT_POSITION_RATIO_ENDPOINT: &str = "/futures/data/topLongShortPositionRatio";

/// Request parameters for the Top Trader Long/Short Ratio (Positions) endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopLongShortPositionRatioRequest {
    /// Trading pair (e.g., "BTCUSD"). Required.
    pub pair: String,

    /// Time interval for statistics. Required. See [`Period`] enum for valid values.
    pub period: Period,

    /// Maximum number of results to return. Optional. Default: 30, Max: 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Start time in milliseconds since epoch. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End time in milliseconds since epoch. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// Represents a single top trader long/short ratio (positions) record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopLongShortPositionRatio {
    /// Trading pair (e.g., "BTCUSD").
    pub pair: String,

    /// Long position ratio (as a decimal, e.g., 0.6442 for 64.42%).
    pub long_position: Decimal,

    /// Short position ratio (as a decimal, e.g., 0.4404 for 44.04%).
    pub short_position: Decimal,

    /// Long/Short ratio (as a decimal, e.g., 0.7869).
    pub long_short_ratio: Decimal,

    /// Timestamp in milliseconds since epoch.
    pub timestamp: i64,
}

impl RestClient {
    /// Top Trader Long/Short Ratio (Positions)
    ///
    /// Returns the proportion of net long and net short positions to total open positions of the top 20% users with the highest margin balance.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Top-Trader-Long-Short-Ratio)
    ///
    /// Rate limit: 1 request per weight unit
    ///
    /// # Arguments
    /// * `request` - The request parameters for the endpoint
    ///
    /// # Returns
    /// A vector of [`TopLongShortPositionRatio`] records
    pub async fn get_top_long_short_position_ratio(
        &self,
        request: TopLongShortPositionRatioRequest,
    ) -> RestResult<Vec<TopLongShortPositionRatio>> {
        self.send_get_request(TOP_LONG_SHORT_POSITION_RATIO_ENDPOINT, Some(request), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_request_serialization() {
        let req = TopLongShortPositionRatioRequest {
            pair: "BTCUSD".to_string(),
            period: Period::I1h,
            limit: Some(100),
            start_time: Some(1592870400000),
            end_time: Some(1592956800000),
        };
        let encoded = serde_urlencoded::to_string(&req).unwrap();
        assert!(encoded.contains("pair=BTCUSD"));
        assert!(encoded.contains("period=1h"));
        assert!(encoded.contains("limit=100"));
        assert!(encoded.contains("startTime=1592870400000"));
        assert!(encoded.contains("endTime=1592956800000"));
    }

    #[test]
    fn test_response_deserialization() {
        let data = r#"[
            {
                "pair": "BTCUSD",
                "longShortRatio": "0.7869",
                "longPosition": "0.6442",
                "shortPosition": "0.4404",
                "timestamp": 1592870400000
            },
            {
                "pair": "BTCUSD",
                "longShortRatio": "1.1231",
                "longPosition": "0.2363",
                "shortPosition": "0.4537",
                "timestamp": 1592956800000
            }
        ]"#;
        let parsed: Vec<TopLongShortPositionRatio> = serde_json::from_str(data).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].pair, "BTCUSD");
        assert_eq!(parsed[0].long_position, dec!(0.6442));
        assert_eq!(parsed[0].short_position, dec!(0.4404));
        assert_eq!(parsed[0].long_short_ratio, dec!(0.7869));
        assert_eq!(parsed[0].timestamp, 1592870400000);
    }
}
