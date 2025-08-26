use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const MARGIN_LOANS_ENDPOINT: &str = "/margin/uni/loans";

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

impl RestClient {
    /// Create a loan
    ///
    /// This endpoint creates a new loan for lending or borrowing.
    pub async fn spot_create_loan(
        &self,
        request: CreateLoanRequest,
    ) -> RestResult<Loan> {
        self.post(MARGIN_LOANS_ENDPOINT, &request).await
    }
}
