//! Request and response structs for public/get-tickers endpoint
//!
//! Fetches the public tickers for all or a particular instrument.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::cryptocom::{ApiResult, EndpointType, PublicRestClient as RestClient, RestResult};

/// Endpoint for getting tickers
const GET_TICKERS_ENDPOINT: &str = "exchange/v1/public/get-tickers";

/// Request parameters for the public/get-tickers endpoint.
///
/// Fetches the public tickers for all or a particular instrument.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTickersRequest {
    /// Instrument name. Optional.
    #[serde(rename = "instrument_name", skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<Cow<'static, str>>,
}

/// Response for public/get-tickers endpoint.
pub type GetTickersResponse = ApiResult<TickersResult>;

/// Result data for tickers.
#[derive(Debug, Clone, Deserialize)]
pub struct TickersResult {
    /// List of ticker data.
    #[serde(rename = "data")]
    pub data: Vec<Ticker>,
}

/// Ticker data for an instrument.
#[derive(Debug, Clone, Deserialize)]
pub struct Ticker {
    /// Instrument name (e.g., "BTCUSD-PERP")
    #[serde(rename = "i")]
    pub instrument_name: Option<String>,

    /// Price of the 24h highest trade
    #[serde(rename = "h")]
    pub high_price_24h: Option<String>,

    /// Price of the 24h lowest trade, null if there weren't any trades
    #[serde(rename = "l")]
    pub low_price_24h: Option<String>,

    /// The price of the latest trade, null if there weren't any trades
    #[serde(rename = "a")]
    pub last_trade_price: Option<String>,

    /// The total 24h traded volume
    #[serde(rename = "v")]
    pub volume_24h: Option<String>,

    /// The total 24h traded volume value (in USD)
    #[serde(rename = "vv")]
    pub volume_24h_value: Option<String>,

    /// Open interest
    #[serde(rename = "oi")]
    pub open_interest: Option<String>,

    /// 24-hour price change, null if there weren't any trades
    #[serde(rename = "c")]
    pub price_change_24h: Option<String>,

    /// The current best bid price, null if there aren't any bids
    #[serde(rename = "b")]
    pub best_bid_price: Option<String>,

    /// The current best ask price, null if there aren't any asks
    #[serde(rename = "k")]
    pub best_ask_price: Option<String>,

    /// The published timestamp in ms
    #[serde(rename = "t")]
    pub timestamp: Option<u64>,
}

impl RestClient {
    /// Calls the public/get-tickers endpoint.
    ///
    /// Fetches the public tickers for all or a particular instrument.
    ///
    /// [docs](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#public-get-tickers)
    pub async fn get_tickers(&self, params: GetTickersRequest) -> RestResult<GetTickersResponse> {
        self.send_get_request(
            GET_TICKERS_ENDPOINT,
            Some(&params),
            EndpointType::PublicGetTickers,
        )
        .await
    }
}
