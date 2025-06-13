use super::client::RestClient;
use crate::crypto_com::{EndpointType, RestResult};
use serde_json::Value;

impl RestClient {
    /// Get risk parameter settings for Smart Cross Margin
    ///
    /// Provides information on risk parameter settings for Smart Cross Margin.
    pub async fn get_risk_parameters(&self) -> RestResult<Value> {
        self.send_request(
            "public/get-risk-parameters",
            reqwest::Method::GET,
            None,
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
