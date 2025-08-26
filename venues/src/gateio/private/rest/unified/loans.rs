use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const LOANS_ENDPOINT: &str = "/unified/loans";

/// Request to borrow or repay in unified account
#[derive(Debug, Clone, Serialize)]
pub struct LoanRequest {
    /// Currency to borrow or repay
    pub currency: String,

    /// Operation type: "borrow" or "repay"
    #[serde(rename = "type")]
    pub operation_type: String,

    /// Amount to borrow or repay
    pub amount: String,

    /// Repayment type (required for repay): "all" to repay all or specific amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repaid_all: Option<bool>,
}

/// Loan operation response
#[derive(Debug, Clone, Deserialize)]
pub struct LoanResponse {
    /// Currency
    pub currency: String,

    /// Operation type: "borrow" or "repay"
    #[serde(rename = "type")]
    pub operation_type: String,

    /// Amount borrowed or repaid
    pub amount: String,

    /// Transaction ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
}

/// Request parameters for querying loans
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetLoansRequest {
    /// Currency to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Page number, starting from 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Page size, maximum 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Current loan information
#[derive(Debug, Clone, Deserialize)]
pub struct LoanInfo {
    /// Currency
    pub currency: String,

    /// Borrowed amount
    pub borrowed: String,

    /// Accrued interest
    pub interest: String,

    /// Interest rate
    pub rate: String,

    /// Last update time in seconds
    pub update_time: i64,
}

impl RestClient {
    /// Borrow or Repay
    ///
    /// Borrow or repay loans in unified account.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#borrow-or-repay)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `req` - Loan request specifying currency, type, and amount
    ///
    /// # Returns
    /// Loan operation response with transaction details
    pub async fn create_loan(&self, req: LoanRequest) -> RestResult<LoanResponse> {
        self.send_post_request(LOANS_ENDPOINT, Some(&req)).await
    }

    /// Query Loans
    ///
    /// Query current loans in unified account.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#query-loans)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Optional request parameters for filtering and pagination
    ///
    /// # Returns
    /// List of current loan information
    pub async fn get_loans(&self, req: Option<GetLoansRequest>) -> RestResult<Vec<LoanInfo>> {
        self.send_get_request(LOANS_ENDPOINT, req.as_ref()).await
    }
}
