use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};
use serde::{Deserialize, Serialize};

/// Request parameters for converting between contract and coin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertContractCoinRequest {
    /// Convert type
    /// 1: Convert currency to contract (default)
    /// 2: Convert contract to currency
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub convert_type: Option<String>,
    /// Instrument ID (required)
    /// Only applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Quantity to buy or sell (required)
    /// It is quantity of currency while converting currency to contract
    /// It is quantity of contract while converting contract to currency
    pub sz: String,
    /// Order price (conditional)
    /// For crypto-margined contracts, it is necessary while converting
    /// For USDT-margined contracts, it is necessary while converting between usdt and contract
    /// It is optional while converting between coin and contract
    /// For OPTION, it is optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    /// The unit of currency
    /// coin (default)
    /// usds: USDT/USDC
    /// Only applicable to USDⓈ-margined contracts from FUTURES/SWAP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Order type
    /// open: round down sz when opening positions
    /// close: round sz to the nearest when closing positions (default)
    /// Applicable to FUTURES SWAP
    #[serde(rename = "opType", skip_serializing_if = "Option::is_none")]
    pub op_type: Option<String>,
}

/// Convert contract coin data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertContractCoinData {
    /// Convert type
    /// 1: Convert currency to contract
    /// 2: Convert contract to currency
    #[serde(rename = "type")]
    pub convert_type: String,
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Order price
    pub px: String,
    /// Quantity to buy or sell
    /// It is quantity of contract while converting currency to contract
    /// It is quantity of currency while contract to currency
    pub sz: String,
    /// The unit of currency
    /// coin
    /// usds: USDT/USDC
    pub unit: String,
}

/// Response for converting between contract and coin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertContractCoinResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Convert data
    pub data: Vec<ConvertContractCoinData>,
}

impl RestClient {
    /// Convert crypto value to number of contracts, or vice versa
    ///
    /// Convert the crypto value to the number of contracts, or vice versa.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-unit-convert
    ///
    /// Rate limit: 10 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The convert contract coin request parameters
    ///
    /// # Returns
    /// Response containing the converted values
    pub async fn convert_contract_coin(
        &self,
        request: ConvertContractCoinRequest,
    ) -> RestResult<ConvertContractCoinResponse> {
        self.send_request(
            "api/v5/public/convert-contract-coin",
            reqwest::Method::GET,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_convert_contract_coin_request_minimal() {
        let request = ConvertContractCoinRequest {
            convert_type: None, // defaults to "1"
            inst_id: "BTC-USD-SWAP".to_string(),
            sz: "100".to_string(),
            px: None,
            unit: None, // defaults to "coin"
            op_type: None, // defaults to "close"
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD-SWAP")
        );
        assert_eq!(serialized.get("sz").and_then(|v| v.as_str()), Some("100"));
        assert!(serialized.get("type").is_none()); // should be omitted when None
        assert!(serialized.get("px").is_none());
        assert!(serialized.get("unit").is_none());
        assert!(serialized.get("opType").is_none());
    }

    #[test]
    fn test_convert_contract_coin_request_all_fields() {
        let request = ConvertContractCoinRequest {
            convert_type: Some("2".to_string()),
            inst_id: "BTC-USD-SWAP".to_string(),
            sz: "1".to_string(),
            px: Some("50000.0".to_string()),
            unit: Some("usds".to_string()),
            op_type: Some("open".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("type").and_then(|v| v.as_str()),
            Some("2")
        );
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD-SWAP")
        );
        assert_eq!(serialized.get("sz").and_then(|v| v.as_str()), Some("1"));
        assert_eq!(
            serialized.get("px").and_then(|v| v.as_str()),
            Some("50000.0")
        );
        assert_eq!(
            serialized.get("unit").and_then(|v| v.as_str()),
            Some("usds")
        );
        assert_eq!(
            serialized.get("opType").and_then(|v| v.as_str()),
            Some("open")
        );
    }

    #[test]
    fn test_convert_contract_coin_data_structure() {
        let data_json = json!({
            "type": "1",
            "instId": "BTC-USD-SWAP",
            "px": "50000.0",
            "sz": "100",
            "unit": "coin"
        });

        let data: ConvertContractCoinData = serde_json::from_value(data_json).unwrap();
        assert_eq!(data.convert_type, "1");
        assert_eq!(data.inst_id, "BTC-USD-SWAP");
        assert_eq!(data.px, "50000.0");
        assert_eq!(data.sz, "100");
        assert_eq!(data.unit, "coin");
    }

    #[test]
    fn test_convert_contract_coin_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "type": "1",
                    "instId": "BTC-USD-SWAP",
                    "px": "50000.0",
                    "sz": "2",
                    "unit": "coin"
                }
            ]
        });

        let response: ConvertContractCoinResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 1);
        
        let data = &response.data[0];
        assert_eq!(data.convert_type, "1");
        assert_eq!(data.inst_id, "BTC-USD-SWAP");
        assert_eq!(data.px, "50000.0");
        assert_eq!(data.sz, "2");
        assert_eq!(data.unit, "coin");
    }

    #[test]
    fn test_convert_contract_coin_serialization_roundtrip() {
        let original = ConvertContractCoinRequest {
            convert_type: Some("1".to_string()),
            inst_id: "ETH-USDT-SWAP".to_string(),
            sz: "0.1".to_string(),
            px: Some("3000.0".to_string()),
            unit: Some("coin".to_string()),
            op_type: Some("close".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: ConvertContractCoinRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.convert_type, deserialized.convert_type);
        assert_eq!(original.inst_id, deserialized.inst_id);
        assert_eq!(original.sz, deserialized.sz);
        assert_eq!(original.px, deserialized.px);
        assert_eq!(original.unit, deserialized.unit);
        assert_eq!(original.op_type, deserialized.op_type);
    }

    #[test]
    fn test_convert_contract_coin_futures_example() {
        let request = ConvertContractCoinRequest {
            convert_type: Some("1".to_string()), // currency to contract
            inst_id: "BTC-USD-240329".to_string(), // futures contract
            sz: "100".to_string(), // 100 USD
            px: Some("50000".to_string()), // price
            unit: Some("coin".to_string()),
            op_type: Some("open".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD-240329")
        );
        assert_eq!(
            serialized.get("type").and_then(|v| v.as_str()),
            Some("1")
        );
    }

    #[test]
    fn test_convert_contract_coin_swap_usdt_example() {
        let request = ConvertContractCoinRequest {
            convert_type: Some("2".to_string()), // contract to currency
            inst_id: "BTC-USDT-SWAP".to_string(),
            sz: "1".to_string(), // 1 contract
            px: Some("50000".to_string()),
            unit: Some("usds".to_string()), // USDT
            op_type: Some("close".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("unit").and_then(|v| v.as_str()),
            Some("usds")
        );
        assert_eq!(
            serialized.get("type").and_then(|v| v.as_str()),
            Some("2")
        );
    }
}