use super::RestClient;
use crate::gateio::delivery::models::{
    DeliveryPositionMarginResponse, UpdateDeliveryPositionMarginRequest,
};

const DELIVERY_POSITION_MARGIN_ENDPOINT: &str = "/delivery/{}/positions/{}/margin";

impl RestClient {
    /// Update delivery position margin
    ///
    /// Adjusts the margin for a specific delivery position.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The margin update request parameters
    ///
    /// # Returns
    /// Updated position margin information
    pub async fn update_delivery_position_margin(
        &self,
        request: UpdateDeliveryPositionMarginRequest,
    ) -> crate::gateio::delivery::Result<DeliveryPositionMarginResponse> {
        let endpoint = DELIVERY_POSITION_MARGIN_ENDPOINT
            .replace("{}", &request.settle)
            .replace("{}", &request.contract);
        self.post(&endpoint, &request).await
    }
}
