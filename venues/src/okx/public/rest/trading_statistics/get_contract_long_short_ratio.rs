use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const CONTRACT_LONG_SHORT_RATIO_ENDPOINT: &str =
    "/api/v5/rubik/stat/contracts/long-short-account-ratio-contract";

/// Request parameters for the get contract long/short ratio request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContractLongShortRatioRequest {
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

/// Response data for the get contract long/short ratio request
/// The data returned will be arranged in an array like this: [ts, longAcctPosRatio].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractLongShortRatioData {
    /// Timestamp, millisecond format of Unix timestamp, e.g. 1597026383085
    #[serde(rename = "ts")]
    pub ts: String,

    /// Long/short position num ratio of all traders
    #[serde(rename = "longShortAcctRatio")]
    pub long_short_acct_ratio: String,
}

impl RestClient {
    /// Get contract long/short ratio
    ///
    /// Retrieve the account long/short ratio of a contract. This endpoint can retrieve the latest 1,440 data entries.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-statistics-rest-api-get-contract-long-short-ratio)
    pub async fn get_contract_long_short_ratio(
        &self,
        request: GetContractLongShortRatioRequest,
    ) -> RestResult<ContractLongShortRatioData> {
        self.send_get_request(
            CONTRACT_LONG_SHORT_RATIO_ENDPOINT,
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
    fn test_get_contract_long_short_ratio_request_serialization() {
        let request = GetContractLongShortRatioRequest {
            inst_id: "BTC-USDT-SWAP".to_string(),
            period: Some("1H".to_string()),
            end: Some("1597026383085".to_string()),
            begin: Some("1597026383000".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetContractLongShortRatioRequest =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_get_contract_long_short_ratio_request_minimal() {
        let request = GetContractLongShortRatioRequest {
            inst_id: "ETH-USDT-SWAP".to_string(),
            period: None,
            end: None,
            begin: None,
            limit: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("instId"));
        assert!(!serialized.contains("period"));
        assert!(!serialized.contains("end"));
        assert!(!serialized.contains("begin"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_contract_long_short_ratio_data_serialization() {
        let data = ContractLongShortRatioData {
            ts: "1597026383085".to_string(),
            long_short_acct_ratio: "0.72".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: ContractLongShortRatioData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_contract_long_short_ratio_data_deserialization_from_api() {
        let json_response = r#"{
            "ts": "1597026383085",
            "longShortAcctRatio": "0.72"
        }"#;

        let data: ContractLongShortRatioData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.ts, "1597026383085");
        assert_eq!(data.long_short_acct_ratio, "0.72");
    }

    #[test]
    fn test_contract_long_short_ratio_array_format() {
        // Test the array format mentioned in docs: [ts, longAcctPosRatio]
        let json_array = r#"["1597026383085", "0.72"]"#;
        let array_data: Vec<String> = serde_json::from_str(json_array).unwrap();

        assert_eq!(array_data.len(), 2);
        assert_eq!(array_data[0], "1597026383085");
        assert_eq!(array_data[1], "0.72");
    }
}
