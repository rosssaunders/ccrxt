use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::bitmart::contract::public::rest::enums::ContractStatus;

const CONTRACT_DETAILS_ENDPOINT: &str = "/contract/public/details";

/// Request parameters for the contract details endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetContractDetailsRequest<'a> {
    /// Symbol of the contract (e.g., BTCUSDT). If not provided, returns all contracts.
    pub symbol: Option<Cow<'a, str>>,
}

/// Response from the contract details endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContractDetailsResponse {
    /// Response code from the API. 1000 indicates success.
    pub code: i32,

    /// Response message from the API.
    pub message: Cow<'static, str>,

    /// Contract details data. May be null if no contracts are found.
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContractDetailsData {
    /// List of contract details when API returns array format.
    List(Vec<ContractDetails>),

    /// Object format when API returns nested object with contracts array.
    Object { contracts: Vec<ContractDetails> },
}

/// Details of a single contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDetails {
    /// Symbol of the contract (e.g., BTCUSDT).
    pub symbol: Cow<'static, str>,

    /// Current status of the contract (e.g., Trading, Delisted).
    #[serde(default)]
    pub status: Option<ContractStatus>,

    /// Delisting time in UTC timestamp (milliseconds). Present only if contract is delisted.
    #[serde(default)]
    pub delist_time: Option<u64>,

    /// Additional fields that may be present in the API response.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

// RestClient implementation for this endpoint
use crate::bitmart::{contract::public::rest::client::RestClient, spot::error::Result};

impl RestClient {
    /// Get Contract Details
    ///
    /// Applicable to query contract details.
    ///
    /// [docs]: https://developer-pro.bitmart.com/en/futuresv2/#get-contract-details
    ///
    /// Rate limit: 12 times/2 sec
    ///
    /// # Arguments
    /// * `req` - The contract details request parameters
    ///
    /// # Returns
    /// Contract details response with symbols array
    pub async fn get_contract_details(
        &self,
        req: &GetContractDetailsRequest<'_>,
    ) -> Result<GetContractDetailsResponse> {
        let url = match &req.symbol {
            Some(symbol) => format!("{}?symbol={}", CONTRACT_DETAILS_ENDPOINT, symbol),
            None => CONTRACT_DETAILS_ENDPOINT.to_string(),
        };
        self.get(&url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_contract_details_request_default() {
        let request = GetContractDetailsRequest::default();
        assert!(request.symbol.is_none());
    }

    #[test]
    fn test_get_contract_details_request_with_symbol() {
        let request = GetContractDetailsRequest {
            symbol: Some("BTCUSDT".into()),
        };
        assert_eq!(request.symbol.as_deref(), Some("BTCUSDT"));
    }

    #[test]
    fn test_contract_details_response_serialization() {
        let response = GetContractDetailsResponse {
            code: 1000,
            message: "Ok".into(),
            data: Some(serde_json::json!([])),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        assert!(serialized.contains("\"code\":1000"));
        assert!(serialized.contains("\"message\":\"Ok\""));
    }

    #[test]
    fn test_contract_details_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "status": "Trading",
            "delist_time": 1234567890
        }"#;

        let details: ContractDetails = serde_json::from_str(json).unwrap();
        assert_eq!(details.symbol, "BTCUSDT");
        assert_eq!(details.status, Some(ContractStatus::Normal));
        assert_eq!(details.delist_time, Some(1234567890));
    }
}
