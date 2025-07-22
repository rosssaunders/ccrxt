use serde::{Deserialize, Serialize};

/// Currency discount tier information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyDiscountTier {
    /// Currency
    pub currency: String,

    /// Tier level
    pub tier: i32,

    /// Discount rate
    pub discount_rate: String,

    /// Minimum amount for this tier
    pub min_amount: String,

    /// Maximum amount for this tier
    pub max_amount: String,
}

/// Loan margin tier information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanMarginTier {
    /// Currency
    pub currency: String,

    /// Tier level
    pub tier: i32,

    /// Margin rate
    pub margin_rate: String,

    /// Minimum amount
    pub min_amount: String,

    /// Maximum amount
    pub max_amount: String,
}