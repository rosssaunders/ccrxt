use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const TAKER_VOLUME_ENDPOINT: &str = "/api/v5/rubik/stat/taker-volume";

/// Request parameters for the get taker volume request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTakerVolumeRequest {
    /// Currency
    #[serde(rename = "ccy")]
    pub ccy: String,

    /// Instrument type
    /// SPOT
    /// CONTRACTS
    #[serde(rename = "instType")]
    pub inst_type: String,

    /// Begin time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,

    /// End time, Unix timestamp format in milliseconds, e.g. 1597026383011
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// Period, the default is 5m, e.g. [5m/1H/1D]
    /// 5m granularity can only query data within two days at most
    /// 1H granularity can only query data within 30 days at most
    /// 1D granularity can only query data within 180 days at most
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Response data for the get taker volume request
/// The return value array order is: [ts,sellVol,buyVol]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakerVolumeData {
    /// Timestamp
    #[serde(rename = "ts")]
    pub ts: String,

    /// Sell volume
    #[serde(rename = "sellVol")]
    pub sell_vol: String,

    /// Buy volume
    #[serde(rename = "buyVol")]
    pub buy_vol: String,
}

impl RestClient {
    /// Get taker volume
    ///
    /// Retrieve the taker volume for both buyers and sellers.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-statistics-rest-api-get-taker-volume)
    pub async fn get_taker_volume(
        &self,
        request: GetTakerVolumeRequest,
    ) -> RestResult<TakerVolumeData> {
        self.send_get_request(
            TAKER_VOLUME_ENDPOINT,
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
    fn test_get_taker_volume_request_serialization() {
        let request = GetTakerVolumeRequest {
            ccy: "BTC".to_string(),
            inst_type: "SPOT".to_string(),
            begin: Some("1597026383000".to_string()),
            end: Some("1597026383085".to_string()),
            period: Some("1H".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetTakerVolumeRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_get_taker_volume_request_minimal() {
        let request = GetTakerVolumeRequest {
            ccy: "ETH".to_string(),
            inst_type: "CONTRACTS".to_string(),
            begin: None,
            end: None,
            period: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("ccy"));
        assert!(serialized.contains("instType"));
        assert!(!serialized.contains("begin"));
        assert!(!serialized.contains("end"));
        assert!(!serialized.contains("period"));
    }

    #[test]
    fn test_taker_volume_data_serialization() {
        let data = TakerVolumeData {
            ts: "1597026383085".to_string(),
            sell_vol: "1000.50".to_string(),
            buy_vol: "1500.75".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: TakerVolumeData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_taker_volume_data_deserialization_from_api() {
        let json_response = r#"{
            "ts": "1597026383085",
            "sellVol": "1000.50",
            "buyVol": "1500.75"
        }"#;

        let data: TakerVolumeData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.ts, "1597026383085");
        assert_eq!(data.sell_vol, "1000.50");
        assert_eq!(data.buy_vol, "1500.75");
    }

    #[test]
    fn test_taker_volume_array_format() {
        // Test the array format mentioned in docs: [ts,sellVol,buyVol]
        let json_array = r#"["1597026383085", "1000.50", "1500.75"]"#;
        let array_data: Vec<String> = serde_json::from_str(json_array).unwrap();

        assert_eq!(array_data.len(), 3);
        assert_eq!(array_data[0], "1597026383085");
        assert_eq!(array_data[1], "1000.50");
        assert_eq!(array_data[2], "1500.75");
    }

    #[test]
    fn test_request_with_all_period_types() {
        let periods = vec!["5m", "1H", "1D"];

        for period in periods {
            let request = GetTakerVolumeRequest {
                ccy: "BTC".to_string(),
                inst_type: "SPOT".to_string(),
                begin: None,
                end: None,
                period: Some(period.to_string()),
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"period\":\"{}\"", period)));
        }
    }

    #[test]
    fn test_request_with_all_inst_types() {
        let inst_types = vec!["SPOT", "CONTRACTS"];

        for inst_type in inst_types {
            let request = GetTakerVolumeRequest {
                ccy: "BTC".to_string(),
                inst_type: inst_type.to_string(),
                begin: None,
                end: None,
                period: None,
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"instType\":\"{}\"", inst_type)));
        }
    }
}
