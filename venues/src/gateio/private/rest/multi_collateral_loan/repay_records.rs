use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const REPAY_ENDPOINT: &str = "/loan/multi_collateral/repay";

/// Repay records query
#[derive(Debug, Clone, Serialize, Default)]
pub struct RepayRecordsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// A repay record entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepayRecord {
    pub order_id: String,

    pub amount: String,

    pub interest: Option<String>,

    pub timestamp: u64,
}

impl RestClient {
    /// Get Repayment Records for Multi-Currency Collateral Loan
    ///
    /// Retrieves historical repayment records for multi-currency collateral loans
    /// with optional filtering by order ID and pagination support for large datasets.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#get-repay-records-multi-currency-collateral-loan)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `query` - Repay records query with optional order ID filter and pagination parameters
    ///
    /// # Returns
    /// List of repayment records with amounts, interest, and timestamps
    pub async fn list_multi_collateral_repay_records(
        &self,
        query: RepayRecordsQuery,
    ) -> RestResult<Vec<RepayRecord>> {
        // Some APIs use GET /repay_records, but docs differ across sections; using GET with query on REPAY_ENDPOINT for consistency
        self.send_get_request(REPAY_ENDPOINT, Some(&query)).await
    }
}
