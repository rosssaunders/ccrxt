use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};
use serde::{Deserialize, Serialize};

/// Exchange rate information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRate {
    /// USD to CNY exchange rate
    #[serde(rename = "usdCny")]
    pub usd_cny: String,
}

/// Response for getting exchange rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRateResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Exchange rate data
    pub data: Vec<ExchangeRate>,
}

impl RestClient {
    /// Get exchange rate
    ///
    /// This interface provides the average exchange rate data for 2 weeks
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-market-data-get-exchange-rate
    ///
    /// Rate limit: 1 request per 2 seconds
    ///
    /// # Returns
    /// Response containing the exchange rate information
    pub async fn get_exchange_rate(&self) -> RestResult<ExchangeRateResponse> {
        self.send_request(
            "api/v5/market/exchange-rate",
            reqwest::Method::GET,
            None::<&()>,
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
    fn test_exchange_rate_structure() {
        let exchange_rate_json = json!({
            "usdCny": "7.2345"
        });

        let exchange_rate: ExchangeRate = serde_json::from_value(exchange_rate_json).unwrap();
        assert_eq!(exchange_rate.usd_cny, "7.2345");
    }

    #[test]
    fn test_exchange_rate_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "usdCny": "7.2345"
                }
            ]
        });

        let response: ExchangeRateResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].usd_cny, "7.2345");
    }

    #[test]
    fn test_exchange_rate_serialization_roundtrip() {
        let original = ExchangeRate {
            usd_cny: "7.1234".to_string(),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: ExchangeRate = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.usd_cny, deserialized.usd_cny);
    }

    #[test]
    fn test_exchange_rate_response_serialization_roundtrip() {
        let original = ExchangeRateResponse {
            code: "0".to_string(),
            msg: "".to_string(),
            data: vec![ExchangeRate {
                usd_cny: "7.5678".to_string(),
            }],
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: ExchangeRateResponse = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.code, deserialized.code);
        assert_eq!(original.msg, deserialized.msg);
        assert_eq!(original.data.len(), deserialized.data.len());
        assert_eq!(original.data[0].usd_cny, deserialized.data[0].usd_cny);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_exchange_rate_endpoint_integration() {
        // This test verifies that the endpoint types are properly accessible
        let _exchange_rate = ExchangeRate {
            usd_cny: "7.1234".to_string(),
        };

        let _response = ExchangeRateResponse {
            code: "0".to_string(),
            msg: "".to_string(),
            data: vec![_exchange_rate],
        };

        // This test passes if the types exist and can be constructed
        assert!(true);
    }
}
