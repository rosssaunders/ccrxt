use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Request parameters for the premium index endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct PremiumIndexRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP"). Optional.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Trading pair (e.g., "BTCUSD"). Optional.
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
}

/// Represents the premium index response for a single symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndex {
    /// Trading symbol.
    pub symbol: String,

    /// Trading pair.
    pub pair: String,

    /// Mark price.
    pub mark_price: String,

    /// Index price.
    pub index_price: String,

    /// Estimated settle price, only useful in the last hour before the settlement starts.
    pub estimated_settle_price: String,

    /// The latest funding rate, for perpetual contract symbols only. For delivery symbols, "" will be shown.
    pub last_funding_rate: String,

    /// The base asset interest rate, for perpetual contract symbols only. For delivery symbols, "" will be shown.
    pub interest_rate: String,

    /// Next funding time for perpetual contract symbols only. For delivery symbols, "" will be shown.
    pub next_funding_time: u64,

    /// Timestamp.
    pub time: u64,
}

impl RestClient {
    /// Query index price and mark price.
    ///
    /// [Official API docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Index-Price-and-Mark-Price)
    ///
    /// Weight: 10
    pub async fn get_premium_index(
        &self,
        params: PremiumIndexRequest,
    ) -> RestResult<Vec<PremiumIndex>> {
        let params_opt = if params.symbol.is_some() || params.pair.is_some() {
            Some(params)
        } else {
            None
        };

        self.send_request(
            "/dapi/v1/premiumIndex",
            reqwest::Method::GET,
            params_opt,
            10,
        )
        .await
    }
}
