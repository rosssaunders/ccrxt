use super::RestClient;
use crate::gateio::delivery::models::{DeliveryLeverageResponse, SetDeliveryLeverageRequest};

const DELIVERY_POSITION_LEVERAGE_ENDPOINT: &str = "/delivery/{}/positions/{}/leverage";

impl RestClient {
    /// Set delivery position leverage
    ///
    /// This endpoint sets the leverage for a specific delivery contract position.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The leverage setting request parameters
    ///
    /// # Returns
    /// Updated leverage information
    pub async fn set_delivery_position_leverage(
        &self,
        request: SetDeliveryLeverageRequest,
    ) -> crate::gateio::delivery::Result<DeliveryLeverageResponse> {
        let endpoint = DELIVERY_POSITION_LEVERAGE_ENDPOINT
            .replace("{}", &request.settle)
            .replace("{}", &request.contract);
        self.post(&endpoint, &request).await
    }
}
