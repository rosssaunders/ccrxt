use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const LONG_SHORT_RATIO_ENDPOINT: &str = "/api/v5/rubik/stat/contracts/long-short-account-ratio";

/// Request parameters for the get long/short ratio request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLongShortRatioRequest {
    /// Currency
    #[serde(rename = "ccy")]
    pub ccy: String,

    /// Begin time, e.g. 1597026383085
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,

    /// End time, e.g. 1597026383011
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// Period, the default is 5m, e.g. [5m/1H/1D]
    /// 5m granularity can only query data within two days at most
    /// 1H granularity can only query data within 30 days at most
    /// 1D granularity can only query data within 180 days at most
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Response data for the get long/short ratio request
/// The return value array order is: [ts,ratio]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LongShortRatioData {
    /// Timestamp
    #[serde(rename = "ts")]
    pub ts: String,

    /// Long/Short ratio
    #[serde(rename = "ratio")]
    pub ratio: String,
}

impl RestClient {
    /// Get long/short ratio
    ///
    /// Retrieve the ratio of users with net long vs net short positions for Expiry Futures and Perpetual Futures.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-statistics-rest-api-get-long-short-ratio)
    pub async fn get_long_short_ratio(
        &self,
        request: GetLongShortRatioRequest,
    ) -> RestResult<LongShortRatioData> {
        self.send_get_request(
            LONG_SHORT_RATIO_ENDPOINT,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_get_long_short_ratio_request_serialization() {
        let request = GetLongShortRatioRequest {
            ccy: "BTC".to_string(),
            begin: Some("1597026383000".to_string()),
            end: Some("1597026383085".to_string()),
            period: Some("1H".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetLongShortRatioRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_long_short_ratio_data_deserialization_from_api() {
        let json_response = r#"{
            "ts": "1597026383085",
            "ratio": "0.68"
        }"#;

        let data: LongShortRatioData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.ts, "1597026383085");
        assert_eq!(data.ratio, "0.68");
    }

    #[test]
    fn test_long_short_ratio_array_format() {
        // Test the array format mentioned in docs: [ts,ratio]
        let json_array = r#"["1597026383085", "0.68"]"#;
        let array_data: Vec<String> = serde_json::from_str(json_array).unwrap();

        assert_eq!(array_data.len(), 2);
        assert_eq!(array_data[0], "1597026383085");
        assert_eq!(array_data[1], "0.68");
    }
}
