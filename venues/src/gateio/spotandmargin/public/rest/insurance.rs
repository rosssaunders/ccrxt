use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for insurance fund history
#[derive(Debug, Clone, Serialize, Default)]
pub struct InsuranceHistoryRequest {
    /// Currency to query (e.g., "BTC", "USDT")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Page number, starting from 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Insurance fund history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceRecord {
    /// Timestamp
    pub t: i64,

    /// Currency
    pub currency: String,

    /// Amount
    pub amount: String,

    /// Type of record (liquidation, fee, etc.)
    #[serde(rename = "type")]
    pub record_type: String,
}

impl RestClient {
    /// Get insurance fund history
    ///
    /// This endpoint returns the history of the insurance fund, showing
    /// liquidation and fee contributions over time.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#spot-insurance-balance-history>
    pub async fn get_insurance_history(
        &self,
        params: InsuranceHistoryRequest,
    ) -> crate::gateio::spotandmargin::Result<Vec<InsuranceRecord>> {
        self.get_with_query("/spot/insurance_history", Some(&params))
            .await
    }
}
