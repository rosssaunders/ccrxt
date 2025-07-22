use serde::{Deserialize, Serialize};

/// Request parameters for unified borrowable
#[derive(Debug, Clone, Serialize)]
pub struct UnifiedBorrowableRequest {
    /// Currency to borrow
    pub currency: String,
}

/// Unified borrowable response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedBorrowableResponse {
    /// Currency
    pub currency: String,

    /// Borrowable amount
    pub borrowable: String,
}

/// Request parameters for batch borrowable
#[derive(Debug, Clone, Serialize)]
pub struct BatchBorrowableRequest {
    /// Currencies to check
    pub currencies: Vec<String>,
}

/// Batch borrowable response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchBorrowableResponse {
    /// Currency
    pub currency: String,

    /// Borrowable amount
    pub borrowable: String,
}