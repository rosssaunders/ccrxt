//! Request and response structs for public/get-risk-parameters endpoint
//!
//! Provides information on risk parameter settings for Smart Cross Margin.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::cryptocom::{EndpointType, RestResult};
use super::client::RestClient;

/// Response for public/get-risk-parameters endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetRiskParametersResponse {
    /// Result data for risk parameters.
    #[serde(rename = "result")]
    pub result: RiskParametersResult,

    /// Success status.
    #[serde(rename = "success")]
    pub success: bool,

    /// Response ID.
    #[serde(rename = "id")]
    pub id: u64,
}

/// Result data for risk parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct RiskParametersResult {
    /// List of base currency configs.
    #[serde(rename = "data")]
    pub data: Vec<BaseCurrencyConfig>,
}

/// Base currency config for risk parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct BaseCurrencyConfig {
    /// Base currency code.
    #[serde(rename = "base_currency")]
    pub base_currency: Cow<'static, str>,

    /// Maintenance margin rate.
    #[serde(rename = "maintenance_margin_rate")]
    pub maintenance_margin_rate: f64,

    /// Initial margin rate.
    #[serde(rename = "initial_margin_rate")]
    pub initial_margin_rate: f64,
}

impl RestClient {
    /// Calls the public/get-risk-parameters endpoint.
    ///
    /// Provides information on risk parameter settings for Smart Cross Margin.
    ///
    /// [Official API docs](https://exchange-docs.crypto.com/spot/index.html#public-get-risk-parameters)
    pub async fn get_risk_parameters(
        &self,
    ) -> RestResult<GetRiskParametersResponse> {
        self.send_request(
            "public/get-risk-parameters",
            reqwest::Method::GET,
            None::<&()>,
            EndpointType::PublicGetRiskParameters,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_parameters_endpoint_type() {
        let risk_parameters_endpoint = EndpointType::PublicGetRiskParameters;
        assert!(risk_parameters_endpoint.rate_limit().max_requests > 0);
    }
}