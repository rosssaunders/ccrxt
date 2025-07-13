use serde::{Deserialize, Serialize};

use super::RestClient;

const DELIVERY_ACCOUNTS_ENDPOINT: &str = "/delivery/{}/accounts";

/// Request parameters for delivery accounts
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryAccountsRequest {
    /// Settlement currency (BTC, USDT, etc.)
    pub settle: String,
}

/// Delivery account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryAccount {
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

    /// Enable credit
    pub enable_credit: bool,

    /// Positions cross margin
    pub position_cross_margin: String,

    /// Orders cross margin
    pub order_cross_margin: String,

    /// Available cross margin
    pub available_cross_margin: String,

    /// Total cross margin
    pub total_cross_margin: String,
}

impl RestClient {
    /// Get delivery account information
    ///
    /// This endpoint returns delivery account balances and margin information.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery accounts request parameters
    ///
    /// # Returns
    /// Delivery account information including balances and margins
    pub async fn get_delivery_accounts(
        &self,
        params: DeliveryAccountsRequest,
    ) -> crate::gateio::delivery::Result<DeliveryAccount> {
        let endpoint = DELIVERY_ACCOUNTS_ENDPOINT.replace("{}", &params.settle);
        self.get(&endpoint).await
    }
}
