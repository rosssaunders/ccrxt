use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const LOAN_RECORDS_ENDPOINT: &str = "/unified/loan_records";

/// Request parameters for querying loan records
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetLoanRecordsRequest {
    /// Loan type filter: "borrow" or "repay"
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub loan_type: Option<String>,

    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Page number, starting from 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Page size, maximum 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Loan record information
#[derive(Debug, Clone, Deserialize)]
pub struct LoanRecord {
    /// Record ID
    pub id: String,

    /// Currency
    pub currency: String,

    /// Loan type: "borrow" or "repay"
    #[serde(rename = "type")]
    pub loan_type: String,

    /// Amount borrowed or repaid
    pub amount: String,

    /// Creation time in seconds
    pub create_time: i64,
}

impl RestClient {
    /// Query Loan Records
    ///
    /// Query historical loan records (borrow/repay) in unified account.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#query-loan-records)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Optional request parameters for filtering and pagination
    ///
    /// # Returns
    /// List of historical loan records
    pub async fn get_loan_records(
        &self,
        req: Option<GetLoanRecordsRequest>,
    ) -> RestResult<Vec<LoanRecord>> {
        self.send_get_request(LOAN_RECORDS_ENDPOINT, req.as_ref())
            .await
    }
}
