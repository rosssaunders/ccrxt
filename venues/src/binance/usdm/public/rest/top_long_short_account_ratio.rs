use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::{RestResult, enums::Period};

const TOP_LONG_SHORT_ACCOUNT_RATIO_ENDPOINT: &str = "/futures/data/topLongShortAccountRatio";

/// Request parameters for the Top Trader Long/Short Ratio (Accounts) endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct TopLongShortAccountRatioRequest<'a> {
    /// The symbol to query (e.g., "BTCUSDT").
    pub symbol: Cow<'a, str>,

    /// The period interval for data aggregation.
    /// Valid values: "5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d".
    pub period: Period,

    /// Number of data points to return.
    /// Default: 30, Maximum: 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Start time for filtering data (milliseconds since epoch).
    /// If not provided, returns most recent data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>,

    /// End time for filtering data (milliseconds since epoch).
    /// If not provided, returns most recent data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>,
}

/// Response data for a single Top Trader Long/Short Ratio (Accounts) data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopLongShortAccountRatioResponse<'a> {
    /// Trading symbol.
    pub symbol: Cow<'a, str>,

    /// Long/short account number ratio of top traders.
    /// Calculated as: Long Account % / Short Account %.
    #[serde(rename = "longShortRatio")]
    pub long_short_ratio: Cow<'a, str>,

    /// Long account number ratio of top traders.
    /// Percentage of top trader accounts with net long positions.
    #[serde(rename = "longAccount")]
    pub long_account: Cow<'a, str>,

    /// Short account number ratio of top traders.
    /// Percentage of top trader accounts with net short positions.
    #[serde(rename = "shortAccount")]
    pub short_account: Cow<'a, str>,

    /// Timestamp for this data point (milliseconds since epoch).
    pub timestamp: u64,
}

impl RestClient {
    /// Top Trader Long/Short Ratio (Accounts)
    ///
    /// Gets the proportion of net long and net short accounts to total accounts of the top
    /// 20% users with the highest margin balance. Each account is counted once only.
    /// Long Account % = Accounts of top traders with net long positions / Total accounts
    /// of top traders with open positions. Short Account % = Accounts of top traders with
    /// net short positions / Total accounts of top traders with open positions.
    /// Long/Short Ratio (Accounts) = Long Account % / Short Account %.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Top-Long-Short-Account-Ratio
    ///
    /// Rate limit: 1000 requests/5min per IP
    ///
    /// # Arguments
    /// * `params` - The request parameters including symbol, period, and optional filters
    ///
    /// # Returns
    /// Vector of top trader long/short account ratio data points
    pub async fn top_long_short_account_ratio<'a>(
        &self,
        params: TopLongShortAccountRatioRequest<'a>,
    ) -> RestResult<Vec<TopLongShortAccountRatioResponse<'a>>> {
        self.send_get_request(TOP_LONG_SHORT_ACCOUNT_RATIO_ENDPOINT, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;

    #[test]
    fn test_request_serialization() {
        let request = TopLongShortAccountRatioRequest {
            symbol: Cow::Borrowed("BTCUSDT"),
            period: Period::I5m,
            limit: Some(50),
            start_time: Some(1583139600000),
            end_time: Some(1583139900000),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("\"symbol\":\"BTCUSDT\""));
        assert!(serialized.contains("\"period\":\"5m\""));
        assert!(serialized.contains("\"limit\":50"));
        assert!(serialized.contains("\"startTime\":1583139600000"));
        assert!(serialized.contains("\"endTime\":1583139900000"));
    }

    #[test]
    fn test_request_serialization_minimal() {
        let request = TopLongShortAccountRatioRequest {
            symbol: Cow::Borrowed("ETHUSDT"),
            period: Period::I1h,
            limit: None,
            start_time: None,
            end_time: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("\"symbol\":\"ETHUSDT\""));
        assert!(serialized.contains("\"period\":\"1h\""));
        assert!(!serialized.contains("\"limit\""));
        assert!(!serialized.contains("\"startTime\""));
        assert!(!serialized.contains("\"endTime\""));
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "longShortRatio": "1.8105",
                "longAccount": "0.6442",
                "shortAccount": "0.3558",
                "timestamp": 1583139600000
            }
        ]"#;

        let responses: Vec<TopLongShortAccountRatioResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 1);

        let response = &responses[0];
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.long_short_ratio, "1.8105");
        assert_eq!(response.long_account, "0.6442");
        assert_eq!(response.short_account, "0.3558");
        assert_eq!(response.timestamp, 1583139600000);
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(
            TOP_LONG_SHORT_ACCOUNT_RATIO_ENDPOINT,
            "/futures/data/topLongShortAccountRatio"
        );
    }
}
