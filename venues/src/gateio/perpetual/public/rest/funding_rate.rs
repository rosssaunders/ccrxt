use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures funding rate
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesFundingRateRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Futures funding rate history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesFundingRate {
    /// Funding time
    pub t: i64,

    /// Funding rate
    pub r: String,
}

impl RestClient {
    /// Get futures funding rate history
    ///
    /// Retrieves historical funding rates for a specific futures contract.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#funding-rate-history>
    pub async fn get_futures_funding_rate(
        &self,
        params: FuturesFundingRateRequest,
    ) -> crate::gateio::perpetual::Result<Vec<FuturesFundingRate>> {
        let endpoint = format!("/futures/{}/funding_rate", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}
