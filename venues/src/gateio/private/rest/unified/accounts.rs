use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{RestClient, RestResult};

const UNIFIED_ACCOUNTS_ENDPOINT: &str = "/unified/accounts";

/// Request parameters for getting unified account info
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetUnifiedAccountRequest {
    /// Currency to retrieve account info for (optional filter)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Unified account information
#[derive(Debug, Clone, Deserialize)]
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
#[derive(Debug, Clone, Deserialize)]
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

impl RestClient {
    /// Get Unified Account Information
    ///
    /// Get unified account information with optional currency filter.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#get-unified-account-information)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Optional request parameters for filtering by currency
    ///
    /// # Returns
    /// Unified account information including balances and margin details
    pub async fn get_unified_accounts(
        &self,
        req: Option<GetUnifiedAccountRequest>,
    ) -> RestResult<UnifiedAccount> {
        self.send_get_request(UNIFIED_ACCOUNTS_ENDPOINT, req.as_ref())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_accounts_endpoint() {
        assert_eq!(UNIFIED_ACCOUNTS_ENDPOINT, "/unified/accounts");
    }
}
