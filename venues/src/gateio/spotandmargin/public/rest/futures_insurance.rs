use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures insurance
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesInsuranceRequest {
    /// Settlement currency
    pub settle: String,
    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Futures insurance balance history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesInsurance {
    /// Timestamp
    pub t: i64,

    /// Insurance balance
    pub b: String,
}

impl RestClient {
    /// Get futures insurance balance history
    ///
    /// Retrieves historical insurance fund balance for the specified settlement currency.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#futures-insurance-balance-history>
    pub async fn get_futures_insurance(
        &self,
        params: FuturesInsuranceRequest,
    ) -> crate::gateio::spotandmargin::Result<Vec<FuturesInsurance>> {
        let endpoint = format!("/futures/{}/insurance", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}
