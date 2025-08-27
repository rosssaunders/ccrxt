use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const OPTIONS_OPEN_INTEREST_AND_VOLUME_ENDPOINT: &str =
    "/api/v5/rubik/stat/option/open-interest-volume";

/// Request parameters for the get options open interest and volume request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOptionsOpenInterestAndVolumeRequest {
    /// Currency
    #[serde(rename = "ccy")]
    pub ccy: String,

    /// Period, the default is 8H. e.g. [8H/1D]
    /// Each granularity can only query 72 pieces of data at the earliest
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Response data for the get options open interest and volume request
/// The return value array order is: [ts,oi,vol]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionsOpenInterestAndVolumeData {
    /// Timestamp
    #[serde(rename = "ts")]
    pub ts: String,

    /// Total open interest , unit in ccy (in request parameter)
    #[serde(rename = "oi")]
    pub oi: String,

    /// Total trading volume , unit in ccy (in request parameter)
    #[serde(rename = "vol")]
    pub vol: String,
}

impl RestClient {
    /// Get options open interest and volume
    ///
    /// Retrieve the open interest and trading volume for options.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-statistics-rest-api-get-options-open-interest-and-volume)
    pub async fn get_options_open_interest_and_volume(
        &self,
        request: GetOptionsOpenInterestAndVolumeRequest,
    ) -> RestResult<OptionsOpenInterestAndVolumeData> {
        self.send_get_request(
            OPTIONS_OPEN_INTEREST_AND_VOLUME_ENDPOINT,
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
    fn test_get_options_open_interest_and_volume_request_serialization() {
        let request = GetOptionsOpenInterestAndVolumeRequest {
            ccy: "BTC".to_string(),
            period: Some("8H".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetOptionsOpenInterestAndVolumeRequest =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_options_open_interest_and_volume_data_deserialization_from_api() {
        let json_response = r#"{
            "ts": "1597026383085",
            "oi": "1250.75",
            "vol": "3456.89"
        }"#;

        let data: OptionsOpenInterestAndVolumeData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.ts, "1597026383085");
        assert_eq!(data.oi, "1250.75");
        assert_eq!(data.vol, "3456.89");
    }

    #[test]
    fn test_options_open_interest_and_volume_array_format() {
        // Test the array format mentioned in docs: [ts,oi,vol]
        let json_array = r#"["1597026383085", "1250.75", "3456.89"]"#;
        let array_data: Vec<String> = serde_json::from_str(json_array).unwrap();

        assert_eq!(array_data.len(), 3);
        assert_eq!(array_data[0], "1597026383085");
        assert_eq!(array_data[1], "1250.75");
        assert_eq!(array_data[2], "3456.89");
    }

    #[test]
    fn test_request_with_period_types() {
        let periods = vec!["8H", "1D"];

        for period in periods {
            let request = GetOptionsOpenInterestAndVolumeRequest {
                ccy: "BTC".to_string(),
                period: Some(period.to_string()),
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"period\":\"{}\"", period)));
        }
    }
}
