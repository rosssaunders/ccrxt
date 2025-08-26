use serde::Serialize;

use super::repay_loan::LoanRepayment;
use super::{RestClient, RestResult};

const MARGIN_UNI_LOAN_RECORDS_ENDPOINT: &str = "/margin/uni/loan_records";

/// Request parameters for repayment records
#[derive(Debug, Clone, Serialize, Default)]
pub struct RepaymentRecordsRequest {
    /// Loan ID
    pub loan_id: String,
}

impl RestClient {
    /// Get repayment records for a loan
    ///
    /// This endpoint returns repayment records for a specific loan.
    pub async fn spot_get_repayment_records(
        &self,
        loan_id: &str,
    ) -> RestResult<Vec<LoanRepayment>> {
        self.get_with_query(
            MARGIN_UNI_LOAN_RECORDS_ENDPOINT,
            &RepaymentRecordsRequest {
                loan_id: loan_id.to_string(),
            },
        )
        .await
    }
}
