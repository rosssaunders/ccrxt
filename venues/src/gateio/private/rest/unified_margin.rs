use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for unified margin currency pairs
#[derive(Debug, Clone, Serialize, Default)]
pub struct UnifiedMarginCurrencyPairsRequest {
    /// Currency pair filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
}

/// Unified margin currency pair information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMarginCurrencyPair {
    /// Currency pair
    pub currency_pair: String,

    /// Base currency
    pub base: String,

    /// Quote currency
    pub quote: String,

    /// Leverage
    pub leverage: String,

    /// Minimum amount
    pub min_amount: String,

    /// Maximum amount
    pub max_amount: String,

    /// Price precision
    pub price_precision: i32,

    /// Amount precision
    pub amount_precision: i32,

    /// Trade status
    pub trade_status: String,

    /// Sell start timestamp
    pub sell_start: i64,

    /// Buy start timestamp
    pub buy_start: i64,
}

/// Request parameters for unified margin loans
#[derive(Debug, Clone, Serialize, Default)]
pub struct UnifiedMarginLoansRequest {
    /// Currency pair filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Page number (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records to return (1-100, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Request to create unified margin loan
#[derive(Debug, Clone, Serialize)]
pub struct CreateUnifiedMarginLoanRequest {
    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// Loan amount
    pub amount: String,
}

/// Unified margin loan information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMarginLoan {
    /// Loan ID
    pub id: String,

    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// Amount
    pub amount: String,

    /// Used amount
    pub used: String,

    /// Repaid amount
    pub repaid: String,

    /// Interest
    pub interest: String,

    /// Interest rate
    pub rate: String,

    /// Creation time
    pub create_time: i64,

    /// Update time
    pub update_time: i64,

    /// Status
    pub status: String,
}

/// Request parameters for unified margin loan records
#[derive(Debug, Clone, Serialize, Default)]
pub struct UnifiedMarginLoanRecordsRequest {
    /// Currency pair filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Page number (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records to return (1-100, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Unified margin loan record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMarginLoanRecord {
    /// Record ID
    pub id: String,

    /// Loan ID
    pub loan_id: String,

    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// Amount
    pub amount: String,

    /// Rate
    pub rate: String,

    /// Interest
    pub interest: String,

    /// Status
    pub status: String,

    /// Borrow time
    pub borrow_time: i64,

    /// Repay time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repay_time: Option<i64>,
}

/// Request parameters for unified margin borrowable
#[derive(Debug, Clone, Serialize)]
pub struct UnifiedMarginBorrowableRequest {
    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,
}

/// Unified margin borrowable information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMarginBorrowable {
    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// Borrowable amount
    pub amount: String,
}

/// Request parameters for unified margin interest records
#[derive(Debug, Clone, Serialize, Default)]
pub struct UnifiedMarginInterestRecordsRequest {
    /// Currency pair filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Page number (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records to return (1-100, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Unified margin interest record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMarginInterestRecord {
    /// Record ID
    pub id: String,

    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// Interest amount
    pub interest: String,

    /// Interest rate
    pub rate: String,

    /// Loan amount
    pub loan_amount: String,

    /// Interest time
    pub interest_time: i64,
}

impl RestClient {
    /// Get unified margin currency pairs
    ///
    /// This endpoint returns currency pairs available for unified margin trading.
    pub async fn get_unified_margin_currency_pairs(
        &self,
        params: UnifiedMarginCurrencyPairsRequest,
    ) -> crate::gateio::Result<Vec<UnifiedMarginCurrencyPair>> {
        self.get_with_query("/margin/uni/currency_pairs", &params)
            .await
    }

    /// Get a specific unified margin currency pair
    ///
    /// This endpoint returns details for a specific unified margin currency pair.
    pub async fn get_unified_margin_currency_pair(
        &self,
        currency_pair: &str,
    ) -> crate::gateio::Result<UnifiedMarginCurrencyPair> {
        let endpoint = format!("/margin/uni/currency_pairs/{}", currency_pair);
        self.get(&endpoint).await
    }

    /// Get unified margin loans
    ///
    /// This endpoint returns unified margin loans for the authenticated user.
    pub async fn get_unified_margin_loans(
        &self,
        params: UnifiedMarginLoansRequest,
    ) -> crate::gateio::Result<Vec<UnifiedMarginLoan>> {
        self.get_with_query("/margin/uni/loans", &params).await
    }

    /// Create unified margin loan
    ///
    /// This endpoint creates a new unified margin loan.
    pub async fn create_unified_margin_loan(
        &self,
        request: CreateUnifiedMarginLoanRequest,
    ) -> crate::gateio::Result<UnifiedMarginLoan> {
        self.post("/margin/uni/loans", &request).await
    }

    /// Get unified margin loan records
    ///
    /// This endpoint returns unified margin loan records.
    pub async fn get_unified_margin_loan_records(
        &self,
        params: UnifiedMarginLoanRecordsRequest,
    ) -> crate::gateio::Result<Vec<UnifiedMarginLoanRecord>> {
        self.get_with_query("/margin/uni/loan_records", &params)
            .await
    }

    /// Get unified margin borrowable amount
    ///
    /// This endpoint returns the amount that can be borrowed in unified margin.
    pub async fn get_unified_margin_borrowable(
        &self,
        params: UnifiedMarginBorrowableRequest,
    ) -> crate::gateio::Result<UnifiedMarginBorrowable> {
        self.get_with_query("/margin/uni/borrowable", &params).await
    }

    /// Get unified margin interest records
    ///
    /// This endpoint returns unified margin interest records.
    pub async fn get_unified_margin_interest_records(
        &self,
        params: UnifiedMarginInterestRecordsRequest,
    ) -> crate::gateio::Result<Vec<UnifiedMarginInterestRecord>> {
        self.get_with_query("/margin/uni/interest_records", &params)
            .await
    }
}
