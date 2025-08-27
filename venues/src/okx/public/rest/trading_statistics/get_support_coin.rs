use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const SUPPORT_COIN_ENDPOINT: &str = "/api/v5/rubik/stat/trading-data/support-coin";

/// Response data for the get support coin request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportCoinData {
    /// Currency supported by derivatives trading data
    #[serde(rename = "contract")]
    pub contract: Vec<String>,

    /// Currency supported by option trading data
    #[serde(rename = "option")]
    pub option: Vec<String>,

    /// Currency supported by spot trading data
    #[serde(rename = "spot")]
    pub spot: Vec<String>,
}

impl RestClient {
    /// Get support coin
    ///
    /// Retrieve the currencies supported by the trading statistics endpoints.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-statistics-rest-api-get-support-coin)
    pub async fn get_support_coin(&self) -> RestResult<SupportCoinData> {
        self.send_get_request(
            SUPPORT_COIN_ENDPOINT,
            None::<&()>,
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
    fn test_support_coin_data_serialization() {
        let data = SupportCoinData {
            contract: vec!["BTC".to_string(), "ETH".to_string()],
            option: vec!["BTC".to_string()],
            spot: vec!["BTC".to_string(), "ETH".to_string(), "LTC".to_string()],
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: SupportCoinData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_support_coin_data_deserialization_from_api() {
        let json_response = r#"{
            "contract": ["BTC", "ETH", "LTC"],
            "option": ["BTC", "ETH"],
            "spot": ["BTC", "ETH", "LTC", "ADA", "DOT"]
        }"#;

        let data: SupportCoinData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.contract.len(), 3);
        assert_eq!(data.option.len(), 2);
        assert_eq!(data.spot.len(), 5);
        assert!(data.contract.contains(&"BTC".to_string()));
        assert!(data.option.contains(&"BTC".to_string()));
        assert!(data.spot.contains(&"BTC".to_string()));
    }

    #[test]
    fn test_support_coin_data_empty_arrays() {
        let json_response = r#"{
            "contract": [],
            "option": [],
            "spot": []
        }"#;

        let data: SupportCoinData = serde_json::from_str(json_response).unwrap();
        assert!(data.contract.is_empty());
        assert!(data.option.is_empty());
        assert!(data.spot.is_empty());
    }
}
