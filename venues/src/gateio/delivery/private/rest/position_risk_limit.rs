use super::RestClient;
use crate::gateio::delivery::models::{DeliveryRiskLimitResponse, UpdateDeliveryRiskLimitRequest};

const DELIVERY_POSITION_RISK_LIMIT_ENDPOINT: &str = "/delivery/{}/positions/{}/risk_limit";

impl RestClient {
    /// Update delivery position risk limit
    ///
    /// Changes the risk limit for a specific delivery position.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The risk limit update request parameters
    ///
    /// # Returns
    /// Updated risk limit information
    pub async fn update_delivery_position_risk_limit(
        &self,
        request: UpdateDeliveryRiskLimitRequest,
    ) -> crate::gateio::delivery::Result<DeliveryRiskLimitResponse> {
        let endpoint = DELIVERY_POSITION_RISK_LIMIT_ENDPOINT
            .replace("{}", &request.settle)
            .replace("{}", &request.contract);
        self.post(&endpoint, &request).await
    }
}
