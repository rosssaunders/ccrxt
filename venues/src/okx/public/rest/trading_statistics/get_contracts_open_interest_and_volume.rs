use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const CONTRACTS_OPEN_INTEREST_AND_VOLUME_ENDPOINT: &str =
    "/api/v5/rubik/stat/contracts/open-interest-volume";

/// Request parameters for the get contracts open interest and volume request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContractsOpenInterestAndVolumeRequest {
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

/// Response data for the get contracts open interest and volume request
/// The return value array order is: [ts,oi,vol]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractsOpenInterestAndVolumeData {
    /// Timestamp
    #[serde(rename = "ts")]
    pub ts: String,

    /// Total open interest（USD）
    #[serde(rename = "oi")]
    pub oi: String,

    /// Total trading volume（USD）
    #[serde(rename = "vol")]
    pub vol: String,
}

impl RestClient {
    /// Get contracts open interest and volume
    ///
    /// Retrieve the open interest and trading volume for Expiry Futures and Perpetual Futures.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-statistics-rest-api-get-contracts-open-interest-and-volume)
    pub async fn get_contracts_open_interest_and_volume(
        &self,
        request: GetContractsOpenInterestAndVolumeRequest,
    ) -> RestResult<ContractsOpenInterestAndVolumeData> {
        self.send_get_request(
            CONTRACTS_OPEN_INTEREST_AND_VOLUME_ENDPOINT,
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
    fn test_get_contracts_open_interest_and_volume_request_serialization() {
        let request = GetContractsOpenInterestAndVolumeRequest {
            ccy: "BTC".to_string(),
            begin: Some("1597026383000".to_string()),
            end: Some("1597026383085".to_string()),
            period: Some("1H".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetContractsOpenInterestAndVolumeRequest =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_contracts_open_interest_and_volume_data_deserialization_from_api() {
        let json_response = r#"{
            "ts": "1597026383085",
            "oi": "12345.67",
            "vol": "98765.43"
        }"#;

        let data: ContractsOpenInterestAndVolumeData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.ts, "1597026383085");
        assert_eq!(data.oi, "12345.67");
        assert_eq!(data.vol, "98765.43");
    }

    #[test]
    fn test_contracts_open_interest_and_volume_array_format() {
        // Test the array format mentioned in docs: [ts,oi,vol]
        let json_array = r#"["1597026383085", "12345.67", "98765.43"]"#;
        let array_data: Vec<String> = serde_json::from_str(json_array).unwrap();

        assert_eq!(array_data.len(), 3);
        assert_eq!(array_data[0], "1597026383085");
        assert_eq!(array_data[1], "12345.67");
        assert_eq!(array_data[2], "98765.43");
    }
}
