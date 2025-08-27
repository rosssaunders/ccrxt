use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const MARGIN_LONG_SHORT_RATIO_ENDPOINT: &str = "/api/v5/rubik/stat/margin/loan-ratio";

/// Request parameters for the get margin long/short ratio request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMarginLongShortRatioRequest {
    /// Currency
    #[serde(rename = "ccy")]
    pub ccy: String,

    /// Begin time, e.g. 1597026383085
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,

    /// End time, e.g. 1597026383085
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// Period
    /// m: Minute, H: Hour, D: Day
    /// the default is 5m, e.g. [5m/1H/1D]
    /// 5m granularity can only query data within two days at most
    /// 1H granularity can only query data within 30 days at most
    /// 1D granularity can only query data within 180 days at most
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Response data for the get margin long/short ratio request
/// The return value array order is: [ts,ratio]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarginLongShortRatioData {
    /// Timestamp
    #[serde(rename = "ts")]
    pub ts: String,

    /// Margin lending ratio
    #[serde(rename = "ratio")]
    pub ratio: String,
}

impl RestClient {
    /// Get margin long/short ratio
    ///
    /// Retrieve the ratio of cumulative amount of quote currency to base currency.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-statistics-rest-api-get-margin-long-short-ratio)
    pub async fn get_margin_long_short_ratio(
        &self,
        request: GetMarginLongShortRatioRequest,
    ) -> RestResult<MarginLongShortRatioData> {
        self.send_get_request(
            MARGIN_LONG_SHORT_RATIO_ENDPOINT,
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
    fn test_get_margin_long_short_ratio_request_serialization() {
        let request = GetMarginLongShortRatioRequest {
            ccy: "BTC".to_string(),
            begin: Some("1597026383000".to_string()),
            end: Some("1597026383085".to_string()),
            period: Some("1H".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetMarginLongShortRatioRequest =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_get_margin_long_short_ratio_request_minimal() {
        let request = GetMarginLongShortRatioRequest {
            ccy: "ETH".to_string(),
            begin: None,
            end: None,
            period: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("ccy"));
        assert!(!serialized.contains("begin"));
        assert!(!serialized.contains("end"));
        assert!(!serialized.contains("period"));
    }

    #[test]
    fn test_margin_long_short_ratio_data_serialization() {
        let data = MarginLongShortRatioData {
            ts: "1597026383085".to_string(),
            ratio: "0.65".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: MarginLongShortRatioData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_margin_long_short_ratio_data_deserialization_from_api() {
        let json_response = r#"{
            "ts": "1597026383085",
            "ratio": "0.65"
        }"#;

        let data: MarginLongShortRatioData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.ts, "1597026383085");
        assert_eq!(data.ratio, "0.65");
    }

    #[test]
    fn test_margin_long_short_ratio_array_format() {
        // Test the array format mentioned in docs: [ts,ratio]
        let json_array = r#"["1597026383085", "0.65"]"#;
        let array_data: Vec<String> = serde_json::from_str(json_array).unwrap();

        assert_eq!(array_data.len(), 2);
        assert_eq!(array_data[0], "1597026383085");
        assert_eq!(array_data[1], "0.65");
    }

    #[test]
    fn test_request_with_all_period_types() {
        let periods = vec!["5m", "1H", "1D"];

        for period in periods {
            let request = GetMarginLongShortRatioRequest {
                ccy: "BTC".to_string(),
                begin: None,
                end: None,
                period: Some(period.to_string()),
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"period\":\"{}\"", period)));
        }
    }

    #[test]
    fn test_request_with_time_ranges() {
        let request = GetMarginLongShortRatioRequest {
            ccy: "BTC".to_string(),
            begin: Some("1597026383000".to_string()),
            end: Some("1597026383085".to_string()),
            period: Some("5m".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("\"begin\":\"1597026383000\""));
        assert!(serialized.contains("\"end\":\"1597026383085\""));
        assert!(serialized.contains("\"period\":\"5m\""));
    }
}
