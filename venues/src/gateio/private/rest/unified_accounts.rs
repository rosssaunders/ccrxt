use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

impl CurrencyBalance {
    /// Calculate total balance (available + freeze)
    pub fn total_balance(&self) -> Result<f64, std::num::ParseFloatError> {
        let available: f64 = self.available.parse()?;
        let freeze: f64 = self.freeze.parse()?;
        Ok(available + freeze)
    }

    /// Check if this currency has any balance
    pub fn has_balance(&self) -> bool {
        let available: f64 = self.available.parse().unwrap_or(0.0);
        let freeze: f64 = self.freeze.parse().unwrap_or(0.0);
        let borrowed: f64 = self.borrowed.parse().unwrap_or(0.0);
        available > 0.0 || freeze > 0.0 || borrowed > 0.0
    }

    /// Check if this currency has debt
    pub fn has_debt(&self) -> bool {
        let borrowed: f64 = self.borrowed.parse().unwrap_or(0.0);
        borrowed > 0.0
    }
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

impl BorrowOrRepayRequest {
    /// Create a borrow request
    pub fn borrow(currency: String, amount: String) -> Self {
        Self {
            currency,
            operation_type: "borrow".to_string(),
            amount,
        }
    }

    /// Create a repay request
    pub fn repay(currency: String, amount: String) -> Self {
        Self {
            currency,
            operation_type: "repay".to_string(),
            amount,
        }
    }
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
    ) -> crate::gateio::Result<UnifiedAccount> {
        let request = GetUnifiedAccountRequest {
            currency: currency.map(|s| s.to_string()),
        };
        self.get_with_query("/unified/accounts", &request).await
    }
}
