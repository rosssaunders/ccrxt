use serde::{Deserialize, Serialize};

use super::RestClient;

/// Currency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    /// Currency name
    pub currency: String,

    /// Whether currency is delisted
    pub delisted: bool,

    /// Whether deposits are disabled
    pub withdraw_disabled: bool,

    /// Whether withdrawals are disabled
    pub withdraw_delayed: bool,

    /// Whether deposits are disabled
    pub deposit_disabled: bool,

    /// Whether trading is disabled
    pub trade_disabled: bool,

    /// Fixed fee rate for withdrawal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_rate: Option<String>,

    /// Chain name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
}

impl RestClient {
    /// List all currencies
    ///
    /// This endpoint returns a list of all supported currencies with their
    /// trading status, withdrawal/deposit status, and fee information.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-all-currencies-details>
    pub async fn list_currencies(&self) -> crate::gateio::spot::Result<Vec<Currency>> {
        self.get("/spot/currencies").await
    }
}
