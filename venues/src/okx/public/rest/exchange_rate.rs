use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

const MARKET_EXCHANGE_RATE_ENDPOINT: &str = "api/v5/market/exchange-rate";

/// Exchange rate information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRate {
    /// USD to CNY exchange rate
    #[serde(rename = "usdCny")]
    pub usd_cny: String,
}

impl RestClient {
    /// Get exchange rate
    ///
    /// This interface provides the average exchange rate data for 2 weeks
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#rest-api-public-rest-api-get-exchange-rate
    ///
    /// Rate limit: 1 request per 2 seconds
    ///
    /// # Returns
    /// Response containing the exchange rate information
    pub async fn get_exchange_rate(&self) -> RestResult<ExchangeRate> {
        self.send_get_request(
            MARKET_EXCHANGE_RATE_ENDPOINT,
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
    use crate::okx::response::OkxApiResponse;

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

        let response: OkxApiResponse<ExchangeRate> =
            serde_json::from_value(response_json).unwrap();
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
}
