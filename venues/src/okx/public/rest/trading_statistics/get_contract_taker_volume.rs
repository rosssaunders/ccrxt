use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const CONTRACT_TAKER_VOLUME_ENDPOINT: &str = "/api/v5/rubik/stat/taker-volume-contract";

/// Request parameters for the get contract taker volume request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContractTakerVolumeRequest {
    /// Instrument ID, eg: BTC-USDT-SWAP
    /// Only applicable to FUTURES, SWAP
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Bar size, the default is 5m
    /// e.g. [5m/15m/30m/1H/2H/4H]
    /// UTC+8 opening price k-line:[6H/12H/1D/2D/3D/5D/1W/1M/3M]
    /// UTC+0 opening price k-line: [6Hutc/12Hutc/1Dutc/2Dutc/3Dutc/5Dutc/1Wutc/1Mutc/3Mutc]
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,

    /// The unit of buy/sell volume, the default is 1
    /// 0: Crypto
    /// 1: Contracts
    /// 2: U
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    /// return records earlier than the requested ts
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// return records newer than the requested ts
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,

    /// Number of results per request. The maximum is 100. The default is 100.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Response data for the get contract taker volume request
/// The data returned will be arranged in an array like this: [ts, sellVol, buyVol].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractTakerVolumeData {
    /// Timestamp, millisecond format of Unix timestamp, e.g. 1597026383085
    #[serde(rename = "ts")]
    pub ts: String,

    /// taker sell volume
    #[serde(rename = "sellVol")]
    pub sell_vol: String,

    /// taker buy volume
    #[serde(rename = "buyVol")]
    pub buy_vol: String,
}

impl RestClient {
    /// Get contract taker volume
    ///
    /// Retrieve the contract taker volume for both buyers and sellers. This endpoint can retrieve the latest 1,440 data entries.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-statistics-rest-api-get-contract-taker-volume)
    pub async fn get_contract_taker_volume(
        &self,
        request: GetContractTakerVolumeRequest,
    ) -> RestResult<ContractTakerVolumeData> {
        self.send_get_request(
            CONTRACT_TAKER_VOLUME_ENDPOINT,
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
    fn test_get_contract_taker_volume_request_serialization() {
        let request = GetContractTakerVolumeRequest {
            inst_id: "BTC-USDT-SWAP".to_string(),
            period: Some("1H".to_string()),
            unit: Some("1".to_string()),
            end: Some("1597026383085".to_string()),
            begin: Some("1597026383000".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetContractTakerVolumeRequest =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_get_contract_taker_volume_request_minimal() {
        let request = GetContractTakerVolumeRequest {
            inst_id: "ETH-USDT-SWAP".to_string(),
            period: None,
            unit: None,
            end: None,
            begin: None,
            limit: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("instId"));
        assert!(!serialized.contains("period"));
        assert!(!serialized.contains("unit"));
        assert!(!serialized.contains("end"));
        assert!(!serialized.contains("begin"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_contract_taker_volume_data_serialization() {
        let data = ContractTakerVolumeData {
            ts: "1597026383085".to_string(),
            sell_vol: "1000.50".to_string(),
            buy_vol: "1500.75".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: ContractTakerVolumeData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_contract_taker_volume_data_deserialization_from_api() {
        let json_response = r#"{
            "ts": "1597026383085",
            "sellVol": "1000.50",
            "buyVol": "1500.75"
        }"#;

        let data: ContractTakerVolumeData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.ts, "1597026383085");
        assert_eq!(data.sell_vol, "1000.50");
        assert_eq!(data.buy_vol, "1500.75");
    }

    #[test]
    fn test_contract_taker_volume_array_format() {
        // Test the array format mentioned in docs: [ts, sellVol, buyVol]
        let json_array = r#"["1597026383085", "1000.50", "1500.75"]"#;
        let array_data: Vec<String> = serde_json::from_str(json_array).unwrap();

        assert_eq!(array_data.len(), 3);
        assert_eq!(array_data[0], "1597026383085");
        assert_eq!(array_data[1], "1000.50");
        assert_eq!(array_data[2], "1500.75");
    }

    #[test]
    fn test_request_with_all_period_types() {
        let periods = vec![
            "5m", "15m", "30m", "1H", "2H", "4H", "6H", "12H", "1D", "2D", "3D", "5D", "1W", "1M",
            "3M",
        ];

        for period in periods {
            let request = GetContractTakerVolumeRequest {
                inst_id: "BTC-USDT-SWAP".to_string(),
                period: Some(period.to_string()),
                unit: None,
                end: None,
                begin: None,
                limit: None,
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"period\":\"{}\"", period)));
        }
    }

    #[test]
    fn test_request_with_all_unit_types() {
        let units = vec!["0", "1", "2"];

        for unit in units {
            let request = GetContractTakerVolumeRequest {
                inst_id: "BTC-USDT-SWAP".to_string(),
                period: None,
                unit: Some(unit.to_string()),
                end: None,
                begin: None,
                limit: None,
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"unit\":\"{}\"", unit)));
        }
    }
}
