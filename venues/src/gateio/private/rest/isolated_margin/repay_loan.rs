use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const MARGIN_UNI_LOANS_ENDPOINT: &str = "/margin/uni/loans";

/// Request to repay a loan
#[derive(Debug, Clone, Serialize)]
pub struct RepayLoanRequest {
    /// Loan ID to repay
    pub loan_id: String,
    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// Repayment mode (all, partial)
    pub mode: String,

    /// Amount to repay (for partial repayment)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

/// Loan repayment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanRepayment {
    /// Loan ID
    pub loan_id: String,

    /// Repayment ID
    pub repay_id: String,

    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// Principal amount repaid
    pub principal: String,

    /// Interest amount repaid
    pub interest: String,

    /// Repayment time
    pub repay_time: i64,
}

impl RestClient {
    /// Repay a loan
    ///
    /// This endpoint creates a repayment for a loan.
    pub async fn spot_repay_loan(
        &self,
        request: RepayLoanRequest,
    ) -> RestResult<Vec<LoanRepayment>> {
        // New unified endpoint for repayments
        // Some unified APIs accept body with loan_id or expect a unified route; if path is required, adapt accordingly.
        self.post(MARGIN_UNI_LOANS_ENDPOINT, &request).await
    }
}
