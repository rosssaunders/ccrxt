use serde::{Deserialize, Serialize};

use super::RestClient;

const DELIVERY_INSURANCE_ENDPOINT: &str = "/delivery/{}/insurance";

/// Request parameters for delivery insurance
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryInsuranceRequest {
    /// Settlement currency
    pub settle: String,
    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Delivery insurance balance history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryInsurance {
    /// Timestamp
    pub t: i64,

    /// Insurance balance
    pub b: f64,
}

impl RestClient {
    /// Get delivery insurance balance history
    ///
    /// Retrieves historical insurance fund balance for the specified settlement currency.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#futures-insurance-balance-history-2>
    pub async fn get_delivery_insurance(
        &self,
        params: DeliveryInsuranceRequest,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryInsurance>> {
        let endpoint = DELIVERY_INSURANCE_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}
