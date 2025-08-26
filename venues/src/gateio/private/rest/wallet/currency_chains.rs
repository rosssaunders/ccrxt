use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const CURRENCY_CHAINS_ENDPOINT: &str = "/wallet/currency_chains";

/// Request parameters for querying supported chains for a currency
#[derive(Debug, Clone, Serialize)]
pub struct CurrencyChainsRequest {
    /// Currency symbol to query chains for
    pub currency: String,
}

/// Supported blockchain chain information for a currency
#[derive(Debug, Clone, Deserialize)]
pub struct CurrencyChain {
    /// Chain name (e.g., "BTC", "ETH", "BSC", "TRON")
    pub chain: String,

    /// Display name for the chain
    pub name_cn: String,

    /// English display name for the chain
    pub name_en: String,

    /// Whether deposits are enabled on this chain
    pub is_deposit_disabled: bool,

    /// Whether withdrawals are enabled on this chain
    pub is_withdraw_disabled: bool,
}

impl RestClient {
    /// Query Chains Supported for Currency
    ///
    /// Query all blockchain chains supported for depositing and withdrawing a specified currency.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#query-chains-supported-for-specified-currency)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Request with currency symbol to query supported chains for
    ///
    /// # Returns
    /// List of supported blockchain chains for the currency
    pub async fn get_currency_chains(
        &self,
        req: CurrencyChainsRequest,
    ) -> RestResult<Vec<CurrencyChain>> {
        self.send_get_request(CURRENCY_CHAINS_ENDPOINT, Some(&req))
            .await
    }
}
