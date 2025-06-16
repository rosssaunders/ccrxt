//! Request and response structs for public/get-insurance endpoint
//!
//! Fetches balance of Insurance Fund for a particular currency.

use super::client::RestClient;
use crate::cryptocom::{EndpointType, RestResult};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for the public/get-insurance endpoint.
///
/// Fetches balance of Insurance Fund for a particular currency.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetInsuranceRequest {
    /// Currency code (e.g., "USD"). Required.
    #[serde(rename = "currency")]
    pub currency: Cow<'static, str>,
}

/// Response for public/get-insurance endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetInsuranceResponse {
    /// Result data for insurance.
    #[serde(rename = "result")]
    pub result: InsuranceResult,

    /// Success status.
    #[serde(rename = "success")]
    pub success: bool,

    /// Response ID.
    #[serde(rename = "id")]
    pub id: u64,
}

/// Result data for insurance.
#[derive(Debug, Clone, Deserialize)]
pub struct InsuranceResult {
    /// List of insurance data.
    #[serde(rename = "data")]
    pub data: Vec<Insurance>,
}

/// Insurance data for a currency.
#[derive(Debug, Clone, Deserialize)]
pub struct Insurance {
    /// Currency code.
    #[serde(rename = "currency")]
    pub currency: Cow<'static, str>,

    /// Insurance fund balance.
    #[serde(rename = "balance")]
    pub balance: f64,
}

impl RestClient {
    /// Calls the public/get-insurance endpoint.
    ///
    /// Fetches balance of Insurance Fund for a particular currency.
    ///
    /// [Official API docs](https://exchange-docs.crypto.com/spot/index.html)
    pub async fn get_insurance(&self, params: GetInsuranceRequest) -> RestResult<GetInsuranceResponse> {
        self.send_request(
            "public/get-insurance",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetInsurance,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::cryptocom::EndpointType;

    use serde_json::json;

    #[test]
    fn test_insurance_endpoint_type() {
        let insurance_endpoint = EndpointType::PublicGetInsurance;
        assert!(insurance_endpoint.rate_limit().max_requests > 0);
    }

    #[test]
    fn test_insurance_parameter_building() {
        let params = json!({
            "currency": "USD",
        });
        assert_eq!(params.get("currency").unwrap(), "USD");
    }
}
