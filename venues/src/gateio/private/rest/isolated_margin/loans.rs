use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const MARGIN_UNI_LOANS_ENDPOINT: &str = "/margin/uni/loans";

/// Request to borrow or repay
#[derive(Debug, Clone, Serialize)]
pub struct BorrowOrRepayRequest {
    /// Currency to borrow/repay
    pub currency: String,

    /// Amount to borrow/repay
    pub amount: String,

    /// Operation type: "borrow" or "repay"
    #[serde(rename = "type")]
    pub operation_type: String,

    /// Trading pair
    pub currency_pair: String,

    /// Whether to repay all borrowed amount (for repay operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repaid_all: Option<bool>,
}

/// Request parameters for querying loans
#[derive(Debug, Clone, Serialize, Default)]
pub struct QueryLoansRequest {
    /// Trading pair filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Records per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Borrow or repay operation result
#[derive(Debug, Clone, Deserialize)]
pub struct BorrowOrRepayResponse {
    /// Currency
    pub currency: String,

    /// Amount borrowed/repaid
    pub amount: String,

    /// Current borrowed amount after operation
    pub borrowed: String,

    /// Current available balance after operation
    pub available: String,
}

/// Loan information
#[derive(Debug, Clone, Deserialize)]
pub struct Loan {
    /// Loan ID
    pub id: String,

    /// Currency
    pub currency: String,

    /// Trading pair
    pub currency_pair: String,

    /// Borrowed amount
    pub amount: String,

    /// Outstanding borrowed amount
    pub left: String,

    /// Interest rate
    pub rate: String,

    /// Auto renew status
    pub auto_renew: bool,

    /// Creation time
    pub create_time: i64,

    /// Update time
    pub update_time: i64,
}

impl RestClient {
    /// Borrow or Repay
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#borrow-or-repay)
    pub async fn borrow_or_repay_margin(&self, req: BorrowOrRepayRequest) -> RestResult<BorrowOrRepayResponse> {
        self.send_post_request(MARGIN_UNI_LOANS_ENDPOINT, Some(&req)).await
    }

    /// Query Loans
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#query-loans)
    pub async fn get_margin_loans(&self, req: Option<QueryLoansRequest>) -> RestResult<Vec<Loan>> {
        self.send_get_request(MARGIN_UNI_LOANS_ENDPOINT, req.as_ref()).await
    }
}