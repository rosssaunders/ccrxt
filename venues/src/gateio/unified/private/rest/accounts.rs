//! Unified account functionality
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for getting unified account info
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetUnifiedAccountRequest {
    /// Currency to retrieve
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Unified account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAccount {
    /// User ID
    pub user_id: i64,
    /// Refresh timestamp in milliseconds
    pub refresh_time: i64,
    /// Whether account is locked
    pub locked: bool,
    /// Balances by currency
    pub balances: HashMap<String, CurrencyBalance>,
    /// Total balance in USDT
    pub total: String,
    /// Total borrowed in USDT
    pub borrowed: String,
    /// Total interest in USDT
    pub interest: String,
    /// Risk ratio
    pub risk: String,
    /// Total initial margin in USDT
    pub total_initial_margin: String,
    /// Total margin balance in USDT
    pub total_margin_balance: String,
    /// Total maintenance margin in USDT
    pub total_maintenance_margin: String,
    /// Total initial margin rate
    pub total_initial_margin_rate: String,
    /// Total maintenance margin rate
    pub total_maintenance_margin_rate: String,
    /// Total available margin in USDT
    pub total_available_margin: String,
    /// Total position initial margin in USDT
    pub total_position_initial_margin: String,
    /// Total order initial margin in USDT
    pub total_order_initial_margin: String,
}

/// Currency balance in unified account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyBalance {
    /// Available balance
    pub available: String,
    /// Freeze balance
    pub freeze: String,
    /// Borrowed balance
    pub borrowed: String,
    /// Interest
    pub interest: String,
}

/// Request to borrow or repay
#[derive(Debug, Clone, Serialize)]
pub struct BorrowOrRepayRequest {
    /// Currency
    pub currency: String,
    /// Type: borrow or repay
    #[serde(rename = "type")]
    pub operation_type: String,
    /// Amount
    pub amount: String,
}

/// Borrow/repay response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowOrRepayResponse {
    /// Currency
    pub currency: String,
    /// Type: borrow or repay
    #[serde(rename = "type")]
    pub operation_type: String,
    /// Amount
    pub amount: String,
    /// Transaction ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Request parameters for getting max borrowable amount
#[derive(Debug, Clone, Serialize)]
pub struct GetBorrowableRequest {
    /// Currency to check
    pub currency: String,
}

/// Borrowable amount response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowableResponse {
    /// Currency
    pub currency: String,
    /// Maximum borrowable amount
    pub borrowable: String,
}

/// Request parameters for getting transferable amounts
#[derive(Debug, Clone, Serialize)]
pub struct GetTransferablesRequest {
    /// Currencies to check (comma-separated)
    pub currency: String,
}

/// Transferable amount response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferableResponse {
    /// Currency
    pub currency: String,
    /// Transferable amount
    pub amount: String,
}

/// Supported loan currencies response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedCurrency {
    /// Currency name
    pub currency: String,
    /// Interest rate
    pub rate: String,
    /// Status
    pub status: String,
}

/// Implementation for the client
impl RestClient {
    /// Get unified account information
    ///
    /// This endpoint returns the unified account information.
    pub async fn get_unified_account(
        &self,
        currency: Option<&str>,
    ) -> crate::gateio::unified::Result<UnifiedAccount> {
        let request = GetUnifiedAccountRequest {
            currency: currency.map(|s| s.to_string()),
        };
        self.get_with_query("/unified/accounts", &request).await
    }
}
