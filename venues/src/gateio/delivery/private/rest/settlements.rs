use serde::{Deserialize, Serialize};

use super::RestClient;

const DELIVERY_SETTLEMENTS_ENDPOINT: &str = "/delivery/{}/settlements";

/// Delivery settlement record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverySettlement {
    /// Settlement time
    pub time: i64,

    /// Contract
    pub contract: String,

    /// Profit in settlement currency
    pub profit: String,

    /// Settlement price
    pub settle_price: String,

    /// Position size at settlement
    pub size: i64,
}

impl RestClient {
    /// Get delivery settlements
    ///
    /// This endpoint returns settlement history for delivery contracts.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `contract` - Optional contract filter
    /// * `limit` - Optional limit for number of records
    ///
    /// # Returns
    /// List of delivery settlement records
    pub async fn get_delivery_settlements(
        &self,
        settle: &str,
        contract: Option<&str>,
        limit: Option<i32>,
    ) -> crate::gateio::delivery::Result<Vec<DeliverySettlement>> {
        let mut endpoint = DELIVERY_SETTLEMENTS_ENDPOINT.replace("{}", settle);
        let mut query_params = Vec::new();

        if let Some(contract) = contract {
            query_params.push(format!("contract={}", contract));
        }
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        self.get(&endpoint).await
    }
}
