use super::RestClient;
use crate::gateio::delivery::models::{DeliveryPosition, DeliveryPositionsRequest};

const DELIVERY_POSITIONS_ENDPOINT: &str = "/delivery/{}/positions";
const DELIVERY_POSITION_ENDPOINT: &str = "/delivery/{}/positions/{}";

impl RestClient {
    /// Get delivery positions
    ///
    /// This endpoint returns all delivery positions for the authenticated user.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery positions request parameters
    ///
    /// # Returns
    /// List of delivery positions
    pub async fn get_delivery_positions(
        &self,
        params: DeliveryPositionsRequest,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryPosition>> {
        let endpoint = DELIVERY_POSITIONS_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Get a specific delivery position
    ///
    /// This endpoint returns details for a specific delivery position.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `contract` - Contract name
    ///
    /// # Returns
    /// Specific delivery position details
    pub async fn get_delivery_position(
        &self,
        settle: &str,
        contract: &str,
    ) -> crate::gateio::delivery::Result<DeliveryPosition> {
        let endpoint = DELIVERY_POSITION_ENDPOINT
            .replace("{}", settle)
            .replace("{}", contract);
        self.get(&endpoint).await
    }
}
