use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for getting interest rate
const ACCOUNT_INTEREST_RATE_ENDPOINT: &str = "api/v5/account/interest-rate";

/// Request parameters for getting interest rate
#[derive(Debug, Clone, Serialize)]
pub struct GetInterestRateRequest {
    /// Currency, e.g. BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Interest rate information
#[derive(Debug, Clone, Deserialize)]
pub struct InterestRate {
    /// Currency, e.g. BTC
    pub ccy: String,

    /// Interest rate
    #[serde(rename = "interestRate")]
    pub interest_rate: String,
}

impl RestClient {
    /// Get interest rate (trading account)
    ///
    /// Get the user's current leveraged currency borrowing interest rate for trading account.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-interest-rate)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The interest rate request parameters
    ///
    /// # Returns
    /// A result containing the interest rate information
    pub async fn get_trading_interest_rate(
        &self,
        request: GetInterestRateRequest,
    ) -> RestResult<InterestRate> {
        self.send_get_request(
            ACCOUNT_INTEREST_RATE_ENDPOINT,
            Some(&request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_get_interest_rate_request_serialization() {
        let request = GetInterestRateRequest {
            ccy: Some("BTC".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"ccy\":\"BTC\""));
    }

    #[test]
    fn test_get_interest_rate_request_empty() {
        let request = GetInterestRateRequest { ccy: None };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_interest_rate_deserialization() {
        let rate_json = json!({
            "ccy": "BTC",
            "interestRate": "0.00098"
        });

        let rate: InterestRate = serde_json::from_value(rate_json).unwrap();
        assert_eq!(rate.ccy, "BTC");
        assert_eq!(rate.interest_rate, "0.00098");
    }

    #[test]
    fn test_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "BTC",
                    "interestRate": "0.00098"
                },
                {
                    "ccy": "ETH",
                    "interestRate": "0.00087"
                }
            ]
        });

        let response: ApiResponse<InterestRate> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].ccy, "BTC");
        assert_eq!(response.data[1].ccy, "ETH");
    }
}
