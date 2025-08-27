use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const CONTRACT_OPEN_INTEREST_HISTORY_ENDPOINT: &str =
    "/api/v5/rubik/stat/contracts/open-interest-history";

/// Request parameters for the get contract open interest history request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContractOpenInterestHistoryRequest {
    /// Instrument ID, eg: BTC-USDT-SWAP
    /// Only applicable to FUTURES, SWAP
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Bar size, the default is 5m
    /// e.g. [5m/15m/30m/1H/2H/4H]
    /// UTC+8 opening price k-line: [6H/12H/1D/2D/3D/5D/1W/1M/3M]
    /// UTC+0 opening price k-line: [6Hutc/12Hutc/1Dutc/2Dutc/3Dutc/5Dutc/1Wutc/1Mutc/3Mutc]
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,

    /// Pagination of data to return records earlier than the requested ts
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// Return records newer than the requested ts
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,

    /// Number of results per request. The maximum is 100. The default is 100.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Response data for the get contract open interest history request
/// The data returned will be arranged in an array like this: [ts, oi, oiCcy, oiUsd].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractOpenInterestHistoryData {
    /// Timestamp, millisecond format of Unix timestamp, e.g. 1597026383085
    #[serde(rename = "ts")]
    pub ts: String,

    /// Open interest in the unit of contracts
    #[serde(rename = "oi")]
    pub oi: String,

    /// Open interest in the unit of crypto
    #[serde(rename = "oiCcy")]
    pub oi_ccy: String,

    /// Open interest in the unit of USD
    #[serde(rename = "oiUsd")]
    pub oi_usd: String,
}

impl RestClient {
    /// Get contract open interest history
    ///
    /// Retrieve the contract open interest statistics of futures and perp. This endpoint can retrieve the latest 1,440 data entries.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-statistics-rest-api-get-contract-open-interest-history)
    pub async fn get_contract_open_interest_history(
        &self,
        request: GetContractOpenInterestHistoryRequest,
    ) -> RestResult<ContractOpenInterestHistoryData> {
        self.send_get_request(
            CONTRACT_OPEN_INTEREST_HISTORY_ENDPOINT,
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
    fn test_get_contract_open_interest_history_request_serialization() {
        let request = GetContractOpenInterestHistoryRequest {
            inst_id: "BTC-USDT-SWAP".to_string(),
            period: Some("1H".to_string()),
            end: Some("1597026383085".to_string()),
            begin: Some("1597026383000".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetContractOpenInterestHistoryRequest =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_get_contract_open_interest_history_request_minimal() {
        let request = GetContractOpenInterestHistoryRequest {
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
    fn test_contract_open_interest_history_data_serialization() {
        let data = ContractOpenInterestHistoryData {
            ts: "1597026383085".to_string(),
            oi: "13305.55".to_string(),
            oi_ccy: "1330555".to_string(),
            oi_usd: "159763133.28".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: ContractOpenInterestHistoryData =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_contract_open_interest_history_data_deserialization_from_api() {
        let json_response = r#"{
            "ts": "1597026383085",
            "oi": "13305.55",
            "oiCcy": "1330555",
            "oiUsd": "159763133.28"
        }"#;

        let data: ContractOpenInterestHistoryData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.ts, "1597026383085");
        assert_eq!(data.oi, "13305.55");
        assert_eq!(data.oi_ccy, "1330555");
        assert_eq!(data.oi_usd, "159763133.28");
    }

    #[test]
    fn test_contract_open_interest_history_array_format() {
        // Test the array format mentioned in docs: [ts, oi, oiCcy, oiUsd]
        let json_array = r#"["1597026383085", "13305.55", "1330555", "159763133.28"]"#;
        let array_data: Vec<String> = serde_json::from_str(json_array).unwrap();

        assert_eq!(array_data.len(), 4);
        assert_eq!(array_data[0], "1597026383085");
        assert_eq!(array_data[1], "13305.55");
        assert_eq!(array_data[2], "1330555");
        assert_eq!(array_data[3], "159763133.28");
    }

    #[test]
    fn test_request_with_all_period_types() {
        let periods = vec![
            "5m", "15m", "30m", "1H", "2H", "4H", "6H", "12H", "1D", "2D", "3D", "5D", "1W", "1M",
            "3M",
        ];

        for period in periods {
            let request = GetContractOpenInterestHistoryRequest {
                inst_id: "BTC-USDT-SWAP".to_string(),
                period: Some(period.to_string()),
                end: None,
                begin: None,
                limit: None,
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"period\":\"{}\"", period)));
        }
    }
}
