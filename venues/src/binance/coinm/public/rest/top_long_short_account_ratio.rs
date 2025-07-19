use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, enums::Period, public::rest::RestClient};

/// Endpoint path for Top Trader Long/Short Ratio (Accounts)
const TOP_LONG_SHORT_ACCOUNT_RATIO_ENDPOINT: &str = "/futures/data/topLongShortAccountRatio";

/// Request parameters for the Top Trader Long/Short Ratio (Accounts) endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopLongShortAccountRatioRequest {
    /// Trading symbol (e.g., "BTCUSD"). Required.
    pub symbol: String,

    /// Time interval for statistics. Required. Valid values: "5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d".
    pub period: Period,

    /// Number of data points to return. Optional. Default: 30, Maximum: 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Start time in milliseconds since epoch. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End time in milliseconds since epoch. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// Represents a single top trader long/short ratio (accounts) data point.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopLongShortAccountRatio {
    /// Trading pair name (e.g., "BTCUSD").
    pub pair: String,

    /// Long account ratio (e.g., 0.6442 for 64.42%).
    pub long_account: Decimal,

    /// Short account ratio (e.g., 0.3558 for 35.58%).
    pub short_account: Decimal,

    /// Long/short ratio (e.g., 1.8105).
    pub long_short_ratio: Decimal,

    /// Timestamp in milliseconds since epoch.
    pub timestamp: i64,
}

impl RestClient {
    /// Top Trader Long/Short Ratio (Accounts)
    ///
    /// Retrieves the proportion of net long and net short accounts to total accounts of the top 20% users with the highest margin balance.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Top-Long-Short-Account-Ratio
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The request parameters for the endpoint
    ///
    /// # Returns
    /// A vector of `TopLongShortAccountRatio` data points
    pub async fn get_top_long_short_account_ratio(
        &self,
        request: TopLongShortAccountRatioRequest,
    ) -> RestResult<Vec<TopLongShortAccountRatio>> {
        self.send_request(
            TOP_LONG_SHORT_ACCOUNT_RATIO_ENDPOINT,
            reqwest::Method::GET,
            Some(request),
            1,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_request_serialization() {
        let req = TopLongShortAccountRatioRequest {
            symbol: "BTCUSD".to_string(),
            period: Period::I1h,
            limit: Some(100),
            start_time: Some(1591261042378),
            end_time: Some(1592870400000),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTCUSD"));
        assert!(json.contains("1h"));
        assert!(json.contains("limit"));
        assert!(json.contains("startTime"));
        assert!(json.contains("endTime"));
    }

    #[test]
    fn test_response_deserialization() {
        let data = r#"[
            {
                "pair": "BTCUSD",
                "longShortRatio": "1.8105",
                "longAccount": "0.6442",
                "shortAccount": "0.3558",
                "timestamp": 1591261042378
            },
            {
                "pair": "BTCUSD",
                "longShortRatio": "1.1110",
                "longAccount": "0.5263",
                "shortAccount": "0.4737",
                "timestamp": 1592870400000
            }
        ]"#;
        let parsed: Vec<TopLongShortAccountRatio> = serde_json::from_str(data).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].pair, "BTCUSD");
        assert_eq!(
            parsed[0].long_short_ratio,
            Decimal::from_str_exact("1.8105").unwrap()
        );
        assert_eq!(
            parsed[1].short_account,
            Decimal::from_str_exact("0.4737").unwrap()
        );
    }
}
