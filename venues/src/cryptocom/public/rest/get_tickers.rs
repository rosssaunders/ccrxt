//! Request and response structs for public/get-tickers endpoint
//!
//! Fetches the public tickers for all or a particular instrument.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::cryptocom::{EndpointType, RestResult};
use super::client::RestClient;

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
#[derive(Debug, Clone, Deserialize)]
pub struct GetTickersResponse {
    /// Result data for tickers.
    #[serde(rename = "result")]
    pub result: TickersResult,

    /// Success status.
    #[serde(rename = "success")]
    pub success: bool,

    /// Response ID.
    #[serde(rename = "id")]
    pub id: u64,
}

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
    /// Instrument name.
    #[serde(rename = "instrument_name")]
    pub instrument_name: Cow<'static, str>,

    /// Last traded price.
    #[serde(rename = "last_trade_price")]
    pub last_trade_price: f64,

    /// 24h high price.
    #[serde(rename = "high_price_24h")]
    pub high_price_24h: f64,

    /// 24h low price.
    #[serde(rename = "low_price_24h")]
    pub low_price_24h: f64,

    /// 24h volume.
    #[serde(rename = "volume_24h")]
    pub volume_24h: f64,
}

impl RestClient {
    /// Calls the public/get-tickers endpoint.
    ///
    /// Fetches the public tickers for all or a particular instrument.
    ///
    /// [Official API docs](https://exchange-docs.crypto.com/spot/index.html#public-get-tickers)
    pub async fn get_tickers(
        &self,
        params: GetTickersRequest,
    ) -> RestResult<GetTickersResponse> {
        self.send_request(
            "public/get-tickers",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetTickers,
        )
        .await
    }
}