//! Mark Price and Funding Rate (GET /fapi/v1/premiumIndex)
//!
//! See: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::{Errors, RestResult};

/// Request parameters for the Mark Price and Funding Rate endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct PremiumIndexRequest<'a> {
    /// The symbol to query (e.g., "BTCUSDT"). If not sent, returns all symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PremiumIndexResponse<'a> {
    pub symbol: Cow<'a, str>,
    #[serde(rename = "markPrice")]
    pub mark_price: Cow<'a, str>,
    #[serde(rename = "indexPrice")]
    pub index_price: Cow<'a, str>,
    #[serde(rename = "estimatedSettlePrice")]
    pub estimated_settle_price: Cow<'a, str>,
    #[serde(rename = "lastFundingRate")]
    pub last_funding_rate: Cow<'a, str>,
    #[serde(rename = "interestRate")]
    pub interest_rate: Cow<'a, str>,
    #[serde(rename = "nextFundingTime")]
    pub next_funding_time: u64,
    pub time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PremiumIndexResult<'a> {
    Multiple(Vec<PremiumIndexResponse<'a>>),
    Single(PremiumIndexResponse<'a>),
}

impl RestClient {
    /// Mark Price and Funding Rate (GET /fapi/v1/premiumIndex)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price)
    pub async fn premium_index<'a>(
        &self,
        params: PremiumIndexRequest<'a>,
    ) -> RestResult<PremiumIndexResult<'a>> {
        let endpoint = "/fapi/v1/premiumIndex";
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| Errors::Error(format!("Failed to serialize params: {e}")))?;
        let resp = self
            .send_request::<PremiumIndexResult>(
                endpoint,
                reqwest::Method::GET,
                Some(&query),
                None,
                1,
            )
            .await?;
        Ok(resp)
    }
}
