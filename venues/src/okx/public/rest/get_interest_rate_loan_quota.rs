use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};


const PUBLIC_INTEREST_RATE_LOAN_QUOTA_ENDPOINT: &str = "api/v5/public/interest-rate-loan-quota";
/// Request parameters for getting interest rate and loan quota
/// This endpoint does not require any parameters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetInterestRateLoanQuotaRequest {
    // This endpoint appears to not require any parameters based on the documentation
}

/// Basic interest rate information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicInterestRate {
    /// Currency
    pub ccy: String,
    /// Daily rate
    pub rate: String,
    /// Max borrow
    pub quota: String,
}

/// VIP interest rate information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VipInterestRate {
    /// VIP Level (e.g., "VIP1")
    pub level: String,
    /// Loan quota coefficient. Loan quota = quota * level
    #[serde(rename = "loanQuotaCoef")]
    pub loan_quota_coef: String,
    /// Interest rate discount (deprecated)
    #[serde(rename = "irDiscount")]
    pub ir_discount: String,
}

/// Regular user interest rate information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularInterestRate {
    /// Regular user Level (e.g., "Lv1")
    pub level: String,
    /// Loan quota coefficient. Loan quota = quota * level
    #[serde(rename = "loanQuotaCoef")]
    pub loan_quota_coef: String,
    /// Interest rate discount (deprecated)
    #[serde(rename = "irDiscount")]
    pub ir_discount: String,
}

/// Interest rate and loan quota data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestRateLoanQuotaData {
    /// Basic interest rate
    pub basic: Vec<BasicInterestRate>,
    /// Interest info for VIP users
    pub vip: Vec<VipInterestRate>,
    /// Interest info for regular users
    pub regular: Vec<RegularInterestRate>,
}

/// Response for getting interest rate and loan quota
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInterestRateLoanQuotaResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Interest rate and loan quota data
    pub data: Vec<InterestRateLoanQuotaData>,
}

impl RestClient {
    /// Get interest rate and loan quota
    ///
    /// Retrieve interest rate and loan quota information for basic, VIP, and regular users.
    ///
    /// See: https://www.okx.com/docs-v5/en/#public-data-rest-api-get-interest-rate-and-loan-quota
    ///
    /// Rate limit: 2 requests per 2 seconds
    ///
    /// # Returns
    /// Response containing interest rate and loan quota information
    pub async fn get_interest_rate_loan_quota(
        &self,
    ) -> RestResult<GetInterestRateLoanQuotaResponse> {
        self.send_request(
            PUBLIC_INTEREST_RATE_LOAN_QUOTA_ENDPOINT,
            reqwest::Method::GET,
            None::<&GetInterestRateLoanQuotaRequest>,
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_get_interest_rate_loan_quota_request_structure() {
        let request = GetInterestRateLoanQuotaRequest::default();
        let serialized = serde_json::to_value(&request).unwrap();

        // Since this endpoint doesn't require parameters, the serialized value should be an empty object
        assert_eq!(serialized, json!({}));
    }

    #[test]
    fn test_basic_interest_rate_structure() {
        let basic_rate_json = json!({
            "ccy": "BTC",
            "rate": "0.0001",
            "quota": "100000"
        });

        let basic_rate: BasicInterestRate = serde_json::from_value(basic_rate_json).unwrap();
        assert_eq!(basic_rate.ccy, "BTC");
        assert_eq!(basic_rate.rate, "0.0001");
        assert_eq!(basic_rate.quota, "100000");
    }

    #[test]
    fn test_vip_interest_rate_structure() {
        let vip_rate_json = json!({
            "level": "VIP1",
            "loanQuotaCoef": "1.2",
            "irDiscount": "0.9"
        });

        let vip_rate: VipInterestRate = serde_json::from_value(vip_rate_json).unwrap();
        assert_eq!(vip_rate.level, "VIP1");
        assert_eq!(vip_rate.loan_quota_coef, "1.2");
        assert_eq!(vip_rate.ir_discount, "0.9");
    }

    #[test]
    fn test_regular_interest_rate_structure() {
        let regular_rate_json = json!({
            "level": "Lv1",
            "loanQuotaCoef": "1.0",
            "irDiscount": "1.0"
        });

        let regular_rate: RegularInterestRate = serde_json::from_value(regular_rate_json).unwrap();
        assert_eq!(regular_rate.level, "Lv1");
        assert_eq!(regular_rate.loan_quota_coef, "1.0");
        assert_eq!(regular_rate.ir_discount, "1.0");
    }

    #[test]
    fn test_interest_rate_loan_quota_data_structure() {
        let data_json = json!({
            "basic": [
                {
                    "ccy": "BTC",
                    "rate": "0.0001",
                    "quota": "100000"
                },
                {
                    "ccy": "ETH",
                    "rate": "0.0002",
                    "quota": "200000"
                }
            ],
            "vip": [
                {
                    "level": "VIP1",
                    "loanQuotaCoef": "1.2",
                    "irDiscount": "0.9"
                }
            ],
            "regular": [
                {
                    "level": "Lv1",
                    "loanQuotaCoef": "1.0",
                    "irDiscount": "1.0"
                }
            ]
        });

        let data: InterestRateLoanQuotaData = serde_json::from_value(data_json).unwrap();
        assert_eq!(data.basic.len(), 2);
        assert_eq!(data.vip.len(), 1);
        assert_eq!(data.regular.len(), 1);

        assert_eq!(data.basic[0].ccy, "BTC");
        assert_eq!(data.basic[1].ccy, "ETH");
        assert_eq!(data.vip[0].level, "VIP1");
        assert_eq!(data.regular[0].level, "Lv1");
    }

    #[test]
    fn test_get_interest_rate_loan_quota_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "basic": [
                        {
                            "ccy": "BTC",
                            "rate": "0.0001",
                            "quota": "100000"
                        }
                    ],
                    "vip": [
                        {
                            "level": "VIP1",
                            "loanQuotaCoef": "1.2",
                            "irDiscount": "0.9"
                        }
                    ],
                    "regular": [
                        {
                            "level": "Lv1",
                            "loanQuotaCoef": "1.0",
                            "irDiscount": "1.0"
                        }
                    ]
                }
            ]
        });

        let response: GetInterestRateLoanQuotaResponse =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 1);

        let data = &response.data[0];
        assert_eq!(data.basic.len(), 1);
        assert_eq!(data.vip.len(), 1);
        assert_eq!(data.regular.len(), 1);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let original_data = InterestRateLoanQuotaData {
            basic: vec![BasicInterestRate {
                ccy: "BTC".to_string(),
                rate: "0.0001".to_string(),
                quota: "100000".to_string(),
            }],
            vip: vec![VipInterestRate {
                level: "VIP1".to_string(),
                loan_quota_coef: "1.2".to_string(),
                ir_discount: "0.9".to_string(),
            }],
            regular: vec![RegularInterestRate {
                level: "Lv1".to_string(),
                loan_quota_coef: "1.0".to_string(),
                ir_discount: "1.0".to_string(),
            }],
        };

        let serialized = serde_json::to_value(&original_data).unwrap();
        let deserialized: InterestRateLoanQuotaData = serde_json::from_value(serialized).unwrap();

        assert_eq!(original_data.basic[0].ccy, deserialized.basic[0].ccy);
        assert_eq!(original_data.vip[0].level, deserialized.vip[0].level);
        assert_eq!(
            original_data.regular[0].level,
            deserialized.regular[0].level
        );
    }

    #[test]
    fn test_rest_client_method_integration() {
        use crate::okx::rate_limit::RateLimiter;

        // Test that the method can be called on RestClient
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();
        let rest_client = RestClient::new("https://www.okx.com", client, rate_limiter);

        // Verify the method exists and has the correct signature
        // This is a compile-time test - if this compiles, the integration is correct
        let _future = rest_client.get_interest_rate_loan_quota();
    }
}
