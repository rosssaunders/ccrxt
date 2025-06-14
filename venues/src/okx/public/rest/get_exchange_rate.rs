use serde::{Deserialize, Serialize};
use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Response for getting exchange rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExchangeRateResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Exchange rate data
    pub data: Vec<ExchangeRate>,
}

/// Exchange rate information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    /// Exchange rate (USD to CNY)
    #[serde(rename = "usdCny")]
    pub usd_cny: String,
}

impl RestClient {
    /// Get exchange rate
    ///
    /// This interface provides the average exchange rate data for 2 weeks.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-market-data-get-exchange-rate
    ///
    /// Rate limit: 1 request per 2 seconds
    ///
    /// # Returns
    /// Response containing the exchange rate information
    pub async fn get_exchange_rate(&self) -> RestResult<GetExchangeRateResponse> {
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
            "usdCny": "6.45"
        });

        let exchange_rate: ExchangeRate = serde_json::from_value(exchange_rate_json).unwrap();
        assert_eq!(exchange_rate.usd_cny, "6.45");
    }

    #[test]
    fn test_get_exchange_rate_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "usdCny": "6.45"
                }
            ]
        });

        let response: GetExchangeRateResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        let exchange_rate = response.data.first().unwrap();
        assert_eq!(exchange_rate.usd_cny, "6.45");
    }
}