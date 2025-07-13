use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request to borrow or repay
#[derive(Debug, Clone, Serialize)]
pub struct BorrowOrRepayRequest {
    /// Currency
    pub currency: String,
    /// Amount
    pub amount: String,
    /// Type of operation: borrow or repay
    #[serde(rename = "type")]
    pub type_: String,
}

/// Borrow/repay response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowOrRepayResponse {
    /// Whether the operation was successful
    pub succeed: bool,
}

/// Request to list loan records
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListLoansRequest {
    /// Currency to query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Number of records per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Loan record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanRecord {
    /// Loan record ID
    pub id: String,
    /// Creation timestamp
    pub create_time: i64,
    /// Update timestamp
    pub update_time: i64,
    /// Currency
    pub currency: String,
    /// Loan amount
    pub amount: String,
    /// Outstanding interest
    pub interest: String,
}

/// Request to get max borrowable amount
#[derive(Debug, Clone, Serialize)]
pub struct GetMaxBorrowableRequest {
    /// Currency to query
    pub currency: String,
}

/// Max borrowable amount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxBorrowable {
    /// Currency
    pub currency: String,
    /// Max borrowable amount
    pub amount: String,
}

/// Request to list loan interest records
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListLoanInterestRecordsRequest {
    /// Currency to query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Number of records per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Loan interest record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanInterestRecord {
    /// Currency
    pub currency: String,
    /// Interest amount
    pub interest: String,
    /// Timestamp of the interest record
    pub create_time: i64,
}

impl RestClient {
    /// Borrow or repay
    ///
    /// This endpoint allows borrowing or repaying funds.
    pub async fn borrow_or_repay(
        &self,
        request: BorrowOrRepayRequest,
    ) -> crate::gateio::unified::Result<BorrowOrRepayResponse> {
        self.post("/unified/borrow_or_repay", &request).await
    }

    /// Borrow funds
    pub async fn borrow(
        &self,
        currency: &str,
        amount: &str,
    ) -> crate::gateio::unified::Result<BorrowOrRepayResponse> {
        let request = BorrowOrRepayRequest {
            currency: currency.to_string(),
            amount: amount.to_string(),
            type_: "borrow".to_string(),
        };
        self.borrow_or_repay(request).await
    }

    /// Repay funds
    pub async fn repay(
        &self,
        currency: &str,
        amount: &str,
    ) -> crate::gateio::unified::Result<BorrowOrRepayResponse> {
        let request = BorrowOrRepayRequest {
            currency: currency.to_string(),
            amount: amount.to_string(),
            type_: "repay".to_string(),
        };
        self.borrow_or_repay(request).await
    }

    /// Repay all borrowed funds for a currency
    pub async fn repay_all(
        &self,
        currency: &str,
    ) -> crate::gateio::unified::Result<BorrowOrRepayResponse> {
        let unified_account = self.get_unified_account(None).await?;
        let borrowed_amount = &unified_account.borrowed;

        if borrowed_amount.parse::<f64>().unwrap_or(0.0) <= 0.0 {
            return Ok(BorrowOrRepayResponse { succeed: true });
        }

        self.repay(currency, borrowed_amount).await
    }

    /// List loan records
    ///
    /// This endpoint returns the borrowing history.
    pub async fn list_loans(
        &self,
        request: ListLoansRequest,
    ) -> crate::gateio::unified::Result<Vec<LoanRecord>> {
        self.get_with_query("/unified/loans", &request).await
    }

    /// Get loan history for a specific currency
    pub async fn get_loan_history(
        &self,
        currency: &str,
        limit: Option<u32>,
    ) -> crate::gateio::unified::Result<Vec<LoanRecord>> {
        let request = ListLoansRequest {
            currency: Some(currency.to_string()),
            limit,
            ..Default::default()
        };
        self.list_loans(request).await
    }

    /// Get all loan history
    pub async fn get_all_loan_history(
        &self,
        limit: Option<u32>,
    ) -> crate::gateio::unified::Result<Vec<LoanRecord>> {
        let request = ListLoansRequest {
            currency: None,
            limit,
            ..Default::default()
        };
        self.list_loans(request).await
    }

    /// Get max borrowable amount
    ///
    /// This endpoint returns the maximum borrowable amount for a specific currency.
    pub async fn get_max_borrowable(
        &self,
        currency: &str,
    ) -> crate::gateio::unified::Result<MaxBorrowable> {
        let request = GetMaxBorrowableRequest {
            currency: currency.to_string(),
        };
        self.get_with_query("/unified/borrowable", &request).await
    }

    /// List loan interest records
    ///
    /// This endpoint returns the interest records for loans.
    pub async fn list_loan_interest_records(
        &self,
        request: ListLoanInterestRecordsRequest,
    ) -> crate::gateio::unified::Result<Vec<LoanInterestRecord>> {
        self.get_with_query("/unified/interest_record", &request)
            .await
    }
}
