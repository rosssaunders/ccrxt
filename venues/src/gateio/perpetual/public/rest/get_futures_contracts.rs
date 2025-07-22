use serde::{Deserialize, Serialize};

use super::RestClient;
use super::contract::FuturesContract;

/// Request parameters for futures contracts
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FuturesContractsRequest {
    /// Settlement currency
    pub settle: String,
}

impl RestClient {
    /// List all futures contracts
    ///
    /// Retrieves all available futures contracts for the specified settlement currency.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#list-all-futures-contracts>
    ///
    /// # Arguments
    /// * `params` - The contracts query parameters
    ///
    /// # Returns
    /// List of futures contracts
    pub async fn get_futures_contracts(
        &self,
        params: FuturesContractsRequest,
    ) -> crate::gateio::perpetual::Result<Vec<FuturesContract>> {
        let endpoint = format!("/futures/{}/contracts", params.settle);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_contracts_request() {
        let request = FuturesContractsRequest {
            settle: "USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
    }

    #[test]
    fn test_endpoint_formatting() {
        let settlements = vec!["USDT", "BTC", "ETH"];

        for settle in settlements {
            let endpoint = format!("/futures/{}/contracts", settle);
            assert!(endpoint.contains(settle));
            assert!(endpoint.starts_with("/futures"));
            assert!(endpoint.ends_with("/contracts"));
        }
    }

    #[test]
    fn test_different_settlement_currencies() {
        let settlements = vec![
            ("USDT", "USDT settled contracts"),
            ("BTC", "BTC settled contracts"),
            ("ETH", "ETH settled contracts"),
        ];

        for (settle, _description) in settlements {
            let request = FuturesContractsRequest {
                settle: settle.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }

    #[test]
    fn test_default_implementation() {
        let request = FuturesContractsRequest::default();
        assert_eq!(request.settle, "");
    }

    #[test]
    fn test_serialization_round_trip() {
        let request = FuturesContractsRequest {
            settle: "USDT".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: FuturesContractsRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.settle, request.settle);
    }
}