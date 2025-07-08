use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for listing loans
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListLoansRequest {
    /// Status filter (open, finished)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Side filter (lend, borrow)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,

    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Currency pair filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Sort direction (asc, desc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,

    /// Reverse sorting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_sort: Option<bool>,

    /// Page number (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records to return (1-100, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Request to create a loan
#[derive(Debug, Clone, Serialize)]
pub struct CreateLoanRequest {
    /// Side (lend, borrow)
    pub side: String,

    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// Loan amount
    pub amount: String,

    /// Interest rate (for lending)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,

    /// Days to lend/borrow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,

    /// Auto renew enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
}

/// Request to modify a loan
#[derive(Debug, Clone, Serialize)]
pub struct ModifyLoanRequest {
    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// New loan amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,

    /// New interest rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,

    /// Auto renew setting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
}

/// Loan information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loan {
    /// Loan ID
    pub id: String,

    /// Side (lend, borrow)
    pub side: String,

    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// Loan rate
    pub rate: String,

    /// Original amount
    pub amount: String,

    /// Days
    pub days: i32,

    /// Auto renew enabled
    pub auto_renew: bool,

    /// In use amount
    pub in_use: String,

    /// Left amount
    pub left: String,

    /// Loan status
    pub status: String,

    /// Creation time
    pub create_time: i64,

    /// Update time
    pub update_time: i64,
}

/// Request to repay a loan
#[derive(Debug, Clone, Serialize)]
pub struct RepayLoanRequest {
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

/// Request parameters for repayment records
#[derive(Debug, Clone, Serialize, Default)]
pub struct RepaymentRecordsRequest {
    /// Loan ID
    pub loan_id: String,
}

/// Request parameters for loan records
#[derive(Debug, Clone, Serialize, Default)]
pub struct LoanRecordsRequest {
    /// Loan record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_record_id: Option<String>,

    /// Status filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Page number (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records to return (1-100, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Loan record information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanRecord {
    /// Loan record ID
    pub id: String,

    /// Loan ID
    pub loan_id: String,

    /// Borrower user ID
    pub borrower_id: i64,

    /// Lender user ID
    pub lender_id: i64,

    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// Loan rate
    pub rate: String,

    /// Amount
    pub amount: String,

    /// Days
    pub days: i32,

    /// Status
    pub status: String,

    /// Repaid amount
    pub repaid: String,

    /// Paid interest
    pub paid_interest: String,

    /// Unpaid interest
    pub unpaid_interest: String,

    /// Creation time
    pub create_time: i64,

    /// Expire time
    pub expire_time: i64,
}

impl RestClient {
    /// List margin loans
    ///
    /// This endpoint returns a list of margin loans (lending or borrowing) for the authenticated user.
    pub async fn list_margin_loans(
        &self,
        params: ListLoansRequest,
    ) -> crate::gateio::Result<Vec<Loan>> {
        self.get_with_query("/margin/loans", &params).await
    }

    /// Get a specific loan
    ///
    /// This endpoint returns details for a specific loan by ID.
    pub async fn get_loan(&self, loan_id: &str) -> crate::gateio::Result<Loan> {
        let endpoint = format!("/margin/loans/{}", loan_id);
        self.get(&endpoint).await
    }

    /// Create a loan
    ///
    /// This endpoint creates a new loan for lending or borrowing.
    pub async fn create_loan(&self, request: CreateLoanRequest) -> crate::gateio::Result<Loan> {
        self.post("/margin/loans", &request).await
    }

    /// Modify a loan
    ///
    /// This endpoint modifies an existing loan's parameters.
    pub async fn modify_loan(
        &self,
        loan_id: &str,
        request: ModifyLoanRequest,
    ) -> crate::gateio::Result<Loan> {
        let endpoint = format!("/margin/loans/{}", loan_id);
        self.patch(&endpoint, &request).await
    }

    /// Cancel a loan
    ///
    /// This endpoint cancels an existing loan.
    pub async fn cancel_loan(
        &self,
        loan_id: &str,
        currency: &str,
        currency_pair: &str,
    ) -> crate::gateio::Result<Loan> {
        let endpoint = format!(
            "/margin/loans/{}?currency={}&currency_pair={}",
            loan_id, currency, currency_pair
        );
        self.delete(&endpoint).await
    }

    /// Repay a loan
    ///
    /// This endpoint creates a repayment for a loan.
    pub async fn repay_loan(
        &self,
        loan_id: &str,
        request: RepayLoanRequest,
    ) -> crate::gateio::Result<Vec<LoanRepayment>> {
        let endpoint = format!("/margin/loans/{}/repayment", loan_id);
        self.post(&endpoint, &request).await
    }

    /// Get repayment records for a loan
    ///
    /// This endpoint returns repayment records for a specific loan.
    pub async fn get_repayment_records(
        &self,
        loan_id: &str,
    ) -> crate::gateio::Result<Vec<LoanRepayment>> {
        let endpoint = format!("/margin/loans/{}/repayment", loan_id);
        self.get(&endpoint).await
    }

    /// Get loan records
    ///
    /// This endpoint returns loan records showing lending/borrowing activity.
    pub async fn get_loan_records(
        &self,
        params: LoanRecordsRequest,
    ) -> crate::gateio::Result<Vec<LoanRecord>> {
        self.get_with_query("/margin/loan_records", &params).await
    }

    /// Get a specific loan record
    ///
    /// This endpoint returns details for a specific loan record by ID.
    pub async fn get_loan_record(
        &self,
        loan_record_id: &str,
        params: LoanRecordsRequest,
    ) -> crate::gateio::Result<LoanRecord> {
        let endpoint = format!("/margin/loan_records/{}", loan_record_id);
        self.get_with_query(&endpoint, &params).await
    }

    /// Modify a loan record
    ///
    /// This endpoint modifies an existing loan record.
    pub async fn modify_loan_record(
        &self,
        loan_record_id: &str,
        auto_renew: bool,
    ) -> crate::gateio::Result<LoanRecord> {
        let endpoint = format!("/margin/loan_records/{}", loan_record_id);
        let request = serde_json::json!({
            "auto_renew": auto_renew
        });
        self.patch(&endpoint, &request).await
    }
}
