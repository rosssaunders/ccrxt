use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for delivery risk limit tiers
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryRiskLimitTiersRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// List offset (default 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Maximum number of records to return (1-500, default 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Risk limit tier information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryRiskLimitTier {
    /// Tier level
    pub tier: i32,

    /// Maximum position size for this tier
    pub risk_limit: String,

    /// Initial margin rate
    pub initial_rate: String,

    /// Maintenance margin rate
    pub maintenance_rate: String,
}

impl RestClient {
    /// List delivery risk limit tiers
    ///
    /// Retrieves risk limit tiers for a specific delivery contract.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-risk-limit-tiers-2>
    /// Higher tiers require higher margin rates but allow larger positions.
    pub async fn get_delivery_risk_limit_tiers(
        &self,
        params: DeliveryRiskLimitTiersRequest,
    ) -> crate::gateio::Result<Vec<DeliveryRiskLimitTier>> {
        let endpoint = format!("/delivery/{}/risk_limit_tiers", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}
