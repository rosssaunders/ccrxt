//! Request and response structs for public/get-risk-parameters endpoint
//!
//! Provides information on risk parameter settings for Smart Cross Margin.

use serde::Deserialize;

use super::client::RestClient;
use crate::cryptocom::{ApiResult, EndpointType, RestResult};

/// Endpoint path for the get-risk-parameters API
const RISK_PARAMETERS_ENDPOINT: &str = "public/get-risk-parameters";

/// Response for public/get-risk-parameters endpoint.
pub type GetRiskParametersResponse = ApiResult<RiskParametersResult>;

/// Result data for risk parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct RiskParametersResult {
    #[serde(rename = "default_max_product_leverage_for_spot")]
    pub default_max_product_leverage_for_spot: Option<String>,
    #[serde(rename = "default_max_product_leverage_for_perps")]
    pub default_max_product_leverage_for_perps: Option<String>,
    #[serde(rename = "default_max_product_leverage_for_futures")]
    pub default_max_product_leverage_for_futures: Option<String>,
    #[serde(rename = "default_unit_margin_rate")]
    pub default_unit_margin_rate: Option<String>,
    #[serde(rename = "default_collateral_cap")]
    pub default_collateral_cap: Option<String>,
    #[serde(rename = "update_timestamp_ms")]
    pub update_timestamp_ms: Option<i64>,
    #[serde(rename = "base_currency_config")]
    pub base_currency_config: Option<Vec<BaseCurrencyConfig>>,
}

/// Base currency config for risk parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct BaseCurrencyConfig {
    #[serde(rename = "instrument_name")]
    pub instrument_name: Option<String>,
    #[serde(rename = "collateral_cap_notional")]
    pub collateral_cap_notional: Option<String>,
    #[serde(rename = "minimum_haircut")]
    pub minimum_haircut: Option<String>,
    #[serde(rename = "max_product_leverage_for_spot")]
    pub max_product_leverage_for_spot: Option<String>,
    #[serde(rename = "max_product_leverage_for_perps")]
    pub max_product_leverage_for_perps: Option<String>,
    #[serde(rename = "max_product_leverage_for_futures")]
    pub max_product_leverage_for_futures: Option<String>,
    #[serde(rename = "unit_margin_rate")]
    pub unit_margin_rate: Option<String>,
    #[serde(rename = "max_short_sell_limit")]
    pub max_short_sell_limit: Option<String>,
    #[serde(rename = "order_limit")]
    pub order_limit: Option<String>,
    #[serde(rename = "max_order_notional_usd")]
    pub max_order_notional_usd: Option<String>,
    #[serde(rename = "min_order_notional_usd")]
    pub min_order_notional_usd: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl RestClient {
    /// Calls the public/get-risk-parameters endpoint.
    ///
    /// Provides information on risk parameter settings for Smart Cross Margin.
    ///
    /// [Official API docs](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#public-get-risk-parameters)
    pub async fn get_risk_parameters(&self) -> RestResult<GetRiskParametersResponse> {
        self.send_request(
            RISK_PARAMETERS_ENDPOINT,
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
