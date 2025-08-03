use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

const ACCOUNT_INTEREST_RATE_ENDPOINT: &str = "api/v5/account/interest-rate";
/// Request to get interest rate
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestRateRequest {
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Interest rate details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestRate {
    /// Currency
    pub ccy: String,

    /// Interest rate
    pub interest_rate: String,
}

impl RestClient {
    /// Get interest rate
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-interest-rate
    ///
    /// # Arguments
    /// * `request` - The get interest rate request
    ///
    /// # Returns
    /// A result containing the interest rate or an error
    pub async fn get_interest_rate(
        &self,
        request: &GetInterestRateRequest,
    ) -> RestResult<OkxApiResponse<InterestRate>> {
        self.send_request(
            ACCOUNT_INTEREST_RATE_ENDPOINT,
            reqwest::Method::GET,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_interest_rate_request_serialization() {
        let request = GetInterestRateRequest {
            ccy: Some("USDT".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("ccy=USDT"));
    }

    #[test]
    fn test_get_interest_rate_all_currencies() {
        let request = GetInterestRateRequest { ccy: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_interest_rate_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "USDT",
                    "interestRate": "0.0001"
                }
            ]
        }"#;

        let response: OkxApiResponse<InterestRate> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let rate = &response.data[0];
        assert_eq!(rate.ccy, "USDT");
        assert_eq!(rate.interest_rate, "0.0001");
    }
}
