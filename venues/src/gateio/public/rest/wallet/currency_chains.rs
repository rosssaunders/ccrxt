use serde::Deserialize;

use super::RestClient;
use crate::gateio::RestResult;

const CURRENCY_CHAINS_ENDPOINT: &str = "/wallet/currency_chains";

#[derive(Debug, Clone, Deserialize)]
pub struct CurrencyChainInfo {
    pub chain: String,
    pub name_cn: String,
    pub name_en: String,
    pub contract_address: String,
    pub is_disabled: i32,
    pub is_deposit_disabled: i32,
    pub is_withdraw_disabled: i32,
    pub decimal: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CurrencyChainsQuery {
    pub currency: String,
}

impl RestClient {
    /// Query chains supported for specified currency (public)
    pub async fn get_currency_chains(
        &self,
        query: CurrencyChainsQuery,
    ) -> RestResult<Vec<CurrencyChainInfo>> {
        self.send_get_request(CURRENCY_CHAINS_ENDPOINT, Some(&query))
            .await
    }
}
