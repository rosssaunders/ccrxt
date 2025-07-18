use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures tickers
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesTickersRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name (optional - if not provided, returns all contracts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
}

/// Futures ticker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesTicker {
    /// Contract name
    pub contract: String,

    /// Last trading price
    pub last: String,

    /// Recent lowest ask
    pub lowest_ask: String,

    /// Recent highest bid
    pub highest_bid: String,

    /// Change percentage (24h)
    pub change_percentage: String,

    /// Change amount (24h)
    pub change_utc0: Option<String>,

    /// Change amount (UTC 8)
    pub change_utc8: Option<String>,

    /// Total trading volume (24h)
    pub total_size: Option<String>,

    /// Trading volume (24h in quote currency)
    pub volume_24h: Option<String>,

    /// Trading volume (24h in base currency)
    pub volume_24h_btc: Option<String>,

    /// Trading volume (24h in quote currency)
    pub volume_24h_usd: Option<String>,

    /// Trading volume (24h in settlement currency)
    pub volume_24h_base: Option<String>,

    /// Trading volume (24h in quote currency)
    pub volume_24h_quote: Option<String>,

    /// Trading volume (24h in settlement currency, BTC denominated)
    pub volume_24h_settle: Option<String>,

    /// Mark price
    pub mark_price: Option<String>,

    /// Funding rate
    pub funding_rate: String,

    /// Predicted funding rate  
    pub funding_rate_indicative: String,

    /// Index price
    pub index_price: Option<String>,

    /// Trading enabled
    pub quanto_base_rate: Option<String>,

    /// Next funding time
    pub funding_next_apply: Option<i64>,

    /// Basis rate
    pub basis_rate: Option<String>,

    /// Basis value
    pub basis_value: Option<String>,

    /// Premium index
    pub premium_index: Option<String>,
}

impl RestClient {
    /// List futures tickers
    ///
    /// Retrieves ticker information for futures contracts.
    /// If contract is not specified, returns tickers for all contracts in the settlement currency.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-futures-tickers>
    pub async fn get_futures_tickers(
        &self,
        params: FuturesTickersRequest,
    ) -> crate::gateio::perpetual::Result<Vec<FuturesTicker>> {
        let endpoint = format!("/futures/{}/tickers", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}
