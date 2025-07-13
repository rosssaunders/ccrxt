use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for margin accounts
#[derive(Debug, Clone, Serialize, Default)]
pub struct MarginAccountsRequest {
    /// Currency pair filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
}

/// Margin account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginAccount {
    /// Currency pair
    pub currency_pair: String,

    /// Locked status
    pub locked: bool,

    /// Risk level
    pub risk: String,

    /// Base currency information
    pub base: MarginBalance,

    /// Quote currency information
    pub quote: MarginBalance,
}

/// Margin balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginBalance {
    /// Currency
    pub currency: String,

    /// Available balance
    pub available: String,

    /// Locked balance
    pub locked: String,

    /// Borrowed amount
    pub borrowed: String,

    /// Interest amount
    pub interest: String,
}

/// Request parameters for margin account book
#[derive(Debug, Clone, Serialize, Default)]
pub struct MarginAccountBookRequest {
    /// Currency pair
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Start time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Page number (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records to return (1-100, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Margin account book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginAccountBookEntry {
    /// Record ID
    pub id: String,

    /// Timestamp
    pub time: i64,

    /// Currency
    pub currency: String,

    /// Change amount
    pub change: String,

    /// Balance after change
    pub balance: String,

    /// Change type
    #[serde(rename = "type")]
    pub change_type: String,

    /// Account type
    pub account: String,

    /// Detail information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,
}

impl RestClient {
    /// Get margin accounts
    ///
    /// This endpoint returns margin account information including balances,
    /// borrowed amounts, and risk levels for each currency pair.
    pub async fn get_margin_accounts(
        &self,
        params: MarginAccountsRequest,
    ) -> crate::gateio::spotandmargin::Result<Vec<MarginAccount>> {
        self.get_with_query("/margin/accounts", &params).await
    }

    /// Get margin account book
    ///
    /// This endpoint returns the margin account ledger showing all balance changes
    /// including trades, loans, repayments, and interest charges.
    pub async fn get_margin_account_book(
        &self,
        params: MarginAccountBookRequest,
    ) -> crate::gateio::spotandmargin::Result<Vec<MarginAccountBookEntry>> {
        self.get_with_query("/margin/account_book", &params).await
    }
}
