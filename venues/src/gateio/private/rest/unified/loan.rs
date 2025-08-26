use serde::{Deserialize, Serialize};

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
