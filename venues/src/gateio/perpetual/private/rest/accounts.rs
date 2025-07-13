use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures accounts
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesAccountsRequest {
    /// Settlement currency (BTC, USDT, etc.)
    pub settle: String,
}

/// Futures account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesAccount {
    /// Total balance
    pub total: String,

    /// Unrealized PnL
    pub unrealised_pnl: String,

    /// Position margin
    pub position_margin: String,

    /// Order margin
    pub order_margin: String,

    /// Available balance
    pub available: String,

    /// Point balance
    pub point: String,

    /// Currency
    pub currency: String,

    /// Balance in settlement currency
    pub in_dual_mode: bool,

    /// Enable credit
    pub enable_credit: bool,

    /// Position cross margin
    pub position_cross_margin: String,

    /// Order cross margin
    pub order_cross_margin: String,

    /// Available cross margin
    pub available_cross_margin: String,

    /// Total cross margin
    pub total_cross_margin: String,
}

impl RestClient {
    /// Get futures accounts
    ///
    /// This endpoint returns futures account information for the authenticated user.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.com/docs/developers/apiv4/#query-futures-account>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - The futures account request parameters
    ///
    /// # Returns
    /// Account information
    pub async fn get_futures_accounts(
        &self,
        params: FuturesAccountsRequest,
    ) -> crate::gateio::perpetual::Result<FuturesAccount> {
        let endpoint = format!("/futures/{}/accounts", params.settle);
        self.get(&endpoint).await
    }
}
