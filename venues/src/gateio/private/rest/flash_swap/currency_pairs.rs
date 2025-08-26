use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const CURRENCY_PAIRS_ENDPOINT: &str = "/flash_swap/currency_pairs";

/// List supported currency pairs for flash swap
#[derive(Debug, Clone, Serialize)]
pub struct CurrencyPairsRequest {
    /// Filter by specific currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Page number, defaults to 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of items per page, default 1000, max 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Flash swap currency pair information
#[derive(Debug, Clone, Deserialize)]
pub struct CurrencyPair {
    /// Currency pair identifier
    pub currency_pair: String,

    /// Minimum sell amount
    pub sell_min: String,

    /// Maximum sell amount  
    pub sell_max: String,

    /// Minimum buy amount
    pub buy_min: String,

    /// Maximum buy amount
    pub buy_max: String,
}

impl RestClient {
    /// List supported currency pairs for flash swap
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#list-currency-pairs-supported-by-flash-swap)
    pub async fn get_flash_swap_currency_pairs(
        &self,
        req: Option<CurrencyPairsRequest>,
    ) -> RestResult<Vec<CurrencyPair>> {
        self.send_get_request(CURRENCY_PAIRS_ENDPOINT, req.as_ref())
            .await
    }
}
