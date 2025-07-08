//! Symbol Price Ticker V2 (GET /fapi/v2/ticker/price)
//!
//! Latest price for a symbol or symbols.
//!
//! See: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker-v2
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::binance::usdm::{Errors, RestResult};

/// Request parameters for the Symbol Price Ticker V2 endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct TickerPriceV2Request<'a> {
    /// The symbol to query (e.g., "BTCUSDT"). If not sent, returns all symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerPriceV2Response<'a> {
    pub symbol: Cow<'a, str>,
    pub price: Cow<'a, str>,
    pub time: u64,
}

// The response can be either a single object or an array. Use an enum for deserialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TickerPriceV2Result<'a> {
    Single(TickerPriceV2Response<'a>),
    Multiple(Vec<TickerPriceV2Response<'a>>),
}

impl crate::binance::usdm::public::rest::RestClient {
    /// Latest price for a symbol or symbols (GET /fapi/v2/ticker/price)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker-v2)
    pub async fn ticker_price_v2<'a>(
        &self,
        params: TickerPriceV2Request<'a>,
    ) -> RestResult<TickerPriceV2Result<'a>> {
        let endpoint = "/fapi/v2/ticker/price";
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| Errors::Error(format!("Failed to serialize params: {e}")))?;
        let resp = self
            .send_request::<TickerPriceV2Result>(
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
