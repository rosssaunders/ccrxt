use serde::{Deserialize, Serialize};

use super::{RestClient, contract::FuturesContract};

/// Request parameters for single futures contract
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FuturesContractRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
}

impl RestClient {
    /// Get a single futures contract
    ///
    /// Retrieves detailed information about a specific futures contract.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/#get-a-single-contract)
    ///
    /// # Arguments
    /// * `params` - The contract query parameters
    ///
    /// # Returns
    /// Futures contract details
    pub async fn get_futures_contract(
        &self,
        params: FuturesContractRequest,
    ) -> crate::gateio::perpetual::RestResult<FuturesContract> {
        let endpoint = format!("/futures/{}/contracts/{}", params.settle, params.contract);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_contract_request() {
        let request = FuturesContractRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_endpoint_formatting() {
        let test_cases = vec![
            ("USDT", "BTC_USDT", "/futures/USDT/contracts/BTC_USDT"),
            ("USDT", "ETH_USDT", "/futures/USDT/contracts/ETH_USDT"),
            ("BTC", "BTC_USD", "/futures/BTC/contracts/BTC_USD"),
            ("ETH", "ETH_USD", "/futures/ETH/contracts/ETH_USD"),
        ];

        for (settle, contract, expected) in test_cases {
            let endpoint = format!("/futures/{}/contracts/{}", settle, contract);
            assert_eq!(endpoint, expected);
        }
    }

    #[test]
    fn test_various_contracts() {
        let contracts = vec![
            ("USDT", "BTC_USDT"),
            ("USDT", "ETH_USDT"),
            ("USDT", "SOL_USDT"),
            ("USDT", "MATIC_USDT"),
            ("BTC", "BTC_USD"),
            ("ETH", "ETH_USD"),
        ];

        for (settle, contract) in contracts {
            let request = FuturesContractRequest {
                settle: settle.to_string(),
                contract: contract.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_default_implementation() {
        let request = FuturesContractRequest::default();
        assert_eq!(request.settle, "");
        assert_eq!(request.contract, "");
    }

    #[test]
    fn test_serialization_round_trip() {
        let request = FuturesContractRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: FuturesContractRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.settle, request.settle);
        assert_eq!(deserialized.contract, request.contract);
    }

    #[test]
    fn test_contract_naming_patterns() {
        // USDT linear contracts
        let usdt_contracts = vec![
            "BTC_USDT",
            "ETH_USDT",
            "BNB_USDT",
            "SOL_USDT",
            "MATIC_USDT",
            "AVAX_USDT",
            "DOT_USDT",
            "LINK_USDT",
        ];

        for contract in usdt_contracts {
            let request = FuturesContractRequest {
                settle: "USDT".to_string(),
                contract: contract.to_string(),
            };

            assert!(request.contract.ends_with("_USDT"));
            assert_eq!(request.settle, "USDT");
        }

        // Inverse contracts
        let inverse_contracts = vec!["BTC_USD", "ETH_USD"];

        for contract in inverse_contracts {
            let settle = contract.split('_').next().unwrap();
            let request = FuturesContractRequest {
                settle: settle.to_string(),
                contract: contract.to_string(),
            };

            assert!(request.contract.ends_with("_USD"));
        }
    }
}
