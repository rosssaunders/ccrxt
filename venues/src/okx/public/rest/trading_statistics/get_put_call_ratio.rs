use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const PUT_CALL_RATIO_ENDPOINT: &str = "/api/v5/rubik/stat/option/open-interest-volume-ratio";

/// Request parameters for the get put/call ratio request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPutCallRatioRequest {
    /// Currency
    #[serde(rename = "ccy")]
    pub ccy: String,

    /// Period, the default is 8H. e.g. [8H/1D]
    /// Each granularity can only query 72 pieces of data at the earliest
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Response data for the get put/call ratio request
/// The return value array order is: [ts,oiRatio,volRatio]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCallRatioData {
    /// Timestamp of data generation time
    #[serde(rename = "ts")]
    pub ts: String,

    /// Long/Short open interest ratio
    #[serde(rename = "oiRatio")]
    pub oi_ratio: String,

    /// Long/Short trading volume ratio
    #[serde(rename = "volRatio")]
    pub vol_ratio: String,
}

impl RestClient {
    /// Get put/call ratio
    ///
    /// Retrieve the open interest ratio and trading volume ratio of calls vs puts.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-statistics-rest-api-get-put-call-ratio)
    pub async fn get_put_call_ratio(
        &self,
        request: GetPutCallRatioRequest,
    ) -> RestResult<PutCallRatioData> {
        self.send_get_request(
            PUT_CALL_RATIO_ENDPOINT,
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
    fn test_get_put_call_ratio_request_serialization() {
        let request = GetPutCallRatioRequest {
            ccy: "BTC".to_string(),
            period: Some("8H".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetPutCallRatioRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_put_call_ratio_data_deserialization_from_api() {
        let json_response = r#"{
            "ts": "1597026383085",
            "oiRatio": "0.45",
            "volRatio": "0.62"
        }"#;

        let data: PutCallRatioData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.ts, "1597026383085");
        assert_eq!(data.oi_ratio, "0.45");
        assert_eq!(data.vol_ratio, "0.62");
    }

    #[test]
    fn test_put_call_ratio_array_format() {
        // Test the array format mentioned in docs: [ts,oiRatio,volRatio]
        let json_array = r#"["1597026383085", "0.45", "0.62"]"#;
        let array_data: Vec<String> = serde_json::from_str(json_array).unwrap();

        assert_eq!(array_data.len(), 3);
        assert_eq!(array_data[0], "1597026383085");
        assert_eq!(array_data[1], "0.45");
        assert_eq!(array_data[2], "0.62");
    }
}
