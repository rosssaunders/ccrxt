use serde::{Deserialize, Serialize};

use super::RestClient;

/// Options account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsAccount {
    /// Total balance
    pub total: String,

    /// Unrealized PnL
    pub unrealised_pnl: String,

    /// Initial margin
    pub init_margin: String,

    /// Maintenance margin
    pub maint_margin: String,

    /// Option value
    pub option_value: String,

    /// Available balance
    pub available: String,

    /// Point balance
    pub point: String,

    /// Currency
    pub currency: String,

    /// Portfolio margin requirement
    pub portfolio_margin: String,
}

impl RestClient {
    /// Get options account information
    ///
    /// This endpoint returns options account balances and margin information.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Returns
    /// Options account information including balances and margins
    pub async fn get_options_accounts(&self) -> crate::gateio::options::Result<OptionsAccount> {
        self.get("/options/accounts").await
    }
}
